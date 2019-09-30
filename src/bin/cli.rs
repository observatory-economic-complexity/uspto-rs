// Do I have to load in entire file before splitting and parsing each block?
// there's probably no other way to do it, since they insert a xml and doctype
// between each patent grant

#![feature(custom_attribute)]

use snafu::{Snafu, ResultExt};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;
use uspto::PatentGrants;
use uspto::fetch;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

fn run() -> Result<(), Error> {
    let opts = CliOpt::from_args();

    match opts.command {
        Command::Fetch { year, target_dir }=> {
            // for now, just one year
            let mut fetcher = fetch::FetchGrants::new(year, year, target_dir);

            fetcher.fetch_listings()
                .context(UsPto)?;

            fetcher.fetch_all()
                .context(UsPto)?;

            Ok(())
        },
        Command::Process { data_filepath } => {
            process(data_filepath)
        },
    }
}

fn process(path: PathBuf) -> Result<(), Error> {
    let f = fs::File::open(path)
        .context(OpenDataFile)?;
    let f = BufReader::new(f);

    // deserialize returns an iter of PatentGrant
    let patents = PatentGrants::from_reader(f);

    for patent_res in patents {
        match patent_res {
            Ok(patent) => {


                //if patent.us_bibliographic_data_grant.publication_reference.doc_number == "RE047539" {
                //    println!("{:#?}", patent);
                //}
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
                println!("{:#?}", patent);
            },
            Err(err) => {
                eprintln!("{}", err);
                break;
            },
        }
    }


    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name="uspto")]
struct CliOpt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name="fetch")]
    Fetch {
        // TODO this currently only allows one year.
        #[structopt(long="year")]
        year: i32,
        #[structopt(long="target-dir", parse(from_os_str))]
        target_dir: PathBuf,
    },
    #[structopt(name="process")]
    Process {
        #[structopt(parse(from_os_str))]
        data_filepath: PathBuf,
    },
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Cli Error: missing filepath"))]
    CliNoPath,
    #[snafu(display("Open Datafile Error: {}", source))]
    OpenDataFile { source: std::io::Error },
    #[snafu(display("Read Datafile Error: {}", source))]
    ReadDataFile { source: std::io::Error },
    #[snafu(display("USPTO lib Error: {}", source))]
    UsPto { source: uspto::Error },
}

