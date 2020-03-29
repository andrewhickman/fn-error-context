#[test]
fn tests() {
    let tests = trybuild::TestCases::new();

    tests.pass("tests/simple.rs");
    tests.pass("tests/arg.rs");
    tests.pass("tests/arg_pattern.rs");
}
