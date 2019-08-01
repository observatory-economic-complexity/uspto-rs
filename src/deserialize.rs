//! deserialize the xml data

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename="us-patent-grant")]
pub struct PatentGrant {
    #[serde(rename="us-bibliographic-data-grant")]
    pub us_bibliographic_data_grant: BibliographicDataGrant,

    pub drawings: Drawings,

    pub description: Description,

    #[serde(rename="us-claim-statement")]
    pub us_claim_statement: String,

    pub claims: Claims,
}

#[derive(Debug, Deserialize)]
pub struct BibliographicDataGrant {
    #[serde(rename="publication-reference")]
    pub publication_reference: PublicationReference,

    #[serde(rename="application-reference")]
    pub application_reference: ApplicationReference,

    #[serde(rename="us-application-series-code")]
    pub us_application_series_code: String,

    #[serde(rename="us-term-of-grant")]
    pub us_term_of_grant: LengthOfGrant,

    #[serde(rename="classification-locarno")]
    pub classification_locarno: ClassificationLocarno,

    #[serde(rename="classification-national")]
    pub classification_national: ClassificationNational,

    #[serde(rename="invention-title")]
    pub invention_title: InventionTitle,

//    #[serde(rename="us-references-cited")]
//    pub us_references_cited: Vec<UsCitation>,

    #[serde(rename="number-of-claims")]
    pub number_of_claims: String,

    #[serde(rename="us-field-of-classification-search")]
    pub us_field_of_classifciation_search: UsFieldOfClassificationSearch,

    #[serde(rename="us-parties")]
    pub us_parties: UsParties,

    pub assignees: Assignees,

    pub examiners: Examiners,
}

#[derive(Debug, Deserialize)]
pub struct PublicationReference {
    #[serde(rename="document-id")]
    pub document_id: DocumentId,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationReference {
    #[serde(rename="document-id")]
    pub document_id: DocumentId,
}

#[derive(Debug, Deserialize)]
pub struct DocumentId {
    pub country: String,
    #[serde(rename="doc-number")]
    pub doc_number: String,
    pub kind: Option<String>,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct LengthOfGrant {
    #[serde(rename="length-of-grant")]
    pub length_of_grant: String,
}

#[derive(Debug, Deserialize)]
pub struct ClassificationLocarno {
    pub edition: String,
    #[serde(rename="main-classification")]
    pub main_classification: String,
}

#[derive(Debug, Deserialize)]
pub struct ClassificationNational {
    pub country: String,
    #[serde(rename="main-classification")]
    pub main_classification: String,
}

#[derive(Debug, Deserialize)]
pub struct InventionTitle {
    pub id: String,
    #[serde(rename="$value")]
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UsFieldOfClassificationSearch {
    #[serde(rename="classification-national")]
    pub classification_nationals: Vec<ClassificationNational>,

    #[serde(rename="classification-cpc-text")]
    pub classification_cpc_text: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UsParties {
    #[serde(rename="us-applicants")]
    pub us_applicants: UsApplicants,

    pub inventors: Inventors,

    pub agents: Agents,
}

#[derive(Debug, Deserialize)]
pub struct UsApplicants {
    #[serde(rename="us-applicant")]
    pub us_applicants: Vec<UsApplicant>,
}

#[derive(Debug, Deserialize)]
pub struct UsApplicant {
    pub sequence: String,
    #[serde(rename="app-type")]
    pub app_type: String,
    pub designation: String,
    #[serde(rename="applicant-authority-category")]
    pub applicant_authority_category: String,
    pub addressbook: AddressBook,
    pub residence: Residence,
}

#[derive(Debug, Deserialize)]
pub struct AddressBook {
    pub orgname: Option<String>,
    #[serde(rename="first-name")]
    pub first_name: Option<String>,
    #[serde(rename="last-name")]
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub address: Address,
}

#[derive(Debug, Deserialize)]
pub struct Address{
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Residence{
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct Inventors {
    #[serde(rename="inventor")]
    pub inventors: Vec<Inventor>,
}

#[derive(Debug, Deserialize)]
pub struct Inventor {
    pub sequence: String,
    pub designation: String,
    pub addressbook: AddressBook,
}

#[derive(Debug, Deserialize)]
pub struct Agents {
    #[serde(rename="agent")]
    pub agents: Vec<Agent>,
}

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub sequence: String,
    #[serde(rename="rep-type")]
    pub rep_type: String,
    pub addressbook: AddressBook,
}

#[derive(Debug, Deserialize)]
pub struct Assignees {
    #[serde(rename="assignee")]
    pub assignee: Vec<Assignee>,
}

#[derive(Debug, Deserialize)]
pub struct Assignee {
    pub addressbook: AddressBook,
}

#[derive(Debug, Deserialize)]
pub struct Examiners {
    #[serde(rename="primary-examiner")]
    pub primary_examiner: PrimaryExaminer,

    // TODO are there other examiners?
}

#[derive(Debug, Deserialize)]
pub struct PrimaryExaminer {
    #[serde(rename="first-name")]
    pub first_name: String,
    #[serde(rename="last-name")]
    pub last_name: String,
    pub department: String,
}

#[derive(Debug, Deserialize)]
pub struct Drawings {
    id: String,
    #[serde(rename="figure")]
    figures: Vec<Figure>,
}

#[derive(Debug, Deserialize)]
pub struct Figure {
    id: String,
    num: String,
    img: Img,
}

#[derive(Debug, Deserialize)]
pub struct Img {
    id: String,
    he: String,
    wi: String,
    file: String,
    alt: String,
    #[serde(rename="img-content")]
    img_content: String,
    #[serde(rename="img-format")]
    img_format: String,
}

// TODO need to implement deserializer to get a raw string of description
#[derive(Debug, Deserialize)]
pub struct Description {
    id: String,
    #[serde(rename="description-of-drawings")]
    #[serde(deserialize_with="deserialize_description_of_drawings")]
    description_of_drawings: String,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    id: String,
    #[serde(rename="claim")]
    pub claims: Vec<Claim>,
}

#[derive(Debug, Deserialize)]
pub struct Claim {
    id: String,
    num: String,
    #[serde(rename="claim-text")]
    claim_text: String,
}

#[derive(Debug)]
pub struct DescriptionOfDrawings {
    body: String,
}

// write custom deserialize for DescriptionOfDrawings
use serde::Deserializer;
use xml::reader::XmlEvent;
pub fn deserialize_description_of_drawings<'de, D>(deser: D) -> Result<String, D::Error>
    where D: Deserializer<'de>
{
    println!("hit");
    let deser_result: String = serde::Deserialize::deserialize(deser)?;
    println!("{:?}", deser_result);
    Ok("".to_owned())
}
