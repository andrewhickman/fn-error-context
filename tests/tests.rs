#[test]
fn tests() {
    let tests = trybuild::TestCases::new();

    tests.pass("tests/simple.rs");
    tests.pass("tests/arg.rs");
    tests.pass("tests/arg_pattern.rs");
    tests.pass("tests/inherent.rs");
    tests.compile_fail("tests/missing_fmt.rs");
    tests.pass("tests/fmt_args.rs");
    tests.pass("tests/failure.rs");
}
