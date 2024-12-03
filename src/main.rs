use std::io::stdin;

use cavp_test::CavpTest;
use error::CavpError;

mod cavp_test;
mod error;
mod sha_data;

#[tokio::main]
async fn main() -> Result<(), CavpError> {
    let test = CavpTest::new("test_dir")?;
    test.download(cavp_test::TestKind::SHA).await?;

    println!("SHA-1 byte test");
    for t in test.sha1_byte_testvectors()? {
        println!("Msg: {}", t.msg);

        let mut md = String::new();
        stdin().read_line(&mut md)?;

        t.test(md.trim().to_string())?;
    }

    Ok(())
}
