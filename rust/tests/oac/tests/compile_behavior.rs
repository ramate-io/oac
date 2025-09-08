// Integration tests for compile-time behavior of the cite macro

#[test]
fn test_compile_pass() {
	let t = trybuild::TestCases::new();
	t.pass("tests/ui/pass/*.rs");
}

#[test]
fn test_compile_fail() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui/fail/*.rs");
}

#[test]
fn test_oac_1_pass() {
	let t = trybuild::TestCases::new();
	t.pass("tests/ui/pass/oac_pass.rs");
}

#[test]
fn test_oac_content_diff() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui/fail/oac_content_diff.rs");
}
