use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use zip::ZipArchive;

use crate::error::CavpError;

type Result<T> = std::result::Result<T, CavpError>;

pub enum TestKind {
    SHA,
}

#[derive(Debug)]
struct ShaTriData {
    bit_len: u32,
    msg: String,
    md: String,
}

impl ShaTriData {
    fn new(bit_len: u32, msg: String, md: String) -> Self {
        Self { bit_len, msg, md }
    }
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
                let mut out =
                    File::create(self.test_root.join(Path::new("shabytetestvectors.zip")))?;
                io::copy(&mut bytes.as_ref(), &mut out)?;

                let mut archive = ZipArchive::new(File::open(
                    self.test_root.join(Path::new("shabytetestvectors.zip")),
                )?)
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

    fn tri_parse(&self, file_name: &Path) -> Result<Vec<ShaTriData>> {
        let mut res = vec![];

        let file = std::fs::File::open(self.test_root.join(file_name))?;

        let mut len = 0;
        let mut msg = "".to_string();
        let mut md = "".to_string();
        for line in std::io::BufReader::new(file).lines() {
            if let Ok(data) = line {
                let datas = data.split(" ").collect::<Vec<&str>>();
                if let Some(prefix) = datas.first() {
                    if *prefix == "Len" {
                        len = datas[2].parse().unwrap();
                    }
                    if *prefix == "Msg" {
                        msg = datas[2].to_string();
                    }
                    if *prefix == "MD" {
                        md = datas[2].to_string();
                        res.push(ShaTriData::new(len, msg.clone(), md.clone()));
                    }
                }
            }
        }

        println!("res: {:?}", res);

        Ok(res)
    }

    pub fn sha1(&self) -> Result<()> {
        self.tri_parse(Path::new("shabytetestvectors/SHA1ShortMsg.rsp"))?;
        Ok(())
    }
}
