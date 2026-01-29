#[test]
fn test_builder_macro() {
    let t = trybuild::TestCases::new();

    t.pass("tests/builder/builder_ok.rs");
    t.compile_fail("tests/builder/builder_fail.rs");
}
