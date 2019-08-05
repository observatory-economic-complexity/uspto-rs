#[macro_export]
macro_rules! try_some {
    ($e:expr) => (
        if let Err(err) = $e {
            return Some(Err(err));
        }
    )
}

