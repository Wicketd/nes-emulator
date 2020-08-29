#![macro_use]

macro_rules! create_error {
    ($error:expr) => { Err($error.into()) };
}
