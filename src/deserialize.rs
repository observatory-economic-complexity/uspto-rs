use quick_xml::{self, Reader};
use quick_xml::events::Event;
use std::io::BufRead;

use crate::data::*;
use crate::error::Error;

pub struct PatentGrants<B: BufRead> {
    rdr: quick_xml::Reader<B>,
}

impl<B: BufRead> PatentGrants<B> {
    pub fn from_reader(b: B) -> Self {
        PatentGrants {
            rdr: Reader::from_reader(b),
        }
    }

    // returns None if no more data
    // else if there's an error in deser (e.g. partial data)
    // return Some(Result<_>)
    fn deser_patent_grant(&mut self) -> Option<Result<PatentGrant, Error>> {
        None
    }
}

impl<B: BufRead> Iterator for PatentGrants<B> {
    type Item = Result<PatentGrant, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.deser_patent_grant()
    }
}
