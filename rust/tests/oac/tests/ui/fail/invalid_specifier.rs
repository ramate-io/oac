use cite::cite;
use oac::oac;

/// TODO: fill in tests with invalid specifiers

/// Test the oac source with citation footnote
#[oac(odemo, 1)]
pub fn test_oac_source() {
	println!("There is no odemo-000-000-001");
}

fn main() {
	test_oac_source();
}
