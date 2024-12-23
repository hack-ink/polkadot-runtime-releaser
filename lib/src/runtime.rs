//! Polkadot Runtime Releaser runtime component.

// crates.io
use serde::Serialize;
use sp_version::RuntimeVersion;
// self
use crate::prelude::*;

#[allow(missing_docs)]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Version {
	pub spec_name: String,
	pub impl_name: String,
	pub authoring_version: u32,
	pub spec_version: u32,
	pub impl_version: u32,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub apis: Vec<String>,
	pub transaction_version: u32,
	pub state_version: u8,
}
impl Version {
	/// Load the version from [`sp_version::RuntimeVersion`].
	pub fn load(ver: RuntimeVersion, simplify: bool) -> Result<Self> {
		let ver = Self {
			spec_name: ver.spec_name.to_string(),
			impl_name: ver.impl_name.to_string(),
			authoring_version: ver.authoring_version,
			spec_version: ver.spec_version,
			impl_version: ver.impl_version,
			apis: if simplify {
				serde_json::to_value(ver.apis)?
					.as_array()
					.map(|apis| {
						apis.iter().filter_map(|api| api.as_str().map(|s| s.to_owned())).collect()
					})
					.unwrap_or_default()
			} else {
				Vec::new()
			},
			transaction_version: ver.transaction_version,
			state_version: ver.state_version,
		};

		Ok(ver)
	}
}

#[test]
fn load_should_work() {
	// crates.io
	use parity_scale_codec::Decode;

	let ver = RuntimeVersion::decode(
		&mut [
			32, 112, 111, 108, 107, 97, 100, 111, 116, 60, 112, 97, 114, 105, 116, 121, 45, 112,
			111, 108, 107, 97, 100, 111, 116, 0, 0, 0, 0, 252, 77, 15, 0, 0, 0, 0, 0, 92, 197, 31,
			241, 250, 63, 93, 12, 202, 1, 0, 0, 0, 223, 106, 203, 104, 153, 7, 96, 155, 5, 0, 0, 0,
			55, 227, 151, 252, 124, 145, 245, 228, 2, 0, 0, 0, 64, 254, 58, 212, 1, 248, 149, 154,
			6, 0, 0, 0, 23, 166, 188, 13, 0, 98, 174, 179, 1, 0, 0, 0, 24, 239, 88, 163, 182, 123,
			167, 112, 1, 0, 0, 0, 210, 188, 152, 151, 238, 208, 143, 21, 3, 0, 0, 0, 247, 139, 39,
			139, 229, 63, 69, 76, 2, 0, 0, 0, 175, 44, 2, 151, 162, 62, 109, 61, 11, 0, 0, 0, 73,
			234, 175, 27, 84, 138, 12, 176, 3, 0, 0, 0, 145, 213, 223, 24, 176, 210, 207, 88, 2, 0,
			0, 0, 42, 94, 146, 70, 85, 57, 158, 96, 1, 0, 0, 0, 237, 153, 197, 172, 178, 94, 237,
			245, 3, 0, 0, 0, 203, 202, 37, 227, 159, 20, 35, 135, 2, 0, 0, 0, 104, 122, 212, 74,
			211, 127, 3, 194, 1, 0, 0, 0, 171, 60, 5, 114, 41, 31, 235, 139, 1, 0, 0, 0, 188, 157,
			137, 144, 79, 91, 146, 63, 1, 0, 0, 0, 55, 200, 187, 19, 80, 169, 162, 168, 4, 0, 0, 0,
			243, 255, 20, 213, 171, 82, 112, 89, 3, 0, 0, 0, 111, 245, 46, 232, 88, 230, 197, 189,
			1, 0, 0, 0, 145, 177, 200, 177, 99, 40, 235, 146, 1, 0, 0, 0, 159, 251, 80, 90, 167,
			56, 214, 156, 1, 0, 0, 0, 251, 197, 119, 185, 215, 71, 239, 214, 1, 0, 0, 0, 26, 0, 0,
			0, 1,
		]
		.as_slice(),
	)
	.unwrap();

	assert_eq!(
		Version::load(ver, true).unwrap(),
		Version {
			spec_name: "polkadot".to_owned(),
			impl_name: "parity-polkadot".to_owned(),
			authoring_version: 0,
			spec_version: 1003004,
			impl_version: 0,
			apis: Vec::new(),
			transaction_version: 26,
			state_version: 1,
		}
	)
}
