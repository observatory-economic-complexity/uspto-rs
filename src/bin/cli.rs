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

    let mut count = 0;
    for patent_res in patents.skip(1000).take(10) {
        match patent_res {
            Ok(patent) => {
                println!("{:?}", patent.descriptions);
                count += 1;
                println!("patent count: {}", count);
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

