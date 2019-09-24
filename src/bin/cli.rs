// Do I have to load in entire file before splitting and parsing each block?
// there's probably no other way to do it, since they insert a xml and doctype
// between each patent grant

#![feature(custom_attribute)]

use snafu::{Snafu, ResultExt, OptionExt};
use std::ffi::OsStr;
use std::fs;
use std::io::BufReader;
use std::path::Path;
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
    let path = Path::new(&data_filepath);

    let zip_ext = OsStr::new("zip");

    let patents = match path.extension() {
        Some(ext) => {
            if ext == zip_ext {
                println!("using zip {:?}", path);
                // test
                use std::io::Read;
                let f = std::fs::File::open(path).unwrap();
                let f = std::io::BufReader::new(f);
                let mut deflater = flate2::bufread::DeflateDecoder::new(f);
                let mut s = String::new();
                deflater.read_to_string(&mut s).unwrap();
                println!("{}", s);

                //real
                PatentGrants::from_zip(&path)
                    .expect("couldn't create patent grants iter from zip; make real errors later")
            } else {
                PatentGrants::from_path(&path)
                    .expect("couldn't create patent grants iter from file; make real errors later")
            }
        },
        _ => {
            PatentGrants::from_path(&path)
                .expect("couldn't create patent grants iter from file; make real errors later")
        },
    };

    // deserialize returns an iter of PatentGrant
    let patents = PatentGrants::from_path(&path)
        .expect("couldn't create patent grants iter; make real errors later");

    for patent_res in patents {
        match patent_res {
            Ok(patent) => {


                if patent.us_bibliographic_data_grant.publication_reference.doc_number == "RE047539" {
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
                //println!("{:#?}", patent);
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

