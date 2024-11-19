use std::{
    fs::{self, File},
    io,
    path::Path,
};

use zip::ZipArchive;

use crate::error::CavpError;

type Result<T> = std::result::Result<T, CavpError>;

pub enum TestKind {
    SHA,
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

    pub async fn download(&self, test_kind: TestKind) -> Result<()> {
        match test_kind {
            TestKind::SHA => {
                let response = reqwest::get(Self::SHA_BYTE_URL).await?;
                let bytes = response.bytes().await?;
                let mut out = File::create(self.test_root.join(Path::new("sha1byte.zip")))?;
                io::copy(&mut bytes.as_ref(), &mut out)?;

                let mut archive =
                    ZipArchive::new(File::open(self.test_root.join(Path::new("sha1byte.zip")))?)
                        .unwrap();
                for i in 0..archive.len() {
                    let mut f = archive.by_index(i).unwrap();
                    let name = f.name().to_string();
                    let file_path = self.test_root.join(Path::new(&name));
                    let prefix = file_path.parent().unwrap();
                    std::fs::create_dir_all(prefix).unwrap();
                    let mut out = File::create(file_path)?;
                    io::copy(&mut f, &mut out)?;
                }
            }
        }
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        fs::remove_dir_all(self.test_root)?;
        Ok(())
    }
}
