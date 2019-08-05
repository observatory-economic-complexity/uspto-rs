#[macro_export]
macro_rules! try_some {
    ($e:expr) => (
        match $e {
            Ok(x) => x,
            Err(err) => return Some(Err(err)),
        }
    )
}

