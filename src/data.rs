//! data struct definitions for xml data

#[derive(Debug, Default)]
pub struct PatentGrant {
    pub us_bibliographic_data_grant: BibliographicDataGrant,
    pub drawings: Drawings,
    pub description: Description,
    pub us_claim_statement: String,
    pub claims: Claims,
}

#[derive(Debug, Default)]
pub struct BibliographicDataGrant {
    pub publication_reference: PublicationReference,
    pub application_reference: ApplicationReference,
    pub us_application_series_code: String,
    pub us_term_of_grant: LengthOfGrant,
    pub classification_locarno: ClassificationLocarno,
    pub classification_national: ClassificationNational,
    pub invention_title: InventionTitle,
//    pub us_references_cited: Vec<UsCitation>,
    pub number_of_claims: String,
    pub us_field_of_classifciation_search: UsFieldOfClassificationSearch,
    pub us_parties: UsParties,
    pub assignees: Assignees,
    pub examiners: Examiners,
}

#[derive(Debug, Default)]
pub struct PublicationReference {
    pub document_id: DocumentId,
}

#[derive(Debug, Default)]
pub struct ApplicationReference {
    pub document_id: DocumentId,
}

#[derive(Debug, Default)]
pub struct DocumentId {
    pub country: String,
    pub doc_number: String,
    pub kind: Option<String>,
    pub date: String,
}

#[derive(Debug, Default)]
pub struct LengthOfGrant {
    pub length_of_grant: String,
}

#[derive(Debug, Default)]
pub struct ClassificationLocarno {
    pub edition: String,
    pub main_classification: String,
}

#[derive(Debug, Default)]
pub struct ClassificationNational {
    pub country: String,
    pub main_classification: String,
}

#[derive(Debug, Default)]
pub struct InventionTitle {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Default)]
pub struct UsFieldOfClassificationSearch {
    pub classification_nationals: Vec<ClassificationNational>,

    pub classification_cpc_text: Vec<String>,
}

#[derive(Debug, Default)]
pub struct UsParties {
    pub us_applicants: UsApplicants,
    pub inventors: Inventors,
    pub agents: Agents,
}

#[derive(Debug, Default)]
pub struct UsApplicants {
    pub us_applicants: Vec<UsApplicant>,
}

#[derive(Debug)]
pub struct UsApplicant {
    pub sequence: String,
    pub app_type: String,
    pub designation: String,
    pub applicant_authority_category: String,
    pub addressbook: AddressBook,
    pub residence: Residence,
}

#[derive(Debug)]
pub struct AddressBook {
    pub orgname: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub address: Address,
}

#[derive(Debug)]
pub struct Address{
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug)]
pub struct Residence{
    pub country: String,
}

#[derive(Debug, Default)]
pub struct Inventors {
    pub inventors: Vec<Inventor>,
}

#[derive(Debug)]
pub struct Inventor {
    pub sequence: String,
    pub designation: String,
    pub addressbook: AddressBook,
}

#[derive(Debug, Default)]
pub struct Agents {
    pub agents: Vec<Agent>,
}

#[derive(Debug)]
pub struct Agent {
    pub sequence: String,
    pub rep_type: String,
    pub addressbook: AddressBook,
}

#[derive(Debug, Default)]
pub struct Assignees {
    pub assignee: Vec<Assignee>,
}

#[derive(Debug)]
pub struct Assignee {
    pub addressbook: AddressBook,
}

#[derive(Debug, Default)]
pub struct Examiners {
    pub primary_examiner: PrimaryExaminer,

    // TODO are there other examiners?
}

#[derive(Debug, Default)]
pub struct PrimaryExaminer {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
}

#[derive(Debug, Default)]
pub struct Drawings {
    id: String,
    figures: Vec<Figure>,
}

#[derive(Debug)]
pub struct Figure {
    id: String,
    num: String,
    img: Img,
}

#[derive(Debug)]
pub struct Img {
    id: String,
    he: String,
    wi: String,
    file: String,
    alt: String,
    img_content: String,
    img_format: String,
}

#[derive(Debug, Default)]
pub struct Description {
    id: String,
    description_of_drawings: String,
}

#[derive(Debug, Default)]
pub struct Claims {
    id: String,
    pub claims: Vec<Claim>,
}

#[derive(Debug)]
pub struct Claim {
    id: String,
    num: String,
    claim_text: String,
}

// TODO this is probably an html document, clean it after.
#[derive(Debug)]
pub struct DescriptionOfDrawings {
    body: String,
}

