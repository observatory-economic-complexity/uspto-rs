use quick_xml::{self, Reader};
use quick_xml::events::{Event, BytesText};
use snafu::OptionExt;
use std::io::BufRead;

use crate::data::*;
use crate::error::Error;
use crate::error::Deser;
// helper macros
use crate::{try_some, parse_struct_update, parse_struct_update_from};
use crate::util::{consume_start, consume_any_end};

pub struct PatentGrants<B: BufRead> {
    rdr: quick_xml::Reader<B>,
    buf: Vec<u8>,
}

impl<B: BufRead> PatentGrants<B> {
    pub fn from_reader(b: B) -> Self {
        let mut rdr = Reader::from_reader(b);

        // TODO check other options
        rdr.trim_text(true);

        PatentGrants {
            rdr,
            buf: Vec::new(),
        }
    }

    /// main entry point for deserialization
    ///
    /// returns None if no more data
    /// else if there's an error in deser (e.g. partial data)
    /// return Some(Result<_>)
    fn deser_patent_grant(&mut self) -> Option<Result<PatentGrant, Error>> {
        // first skip through headers
        let hdr = deser_header(&mut self.rdr, &mut self.buf);
        match hdr {
            Some(hdr_res) => {
                if let Err(err) = hdr_res {
                    return Some(Err(err));
                }
            },
            None => return None,
        }
        self.buf.clear();

        // if headers are in the right place, we can continue
        let mut patent_grant = PatentGrant::default();

        // deser for each element, update default patent grant
        loop {
            match self.rdr.read_event(&mut self.buf) {
                Ok(Event::PI(pi_bytes)) => {
                    try_some!(deser_top_pi(pi_bytes, &mut self.rdr, &mut patent_grant));
                },
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"us-claim-statement" => {
                            patent_grant.us_claim_statement = try_some!(deser_text_from(e.name(), &mut self.rdr));
                        },
                        b"claims" => {
                            try_some!(deser_claims(&mut self.rdr, &mut self.buf, &mut patent_grant));
                        },
                        b"us-bibliographic-data-grant" => {
                            try_some!(deser_biblio(&mut self.rdr, &mut self.buf, &mut patent_grant.us_bibliographic_data_grant));
                        },
                        _ => continue,
                    }
                },
                Ok(Event::Eof) => break,
                Ok(Event::End(e)) => {
                    if e.name() == b"us-patent-grant" {
                        break;
                    } else {
                        continue;
                    }
                },
                Ok(_) => continue,
                Err(err) => return Some(Err(Error::Deser { src: err.to_string() })),
            };
        }

        self.buf.clear();

        Some(Ok(patent_grant))
    }
}

impl<B: BufRead> Iterator for PatentGrants<B> {
    type Item = Result<PatentGrant, Error>;

    // clear buf after each PatentGrant;
    // in the future, when GAT lands,
    // the iterator will be able to borrow
    // the underlying data.
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.deser_patent_grant();
        self.buf.clear();

        res
    }
}

// helper fns for deser
// never clear buffer inside fn!

/// only returns None if there's no input. Otherwise
/// tries to parse, and will error if necessary.
fn deser_header<B: BufRead>(rdr: &mut quick_xml::Reader<B>, buf: &mut Vec<u8>) -> Option<Result<(), Error>> {
    // first match xml declaration
    match rdr.read_event(buf) {
        Ok(Event::Decl(_)) => (),
        Ok(Event::Eof) => return None,
        Ok(_) => return Some(Err(Error::Deser { src: "xml decl not found at head of patent grant xml".to_owned() })),
        Err(err) => return Some(Err(Error::Deser { src: err.to_string() })),
    }

    // then match doctype declaration
    match rdr.read_event(buf) {
        Ok(Event::DocType(_)) => Some(Ok(())),
        Ok(Event::Eof) => None,
        Ok(_) => Some(Err(Error::Deser { src: "doctype decl not found at head of patent grant xml".to_owned() })),
        Err(err) => Some(Err(Error::Deser { src: err.to_string() })),
    }
}

