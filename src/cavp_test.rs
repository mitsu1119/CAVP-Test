use std::{fs, io, path::Path};

#[derive(Debug)]
pub struct CavpTest<'a> {
    test_root: &'a Path,
}

impl<'a> CavpTest<'a> {
    pub fn new(test_dir: &'a str) -> Result<Self, io::Error> {
        let test_root = Path::new(test_dir);
        if test_root.is_dir() {
            Ok(Self { test_root })
        } else {
            fs::create_dir(test_root)?;
            Ok(Self { test_root })
        }
    }
}
