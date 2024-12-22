//! Polkadot Runtime Releaser runtime component.

// crates.io
use serde::Serialize;
use sp_version::RuntimeVersion;
// self
use crate::prelude::*;

#[allow(missing_docs)]
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