/// top level program instruction handling.
/// encompasses all possible descriptions in grant:
/// - brief-description-of-drawings
/// - BRFSUM (brief summary)
/// - RELAPP (other patent relations)
/// - DETDESC (detailed description)
/// - in-line-formulae
///
/// This one is a little more involved. The idea is to go from the top-level program instruction,
/// and find the next top-level instruction that has end = tail. In the meantime, all of the
/// bytes are being written to a new buffer instead of the overall buffer. That means that the
/// new buffer cvan then be converted directly to a string.
///
/// One downside of this string conversion: tags are lost (i guess quick-xml didn't think it needed
/// to save them)
fn deser_top_pi<B: BufRead>(
    pi_bytes: BytesText,
    rdr: &mut quick_xml::Reader<B>,
    patent_grant: &mut PatentGrant
    ) -> Result<(), Error>
{
    let pi_name_res = pi_bytes.unescape_and_decode(&rdr);
    let pi_name = match pi_name_res {
        Ok(ref s) => s.split_whitespace().nth(0).context(Deser { src: "No name for PI".to_string() })?,
        Err(_) => return Err(Error::Deser { src: "No name for PI".into() }),
    };

    let end = match pi_name_res {
        Ok(ref s) => s.split_whitespace().last().context(Deser { src: "No end for PI".to_string() })?,
        Err(_) => return Err(Error::Deser { src: "No end for PI".into() }),
    };

    if end != "end=\"lead\"" {
        // just skip if not lead; it means it's some other top level PI
        return Ok(());
    }

    // get end byte of PI.
    // find beginning byte of next PI.
    // get string in between
    let mut text_buf = Vec::new();
    loop {
        match rdr.read_event(&mut text_buf) {
            Ok(Event::PI(pi_bytes_2)) => {
                // just search for the next tail, don't need to match on name.
                let pi_2_res = pi_bytes_2.unescape_and_decode(&rdr);

                let end = match pi_2_res {
                    Ok(ref s) => s.split_whitespace().last().context(Deser { src: "No end for PI".to_string() })?,
                    Err(_) => return Err(Error::Deser { src: "No end for PI".into() }),
                };

                if end != "end=\"tail\"" {
                    // in case of nested PI; I don't care about them unless they're
                    // one of the description ones, so just grab it as part of text
                    continue;
                }

                break;
            },
            Ok(_) => continue,
            Err(err) => return Err(Error::Deser { src: err.to_string() }),

        }
    }
    let text = match String::from_utf8(text_buf.to_vec()) {
        Ok(s) => s,
        Err(err) => return Err(Error::Deser { src: err.to_string() }),
    };
    patent_grant.descriptions.insert(pi_name.to_string(), text);

    Ok(())
}

fn deser_claims<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    patent_grant: &mut PatentGrant
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"claim" {
                    match rdr.read_event(buf) {
                        Ok(Event::Start(ref e)) => {
                            if e.name() == b"claim-text" {
                                patent_grant.claims.push(deser_text_from(e.name(), rdr)?);
                            } else {
                                break;
                            }
                        },
                        Ok(_) => break,
                        Err(err) => return Err(Error::Deser { src: err.to_string() }),
                    }
                } else {
                    break; // if no claims, exit
                }
            },
            Ok(_) => break, // if there's no more claims, exit
            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

