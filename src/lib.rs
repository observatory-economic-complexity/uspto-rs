pub mod data;
mod deserialize;
pub mod error;

pub use crate::deserialize::PatentGrants;
pub use crate::error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
