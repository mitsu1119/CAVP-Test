use crate::error::CavpError;

type Result<T> = std::result::Result<T, CavpError>;

#[derive(Debug)]
pub struct ShaTriData {
    pub bit_len: u32,
    pub msg: String,
    md: String,
}

impl ShaTriData {
    pub fn new(bit_len: u32, msg: String, md: String) -> Self {
        Self { bit_len, msg, md }
    }

    pub fn test(&self, md: String) -> Result<()> {
        if md == self.md {
            Ok(())
        } else {
            Err(CavpError::TestFailed(self.msg.clone()))
        }
    }

    pub fn as_bytes(&self) -> ShaTriDataBytes {
        let bytes_msg = (0..self.msg.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self.msg[i..i + 2], 16).unwrap())
            .collect();

        let bytes_md = (0..self.md.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self.md[i..i + 2], 16).unwrap())
            .collect();

        ShaTriDataBytes::new(self.bit_len, bytes_msg, bytes_md)
    }
}

#[derive(Debug)]
pub struct ShaTriDataBytes {
    pub bit_len: u32,
    pub msg: Vec<u8>,
    md: Vec<u8>,
}

impl ShaTriDataBytes {
    fn new(bit_len: u32, msg: Vec<u8>, md: Vec<u8>) -> Self {
        Self { bit_len, msg, md }
    }

    pub fn test(&self, md: Vec<u8>) -> Result<()> {
        if md == self.md {
            Ok(())
        } else {
            Err(CavpError::TestFailed(format!("{:x?}", self.msg)))
        }
    }
}
