//use chrono::{Utc, Datelike};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use snafu::ResultExt;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::Error;
use crate::error::{Fetch, CreateFile};

lazy_static!{
    static ref DIR_RE: Regex = Regex::new(r#"ipgb[0-9]{8}_wk[0-9]{2}\.zip"#).unwrap();
}

#[derive(Debug)]
pub struct FetchGrants {
    year_min: i32,
    year_max: i32,
    listings: HashMap<i32, Vec<String>>,
    target: PathBuf,
}

impl FetchGrants {
    pub fn new(year_min: i32, year_max: i32, target_dir: PathBuf) -> Self {

        //let current_year = Utc::now().year();

        Self {
            year_min,
            year_max,
            listings: HashMap::new(),
            target: target_dir,
        }
    }

    pub fn fetch_listings(&mut self) -> Result<(), Error> {
        let years = self.year_min..=self.year_max;

        for year in years {
            println!("Fetching listing for {}", year);
            let listing = fetch_year_listing(year)?;
            let year_listing = self.listings.entry(year).or_default();
            year_listing.extend_from_slice(&listing);
        }

        Ok(())
    }

    /// fetch one year only.
    pub fn fetch_file_year(&self, year: i32) -> Result<(), Error> {
        println!("Fetching files for {}", year);

        // get year listing from cache
        let listing = self.listings.get(&year).expect("make error for this; fetch listings must run first");

        for file_name in listing {
            println!("Fetching file {}", file_name);
            //io copy from response to target
            let target_filepath = self.target.join(file_name.as_str());
            let mut target_file = std::fs::File::create(target_filepath)
                .context(CreateFile)?;

            std::io::copy(
                &mut fetch_file_week(year, file_name.as_str())?,
                &mut target_file,
            )
            .context(CreateFile)?;
        }

        Ok(())
    }

    /// fetch all years (from 1976)
    pub fn fetch_all(&self) -> Result<(), Error> {
        println!("Fetching all for {:?}", self);
        let years = self.year_min..=self.year_max;

        for year in years {
            self.fetch_file_year(year)?;
        }
        Ok(())
    }
}

/// fetch one week
/// id is of the format "20190101_wk01"
///
/// TODO should this just write straight to disk?
pub fn fetch_file_week(year: i32, file_name: &str) -> Result<reqwest::Response, Error> {
    let url = format!("https://bulkdata.uspto.gov/data/patent/grant/redbook/bibliographic/{}/{}", year, file_name);
    reqwest::get(&url)
        .context(Fetch)
}

/// fetch_year_listing
/// - get year's listing
/// - find file names
/// - save to hashmap
pub fn fetch_year_listing(year: i32) -> Result<Vec<String>, Error> {
    let dir_url = format!("https://bulkdata.uspto.gov/data/patent/grant/redbook/bibliographic/{}", year);

    let dir_listing = reqwest::get(&dir_url)
        .context(Fetch)?
        .text()
        .context(Fetch)?;

    Ok(
        DIR_RE.find_iter(&dir_listing).map(|m| m.as_str().to_string()).collect()
    )
}
