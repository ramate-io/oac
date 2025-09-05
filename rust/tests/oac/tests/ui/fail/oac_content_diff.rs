use cite::cite;
use oac::oac;

/// Test the oac source with citation footnote
#[oac(odemo, 0)]
#[cite(above, reason = "Testing oac source 1")]
#[oac(oroad, 5)]
#[cite(
	above,
	reason = "Testing oac source 2",
	referenced_revision = "8186abec357fcc37a78fc57056c38057a58692d1"
)]
pub fn test_oac_source() {
	println!("This function has a citation with a git source");
}

fn main() {
	test_oac_source();
}
