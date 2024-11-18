use cavp_test::CavpTest;

mod cavp_test;

fn main() {
    let test = CavpTest::new("test_dir");
    println!("{:?}", test);
}
