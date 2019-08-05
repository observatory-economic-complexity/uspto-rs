//! data struct definitions for xml data

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PatentGrant {
    pub us_bibliographic_data_grant: BibliographicDataGrant,

    //pub drawings: Drawings,

    // encompasses all possible descriptions in grant:
    // - brief-description-of-drawings
    // - BRFSUM (brief summary)
    // - RELAPP (other patent relations)
    // - DETDESC (detailed description)
    // - in-line-formulae
    pub descriptions: HashMap<String, String>,

    pub us_claim_statement: String,
    pub claims: Vec<String>,
}

#[derive(Debug, Default)]
pub struct BibliographicDataGrant {
    pub publication_reference: DocumentId,
    pub application_reference: DocumentId,
    pub us_application_series_code: String,
    pub us_term_of_grant: String,
    pub classification_locarno: ClassificationLocarno,
    pub classification_national: ClassificationNational,
    pub invention_title: InventionTitle,
//    pub us_references_cited: Vec<UsCitation>,
    pub number_of_claims: String,
    pub us_field_of_classification_search: UsFieldOfClassificationSearch,

    // ==================
    // Us Parties
    pub us_applicants: Vec<UsApplicant>,
    pub inventors: Vec<Inventor>,
    pub agents: Vec<Agent>,
    // ==================

    pub assignees: Vec<Assignee>,
    pub examiners: Examiners,
}

#[derive(Debug, Default)]
pub struct DocumentId {
    pub country: String,
    pub doc_number: String,
    pub kind: Option<String>,
    pub date: String,
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

#[derive(Debug)]
pub struct UsApplicant {
    pub sequence: String,
    pub app_type: String,
    pub designation: String,
    pub applicant_authority_category: String,
    pub addressbook: AddressBook,
    pub residence: String, // Country
}

#[derive(Debug)]
pub struct AddressBook {
    pub orgname: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,

    // Address
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug)]
pub struct Inventor {
    pub sequence: String,
    pub designation: String,
    pub addressbook: AddressBook,
}

#[derive(Debug)]
pub struct Agent {
    pub sequence: String,
    pub rep_type: String,
    pub addressbook: AddressBook,
}

#[derive(Debug)]
pub struct Assignee {
    pub addressbook: AddressBook,
}

#[derive(Debug, Default)]
pub struct Examiners {
    pub primary_examiner: Examiner,

    // TODO are there other examiners?
}

#[derive(Debug, Default)]
pub struct Examiner {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
}

