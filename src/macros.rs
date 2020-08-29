#![macro_use]

macro_rules! create_error {
    ($error:expr) => { Err($error.into()) };
}

#[cfg(test)]
macro_rules! load_test_rom {
    ($name:literal) => {{
        let path = format!("rom/{}.bin", $name);
        $crate::rom::Rom::from_file(Path::new(&path)).unwrap()
    }};
}
