use cite::cite;
use oac::oac;

/// Test the git source with citation footnote
#[oac(odemo, 0)]
#[cite(above, reason = "Testing oac source 1")]
#[oac(oroad, 5)]
#[cite(above, reason = "Testing oac source 2")]
pub fn test_oac_source() {
	println!("This function has a citation with a git source");
}
