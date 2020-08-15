#![macro_use]

macro_rules! error {
    ($error:path) => { Err($error.into()) };
}
