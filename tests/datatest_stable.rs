// We want to share tests between "nightly" and "stable" suites. These have to be two different
// suites as we set `harness = false` for the "stable" one.
include!("tests/mod.rs");

datatest::harness!();

// Regular test have to use `datatest` variant of `#[test]` to work.
use datatest::test;

#[test]
fn regular_test() {
    println!("regular tests also work!");
}

#[test]
fn regular_test_result() -> Result<(), Box<dyn std::error::Error>> {
    println!("regular tests also work!");
    Ok(())
}
