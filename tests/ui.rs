#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/01-parse.rs");
    t.compile_fail("tests/build/02-enum.rs");
    t.compile_fail("tests/build/03-unsupported.rs");
}
