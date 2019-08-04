use quick_xml::{self, Reader};
use quick_xml::events::Event;
use std::io::BufRead;

use crate::data::*;

pub struct PatentGrants<B: BufRead> {
    rdr: quick_xml::Reader<B>,
}

impl<B: BufRead> PatentGrants<B> {
    pub fn from_reader(b: B) -> Self {
        PatentGrants {
            rdr: Reader::from_reader(b),
        }
    }
}

//impl Iterator 
