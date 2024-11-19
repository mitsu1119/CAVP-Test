use std::{
    fs::{self, File},
    io,
    path::Path,
};

use crate::error::CavpError;

type Result<T> = std::result::Result<T, CavpError>;

pub enum TestKind {
    SHA1,
}

#[derive(Debug)]
pub struct CavpTest<'a> {
    test_root: &'a Path,
}

impl<'a> CavpTest<'a> {
    const SHA_BYTE_URL: &'a str = "https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/shabytetestvectors.zip";

    pub fn new(test_dir: &'a str) -> Result<Self> {
        let test_root = Path::new(test_dir);
        if test_root.is_dir() {
            Ok(Self { test_root })
        } else {
            fs::create_dir(test_root)?;
            Ok(Self { test_root })
        }
    }
}
