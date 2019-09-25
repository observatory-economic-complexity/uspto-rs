// TODO define errors per module? Check with https://github.com/shepmaster/snafu/issues/28 on
// thoughts about pub visibility for errors.

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility="pub(crate)")]
pub enum Error {
    // wait for fix to:
    // error[E0599]: no method named `as_error_source` found for type `&quick_xml::errors::Error` in the current scope
    // --> src/error.rs:3:17
    //  |
    //3 | #[derive(Debug, Snafu)]
    //  |                 ^^^^^
    //  |
    //  = note: the method `as_error_source` exists but the following trait bounds were not satisfied:
    //          `&quick_xml::errors::Error : snafu::AsErrorSource`
    //          `quick_xml::errors::Error : snafu::AsErrorSource`
    #[snafu(display("Xml Deserialization Error: {}", src))]
    Deser{ src: String },
    #[snafu(display("Fetch Error: {}", source))]
    Fetch{ source: reqwest::Error },
    #[snafu(display("Fetch Create File Error: {}", source))]
    CreateFile{ source: std::io::Error },
}