/// call after you hit biblio tag
fn deser_biblio<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    biblio: &mut BibliographicDataGrant,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"publication-reference" => {
                        deser_doc_id(rdr, buf, &mut biblio.publication_reference)?;
                    },
                    b"application-reference" => {
                        deser_doc_id(rdr, buf, &mut biblio.application_reference)?;
                    },
                    b"us-application-series-code" => {
                        biblio.us_application_series_code = deser_text_from(e.name(), rdr)?;
                    },
                    b"us-term-of-grant" => {
                        biblio.us_term_of_grant = deser_text(b"length-of-grant", rdr)?;
                    },
                    b"classification-locarno" => {
                        deser_class_locarno(rdr, buf, &mut biblio.classification_locarno)?;
                    },
                    b"classification-national" => {
                        deser_class_national(rdr, buf, &mut biblio.classification_national)?;
                    },
                    b"invention-title" => {
                        biblio.invention_title = deser_text_with_tags_from(e.name(), rdr)?;
                    },
                    b"number-of-claims" => {
                        biblio.number_of_claims = deser_text_from(e.name(), rdr)?;
                    },
                    b"us-exemplary-claim" => {
                        biblio.us_exemplary_claim = deser_text_from(e.name(), rdr)?;
                    },
                    b"us-field-of-classification-search" => {
                        deser_field_class_search(rdr, buf, &mut biblio.us_field_of_classification_search)?;
                    },
                    b"us-applicants" => {
                        deser_us_applicants(rdr, buf, &mut biblio.us_applicants)?;
                    },
                    b"inventors" => {
                        deser_inventors(rdr, buf, &mut biblio.inventors)?;
                    },
                    b"agents" => {
                        deser_agents(rdr, buf, &mut biblio.agents)?;
                    },
                    b"assignees" => {
                        deser_assignees(rdr, buf, &mut biblio.assignees)?;
                    },
                    b"examiners" => {
                        deser_examiners(rdr, buf, &mut biblio.examiners)?;
                    },

                    // TODO when all elements in, use this line instead
                    //_ => break,
                    _ => continue,
                }
            },
            Ok(Event::End(ref e)) => {
                if e.name() == b"us-bibliographic-data-grant" {
                    break;
                }
            },
            // TODO when all elements in, use this line instead
            // Ok(_) => return Err(Error::Deser { src: "found non-start-element not in biblio".to_string() }),
            // for now, can just break out of biblio loop
            Ok(_) => continue,
            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        };
    }

    Ok(())
}

/// pub struct DocumentId {
///     pub country: String,
///     pub doc_number: String,
///     pub kind: Option<String>,
///     pub date: String,
/// }
fn deser_doc_id<B: BufRead>(rdr: &mut quick_xml::Reader<B>, buf: &mut Vec<u8>, doc_id: &mut DocumentId) -> Result<(), Error> {
    parse_struct_update!(
        rdr,
        buf,
        "document-id",
        doc_id,
        // Required
        {
            b"country" => country,
            b"doc-number" => doc_number,
            b"date" => date,
        },
        // Option
        {
            b"kind" => kind,
        }
    );

    Ok(())
}

/// pub struct ClassificationLocarno {
///     pub edition: String,
///     pub main_classification: String,
/// }
fn deser_class_locarno<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    class_locarno: &mut ClassificationLocarno,
    ) -> Result<(), Error>
{
    parse_struct_update_from!(
        rdr,
        buf,
        "classification-locarno",
        class_locarno,
        // Required
        {
            b"edition" => edition,
            b"main-classification" => main_classification,
        },
        // Optional
        {}
    );

    Ok(())
}

/// pub struct ClassificationNational {
///     pub country: String,
///     pub main_classification: String,
/// }
fn deser_class_national<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    class_national: &mut ClassificationNational,
    ) -> Result<(), Error>
{
    parse_struct_update_from!(
        rdr,
        buf,
        "classification-national",
        class_national,
        // Required
        {
            b"country" => country,
            b"additional-info" => additional_info,
            b"main-classification" => main_classification,
        },
        // Optional
        {
            b"further-classification" => further_classification,
        }
    );

    Ok(())
}

