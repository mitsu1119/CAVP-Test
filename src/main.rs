use cavp_test::CavpTest;
use error::CavpError;

mod cavp_test;
mod error;

#[tokio::main]
async fn main() -> Result<(), CavpError> {
    let test = CavpTest::new("test_dir")?;
    test.download(cavp_test::TestKind::SHA).await?;

    test.sha1()?;

    Ok(())
}
