pub mod data;
mod deserialize;

pub use crate::deserialize::PatentGrants;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
