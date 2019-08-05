use quick_xml::{self, Reader};
use quick_xml::events::Event;
use std::io::BufRead;

use crate::data::*;
use crate::error::Error;

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

        // deser for each element, change default patent grant
        loop {
            match self.rdr.read_event(&mut self.buf) {
                Ok(Event::PI(pi_bytes)) => {
                    let pi_name_res = pi_bytes.unescape_and_decode(&self.rdr);
                    let pi_name = match pi_name_res {
                        Ok(s) => s.split_whitespace().nth(0).expect("no name for PI").to_string(),
                        Err(err) => return Some(Err(Error::Deser { src: "No name for PI".into() })),
                    };

                    // get end byte of PI.
                    // find beginning byte of next PI.
                    // get string in between
                    let mut text_buf = Vec::new();
                    let mut pi_name_2 = String::new();
                    loop {
                        match self.rdr.read_event(&mut text_buf) {
                            Ok(Event::PI(pi_bytes_2)) => {
                                let pi_name_2_res = pi_bytes_2.unescape_and_decode(&self.rdr);
                                pi_name_2 = match pi_name_2_res {
                                    Ok(s) => s.split_whitespace().nth(0).expect("no name for PI").to_string(),
                                    Err(err) => return Some(Err(Error::Deser { src: "No name for PI".into() })),
                                };
                                break;
                            },
                            Ok(_) => continue,
                            Err(err) => return Some(Err(Error::Deser { src: err.to_string() })),

                        }
                    }
                    let text = String::from_utf8(text_buf.to_vec()).expect("invalid utf8");
                    // patent_grant.description = description_text
                    println!("pi_name: {:?}", pi_name);
                    println!("pi_name_2: {:?}", pi_name_2);
                    println!("{}", text);
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