/// pub struct UsFieldOfClassificationSearch {
///     pub classification_nationals: Vec<ClassificationNational>,
///     pub classification_cpc_text: Vec<String>,
///     pub classification_cpc_combinationtext: Vec<String>,
/// }
fn deser_field_class_search<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    field_class_search: &mut UsFieldOfClassificationSearch,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"classification-national" => {
                        let mut class_national = ClassificationNational::default();

                        parse_struct_update_from!(
                            rdr,
                            buf,
                            "classification-national",
                            class_national,
                            // Required
                            {
                                b"country" => country,
                                b"additional-info" => additional_info,
                                b"main-classification" => main_classification,
                            },
                            // Optional
                            {
                                b"further-classification" => further_classification,
                            }
                        );
                        field_class_search.classification_nationals.push(class_national);
                    },
                    b"classification-cpc-text" => {
                        field_class_search.classification_cpc_texts.push(
                            deser_text_from(e.name(), rdr)?
                        );
                    },
                    b"classification-cpc-combination-text" => {
                        field_class_search.classification_cpc_combination_texts.push(
                            deser_text_from(e.name(), rdr)?
                        );
                    },
                    _ => return Err(Error::Deser { src: format!("found element {:?}, not in us-field-of-classification-search", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "us-field-of-classification-search".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides classification-national") }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

/// pub struct UsApplicant {
///    pub sequence: String,
///    pub app_type: String,
///    pub designation: String,
///    pub applicant_authority_category: String,
///    pub addressbook: AddressBook,
///    pub residence: String, // Country
/// }
///
/// Deserializes a Vec of Applicant
///
/// called after tag us-applicants is already hit
fn deser_us_applicants<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    applicants: &mut Vec<UsApplicant>,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"us-applicant" => {
                        let mut applicant = UsApplicant::default();

                        // first update attributes
                        for attr_res in e.attributes() {
                            let attr = attr_res
                                .map_err(|err| Error::Deser { src: err.to_string() })?;

                            match attr.key {
                                b"sequence" => applicant.sequence = attr.unescape_and_decode_value(rdr).expect("never fail utf8?"),
                                b"app-type" => applicant.app_type = attr.unescape_and_decode_value(rdr).expect("never fail utf8?"),
                                b"designation" => applicant.designation = attr.unescape_and_decode_value(rdr).expect("never fail utf8?"),
                                b"applicant-authority-category" => applicant.applicant_authority_category = Some(attr.unescape_and_decode_value(rdr).expect("never fail utf8?")),
                                _ => return Err(Error::Deser { src: format!("unrecognized attr in us-applicant") }),
                            }
                        }

                        // now parse and update the addressbook
                        deser_addressbook(rdr, buf, &mut applicant.addressbook)?;

                        // TODO this is done in order for now; if need to do out of order w/
                        // addressbook, create a loop and match
                        applicant.residence = {
                            consume_start(rdr, buf, b"residence")?;
                            deser_text(b"country", rdr)?
                        };

                        applicants.push(applicant);
                    },
                    _ => return Err(Error::Deser { src: format!("found element {:?}, not us-applicant", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "us-applicants".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides us-applicants") }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

/// pub struct Inventor . {
///    pub sequence: String,
///    pub designation: String,
///    pub addressbook: AddressBook,
/// }
///
/// Deserializes a Vec of Inventor
///
/// called after tag us-applicants is already hit
fn deser_inventors<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    inventors: &mut Vec<Inventor>,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"inventor" => {
                        let mut inventor = Inventor::default();

                        // first update attributes
                        for attr_res in e.attributes() {
                            let attr = attr_res
                                .map_err(|err| Error::Deser { src: err.to_string() })?;

                            match attr.key {
                                b"sequence" => inventor.sequence = attr.unescape_and_decode_value(rdr).expect("never fail utf8?"),
                                b"designation" => inventor.designation = attr.unescape_and_decode_value(rdr).expect("never fail utf8?"),
                                _ => return Err(Error::Deser { src: format!("unrecognized attr in inventor") }),
                            }
                        }

                        // now parse and update the addressbook
                        deser_addressbook(rdr, buf, &mut inventor.addressbook)?;

                        inventors.push(inventor);
                    },
                    _ => return Err(Error::Deser { src: format!("found element {:?}, not inventor", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "inventors".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides inventors") }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

// TODO: refactor Agent, Inventor, UsApplicant into one deser method with params?
/// pub struct Agent {
///    pub sequence: String,
///    pub rep_type: String,
///    pub addressbook: AddressBook,
/// }
///
/// Deserializes a Vec of Agent
///
/// called after tag agents is already hit
fn deser_agents<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    agents: &mut Vec<Agent>,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"agent" => {
                        let mut agent = Agent::default();

                        // first update attributes
                        for attr_res in e.attributes() {
                            let attr = attr_res
                                .map_err(|err| Error::Deser { src: err.to_string() })?;

                            match attr.key {
                                b"sequence" => agent.sequence = attr.unescape_and_decode_value(rdr).expect("never fail utf8?"),
                                b"rep-type" => agent.rep_type = attr.unescape_and_decode_value(rdr).expect("never fail utf8?"),
                                _ => return Err(Error::Deser { src: format!("unrecognized attr in agent") }),
                            }
                        }

                        // now parse and update the addressbook
                        deser_addressbook(rdr, buf, &mut agent.addressbook)?;

                        agents.push(agent);
                    },
                    _ => return Err(Error::Deser { src: format!("found element {:?}, not agent", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "agents".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides agents") }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

// TODO: refactor Agent, Inventor, UsApplicant into one deser method with params?
/// pub struct Assignee {
///    pub orgname: Option<String>,
///    pub role: Option<String>,
///    pub addressbook: AddressBook,
/// }
///
/// Deserializes a Vec of Assignee
///
/// called after tag assignees is already hit
fn deser_assignees<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    assignees: &mut Vec<Assignee>,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"assignee" => {
                        let mut assignee = Assignee::default();

                        deser_assignee(rdr, buf, &mut assignee)?;
                        assignees.push(assignee);
                    },
                    _ => return Err(Error::Deser { src: format!("found element {:?}, not assignee", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "assignees".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides assignees") }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

/// assignee
fn deser_assignee<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    assignee: &mut Assignee,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"orgname" => {
                        let txt = deser_text_from(b"orgname", rdr)?;
                        assignee.orgname = Some(txt);
                    },
                    b"role" => {
                        let txt = deser_text_from(b"role", rdr)?;
                        assignee.role = Some(txt);
                    },
                    b"addressbook" => {
                        deser_addressbook_from(rdr, buf, &mut assignee.addressbook)?;
                    },
                    _ => return Err(Error::Deser { src: format!("found element {:?}, not in assignee", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "assignee".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides assignee") }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

/// #[derive(Debug)]
/// pub struct AddressBook {
///     pub orgname: Option<String>,
///     pub first_name: Option<String>,
///     pub last_name: Option<String>,
///     pub role: Option<String>,
/// 
///     // Address
///     pub city: Option<String>,
///     pub state: Option<String>,
///     pub country: Option<String>,
/// }
///
/// called before addressbook tag consumed
fn deser_addressbook<B: BufRead>(rdr: &mut quick_xml::Reader<B>, buf: &mut Vec<u8>, addressbook: &mut AddressBook) -> Result<(), Error> {
    consume_start(rdr, buf, b"addressbook")?;
    deser_addressbook_from(rdr, buf, addressbook)
}

fn deser_addressbook_from<B: BufRead>(rdr: &mut quick_xml::Reader<B>, buf: &mut Vec<u8>, addressbook: &mut AddressBook) -> Result<(), Error> {
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"orgname" => addressbook.orgname = Some(deser_text_from(e.name(), rdr)?),
                    b"first-name" => addressbook.first_name = Some(deser_text_from(e.name(), rdr)?),
                    b"last-name" => addressbook.last_name = Some(deser_text_from(e.name(), rdr)?),
                    b"role" => addressbook.role = Some(deser_text_from(e.name(), rdr)?),
                    b"address" => {
                        let address = &mut addressbook.address;

                        parse_struct_update_from!(
                            rdr,
                            buf,
                            "address",
                            address,
                            // Required
                            {
                            },
                            // Optional
                            {
                                b"city" => city,
                                b"state" => state,
                                b"country" => country,
                            }
                        );
                    }
                    _ => return Err(Error::Deser { src: format!("unrecognized element {:?} in addressbook", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "addressbook".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(e) => return Err(Error::Deser { src: format!("found non-start-element {:?} besides addressbook", e) }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

/// pub struct Examiners {
///    pub primary_examiner: Examiner,
/// }
///
/// pub struct Examiner {
///    pub first_name: String,
///    pub last_name: String,
///    pub department: String,
///
/// }
///
/// called after tag examiners is already hit
fn deser_examiners<B: BufRead>(
    rdr: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
    examiners: &mut Examiners,
    ) -> Result<(), Error>
{
    loop {
        match rdr.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"primary-examiner" => {
                        let primary_examiner = &mut examiners.primary_examiner;

                        parse_struct_update_from!(
                            rdr,
                            buf,
                            "primary-examiner",
                            primary_examiner,
                            {
                                b"first-name" => first_name,
                                b"last-name" => last_name,
                            },
                            {
                                b"department" => department,
                            }
                        );
                    },
                    b"assistant-examiner" => {
                        let assistant_examiner = &mut examiners.assistant_examiner;

                        parse_struct_update_from!(
                            rdr,
                            buf,
                            "assistant-examiner",
                            assistant_examiner,
                            {
                                b"first-name" => first_name,
                                b"last-name" => last_name,
                            },
                            {
                                b"department" => department,
                            }
                        );
                    },
                    _ => return Err(Error::Deser { src: format!("found element {:?}, not expected in examiners", std::str::from_utf8(e.name())) }),
                }
            },
            Ok(Event::End(e)) => {
                if e.name() == "examiners".as_bytes() {
                    break;
                } else {
                    continue;
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides examiners") }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    }

    Ok(())
}

/// call when the start tag has already been consumed, now you need the text to the end tag
fn deser_text_from<B: BufRead, K: AsRef<[u8]>>(end: K, rdr: &mut quick_xml::Reader<B>) -> Result<String, Error> {
    match rdr.read_text(end, &mut Vec::new()) {
        Ok(txt) => Ok(txt),
        Err(err) => Err(Error::Deser { src: format!("err: {}, position: {}", err, rdr.buffer_position()) }),
    }
}

/// call when the start tag has already been consumed, now you need the text to the end tag
fn deser_text<B: BufRead>(name: &[u8], rdr: &mut quick_xml::Reader<B>) -> Result<String, Error> {
    let mut buf = Vec::new();

    consume_start(rdr, &mut buf, name)?;

    buf.clear();

    match rdr.read_text(name, &mut buf) {
        Ok(txt) => Ok(txt),
        Err(err) => Err(Error::Deser { src: err.to_string() }),
    }
}

/// call when the start tag has already been consumed, now you need the text to the end tag
/// only deals with nested tags one level
fn deser_text_with_tags_from<B: BufRead, K: AsRef<[u8]>>(end: K, rdr: &mut quick_xml::Reader<B>) -> Result<String, Error> {
    // TODO don't allocate a new vec at each try?

//    loop {
        match rdr.read_text(&end, &mut Vec::new()) {
            Ok(txt) => Ok(txt),
            Err(_) => {
                println!("hitttt");
                deser_text_from(&end, rdr)
                // try one more time, to see if we can get past a tag at the beginning
                // of the text
                //match rdr.read_event(&mut Vec::new()) {
                //    Ok(Event::Start(ref e)) => {
                //        let tagged_txt = match rdr.read_text(e.name(), &mut Vec::new()) {
                //            Ok(txt) => txt,
                //            Err(err) => return Err(Error::Deser { src: format!("err: {}, position: {}", err, rdr.buffer_position()) }),
                //        };

                //        // now past tagged text, read to end tag
                //        let txt = deser_text_from(end, rdr)?;
                //        return Ok(tagged_txt + &txt);
                //    },
                //    Ok(_) => break,
                //    Err(err) => return Err(Error::Deser { src: format!("err: {}, position: {}", err, rdr.buffer_position()) }),
                //}
            },
        }
 //   }
    //Err(Error::Deser { src: format!("Could not deserialize string with embedded tags") })
}

