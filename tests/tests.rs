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
    tests.pass("tests/non_copy_arg.rs");
    tests.pass("tests/non_copy_fmt_arg.rs");
    tests.pass("tests/as_ref.rs");
    tests.compile_fail("tests/fmt_missing_arg.rs");
    tests.compile_fail("tests/fmt_unused_arg.rs");
    tests.pass("tests/fmt_named_arg.rs");
}
