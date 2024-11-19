use cavp_test::CavpTest;
use error::CavpError;

mod cavp_test;
mod error;

fn main() -> Result<(), CavpError> {
    let test = CavpTest::new("test_dir")?;

    Ok(())
}
