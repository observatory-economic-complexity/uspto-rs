use serde::Serialize;
use std::convert::From;

use crate::data::PatentGrant;

/// Output format (csv) to be ingested into rows of cube
///
/// No measures; just count
#[derive(Serialize)]
pub struct PatentOutput {
    id: String,
    date: String,
    country_inventor: Vec<String>,
    country_assignee: Vec<String>,
    classification_locarno: String,
    classification_national: String,
}

impl From<&PatentGrant> for PatentOutput {
    fn from(pg: &PatentGrant) -> Self {
        let dg = &pg.us_bibliographic_data_grant;

        let country_inventor = dg
            .inventors
            .iter()
            .filter_map(|inventor| inventor.addressbook.address.country.as_ref())
            .cloned()
            .collect();
        let country_assignee = dg
            .assignees
            .iter()
            .filter_map(|assignee| assignee.addressbook.address.country.as_ref())
            .cloned()
            .collect();

        PatentOutput {
            id: dg.publication_reference.doc_number.clone(),
            date: dg.publication_reference.date.clone(),
            country_inventor,
            country_assignee,
            classification_locarno: dg.classification_locarno.main_classification.clone(),
            classification_national: dg.classification_national.main_classification.clone(),
        }
    }
}
