// Do I have to load in entire file before splitting and parsing each block?
// there's probably no other way to do it, since they insert a xml and doctype
// between each patent grant

#![feature(custom_attribute)]

use serde_xml_rs;
use snafu::{Snafu, ResultExt, OptionExt};
use std::fs;
use uspto::deserialize;

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

    let xml_data = fs::File::open(data_filepath)
        .context(OpenDataFile)?;

    // deserialize returns an iter of PatentGrant
    let patents = uspto::deserialize();

    //println!("patents: {}", patents.len());

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

