// Do I have to load in entire file before splitting and parsing each block?
// there's probably no other way to do it, since they insert a xml and doctype
// between each patent grant

#![feature(custom_attribute)]

use snafu::{Snafu, ResultExt, OptionExt};
use std::fs;
use std::io::BufReader;
use uspto::PatentGrants;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

fn run() -> Result<(), Error> {
    let data_filepath = std::env::args()
        .nth(1)
        .context(CliNoPath)?;

    let f = fs::File::open(data_filepath)
        .context(OpenDataFile)?;
    let f = BufReader::new(f);

    // deserialize returns an iter of PatentGrant
    let patents = PatentGrants::from_reader(f);

    for patent_res in patents {
        match patent_res {
            Ok(patent) => {


                if patent.claims.iter().any(|claim| claim.contains("represents an organic group")) {
                    println!("{:#?}", patent);
                }
                //println!("{:#?}", patent.us_bibliographic_data_grant.examiners);
                //println!("{:#?}", patent.us_bibliographic_data_grant.assignees);
                //println!("{:#?}", patent.us_bibliographic_data_grant.agents);
                //println!("{:#?}", patent.us_bibliographic_data_grant.inventors);
                //println!("{:#?}", patent.us_bibliographic_data_grant.us_applicants);
                //println!("{:#?}", patent.us_bibliographic_data_grant.us_field_of_classification_search);
                //println!("{:#?}", patent.us_bibliographic_data_grant.number_of_claims);
                //println!("{:#?}", patent.us_bibliographic_data_grant.us_exemplary_claim);
                //println!("{:#?}", patent.us_bibliographic_data_grant.invention_title);
                //println!("{:#?}", patent.us_bibliographic_data_grant.us_term_of_grant);
                //println!("{:#?}", patent.us_bibliographic_data_grant.us_application_series_code);
                //println!("{:#?}", patent.us_bibliographic_data_grant.classification_national);
                //println!("{:#?}", patent.us_bibliographic_data_grant.classification_locarno);
                //println!("{:#?}", patent.us_bibliographic_data_grant.application_reference);
                //println!("{:#?}", patent.us_bibliographic_data_grant.publication_reference);
                //println!("{:#?}", patent.descriptions);
                //println!("{:#?}", patent.us_claim_statement);
                //println!("{:#?}", patent.claims);
            },
            Err(err) => {
                eprintln!("{}", err);
                break;
            },
        }
    }


    Ok(())
}


#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Cli Error: missing filepath"))]
    CliNoPath,
    #[snafu(display("Open Datafile Error: {}", source))]
    OpenDataFile { source: std::io::Error },
    #[snafu(display("Read Datafile Error: {}", source))]
    ReadDataFile { source: std::io::Error },
}

