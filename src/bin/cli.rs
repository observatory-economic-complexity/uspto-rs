// Do I have to load in entire file before splitting and parsing each block?
// there's probably no other way to do it, since they insert a xml and doctype
// between each patent grant

#![feature(custom_attribute)]

use serde_xml_rs;
use snafu::{Snafu, ResultExt, OptionExt};
use std::fs;
use uspto::deserialize::PatentGrant;

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

    let xml_data = fs:: read_to_string(data_filepath)
        .context(OpenDataFile)?;

    let xml_patents = xml_data.split(SPLIT_PATENTS);

    let patents: Vec<PatentGrant> = xml_patents
        .map(|xml_patent| {
            serde_xml_rs::from_reader(xml_patent.as_bytes())
            .context(Deser)
        })
        .collect::<Result<_,_>>()?;

    println!("{:#?}", patents[0]);

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
    #[snafu(display("Deserialize Error: {}", source))]
    Deser{ source: serde_xml_rs::Error },
}

// TODO use regex to capture this from each file before splitting and parsing.
const SPLIT_PATENTS: &str =
r#"<?xml version="1.0" encoding="UTF-8"?>
!DOCTYPE us-patent-grant SYSTEM "us-patent-grant-v45-2014-04-03.dtd" [ ]>"#;
