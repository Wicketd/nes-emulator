use crate::types::Result;
use anyhow::Context;
use std::{
    path::Path,
    fs::File,
    io::Read,
};

const HEADER_LEN: usize = 3;
const HEADER_BYTES: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

#[derive(Error, Debug)]
pub enum RomError {
    #[error("invalid header signature")]
    HeaderSignatureInvalid,
}

pub struct Rom {
    prg_size: u32,
    chr_size: u32,
    bytes: Vec<u8>,
}

impl Rom {
    pub fn from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path)
            .with_context(|| "failed to read rom file")?;

        let header = Self::read_header(&mut file)?;

        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;

        Ok(Self {
            prg_size: header.prg_chunks as u32 * 16 * 1024,
            chr_size: header.chr_chunks as u32 * 8 * 1024,
            bytes,
        })
    }

    fn read_header(file: &mut File) -> Result<Header> {
        let mut buffer = [0u8; 16];
        file.read_exact(&mut buffer)?;

        if buffer[0..=HEADER_LEN] == HEADER_BYTES {
            Ok(Header {
                prg_chunks: buffer[4],
                chr_chunks: buffer[5],
            })
        } else {
            create_error!(RomError::HeaderSignatureInvalid)
        }
    }

    pub fn as_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

struct Header {
    prg_chunks: u8,
    chr_chunks: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROM_PATH: &str = "tests/rom/main.bin";

    fn rom() -> Rom {
        Rom::from_file(Path::new(ROM_PATH)).unwrap()
    }

    #[test]
    fn from_file() {
        let rom = rom();
        assert_eq!(rom.prg_size, 2 * 16 * 1024);
        assert_eq!(rom.chr_size, 8 * 1024);
        assert_ne!(rom.bytes[0..=HEADER_LEN], HEADER_BYTES);
    }
}
