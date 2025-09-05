use cite::cite;
use oac::oac;

/// TODO: fill in tests with invalid artifacts

/// Test the oac source with citation footnote
#[oac(omope, 1)]
pub fn test_oac_source() {
	println!("This function has a citation with an invalid artifact");
}

fn main() {
	test_oac_source();
}
