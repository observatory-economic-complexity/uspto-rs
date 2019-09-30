pub mod data;
mod deserialize;
pub mod error;
pub mod fetch;
mod serialize;
pub mod util;

pub use crate::deserialize::PatentGrants;
pub use crate::error::Error;
pub use crate::serialize::PatentOutput;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
