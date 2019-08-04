use quick_xml::{self, Reader};
use quick_xml::events::Event;
use snafu::{Snafu, ResultExt, OptionExt};
use std::io::BufRead;

use crate::data::*;
use crate::error::Error;

pub struct PatentGrants<B: BufRead> {
    rdr: quick_xml::Reader<B>,
    buf: Vec<u8>,
}

impl<B: BufRead> PatentGrants<B> {
    pub fn from_reader(b: B) -> Self {
        let mut rdr = Reader::from_reader(b);

        // TODO check other options
        rdr.trim_text(true);

        PatentGrants {
            rdr,
            buf: Vec::new(),
        }
    }

    /// main entry point for deserialization
    ///
    /// returns None if no more data
    /// else if there's an error in deser (e.g. partial data)
    /// return Some(Result<_>)
    fn deser_patent_grant(&mut self) -> Result<PatentGrant, Error> {
        // first skip through headers
        deser_header(&mut self.rdr, &mut self.buf)?;
        self.buf.clear();

        // if headers are in the right place, we can continue
        let mut patent_grant = PatentGrant::default();

        Ok(patent_grant)
    }
}

impl<B: BufRead> Iterator for PatentGrants<B> {
    type Item = Result<PatentGrant, Error>;

    // clear buf after each PatentGrant;
    // in the future, when GAT lands,
    // the iterator will be able to borrow
    // the underlying data.
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.deser_patent_grant();
        self.buf.clear();

        Some(res)
    }
}

// helper fns for deser
// never clear buffer inside fn!

fn deser_header<B: BufRead>(rdr: &mut quick_xml::Reader<B>, buf: &mut Vec<u8>) -> Result<(), Error> {
    // first match xml declaration
    match rdr.read_event(buf) {
        Ok(Event::Decl(_)) => (),
        Ok(_) => return Err(Error::Deser { src: "xml decl not found at head of patent grant xml".to_owned() }),
        Err(err) => return Err(Error::Deser { src: err.to_string() }),
    }

    // then match doctype declaration
    match rdr.read_event(buf) {
        Ok(Event::DocType(_)) => Ok(()),
        Ok(_) => return Err(Error::Deser { src: "doctype decl not found at head of patent grant xml".to_owned() }),
        Err(err) => return Err(Error::Deser { src: err.to_string() }),
    }
}
