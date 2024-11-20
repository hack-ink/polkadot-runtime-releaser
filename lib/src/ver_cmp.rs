//! Polkadot Runtime Releaser version comparison component.

// crates.io
use serde::Deserialize;
// self
use crate::error::Error;

#[allow(missing_docs)]
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Tag style.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "value")]
pub enum TagStyle {
	/// Semantic version style.
	SemanticVersion(SemanticVersion),
	/// Tailing spec version style.
	TailingSpecVersion(TailingSpecVersion),
	// TODO: we can add more styles in the future.
}
impl TagStyle {
	/// Compare the given tag with the given spec version.
	pub fn eq(&self, tag: &str, spec_version: u32) -> Result<bool> {
		Ok(self.spec_version_of(tag)? == spec_version)
	}

	fn spec_version_of(&self, tag: &str) -> Result<u32> {
		let ver = match self {
			TagStyle::SemanticVersion(rule) => {
				let mut ver = tag.split_terminator('.');
				let major_str = ver
					.next()
					// Remove non-digit characters prefix.
					.map(|s| s.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
					.ok_or_else(|| Error::UnsupportedTagStyle(tag.into()))?;
				let major_offset = rule
					.major
					.checked_sub(major_str.len() as u32)
					.ok_or_else(|| Error::UnsupportedTagStyle(tag.into()))?;
				let minor_str = ver.next().ok_or_else(|| Error::UnsupportedTagStyle(tag.into()))?;
				let minor_offset = rule
					.minor
					.checked_sub(minor_str.len() as u32)
					.ok_or_else(|| Error::UnsupportedTagStyle(tag.into()))?;
				let patch_str = ver.next().ok_or_else(|| Error::UnsupportedTagStyle(tag.into()))?;
				let patch_offset = rule
					.patch
					.checked_sub(patch_str.len() as u32)
					.ok_or_else(|| Error::UnsupportedTagStyle(tag.into()))?;
				let major = major_str
					.chars()
					.filter(|c| c.is_ascii_digit())
					.collect::<String>()
					.parse::<u32>()
					.map_err(Error::ParseInt)?
					* 10_u32.pow(major_offset + rule.minor + rule.patch);
				let minor = minor_str.parse::<u32>().map_err(Error::ParseInt)?
					* 10_u32.pow(minor_offset + rule.patch);
				let patch = patch_str.parse::<u32>().map_err(Error::ParseInt)?
					* 10_u32.pow(patch_offset);

				major + minor + patch
			},
			TagStyle::TailingSpecVersion(rule) => tag
				.rsplit_once(&rule.separator)
				.and_then(|(_, s)| s.parse::<u32>().map_err(Error::ParseInt).ok())
				.ok_or_else(|| Error::UnsupportedTagStyle(tag.into()))?,
		};

		Ok(ver)
	}
}
/// Semantic version style.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SemanticVersion {
	/// The length of the major version.
	pub major: u32,
	/// The length of the minor version.
	pub minor: u32,
	/// The length of the patch version.
	pub patch: u32,
}
/// Tailing spec version style.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TailingSpecVersion {
	/// The separator between the context and the spec version.
	pub separator: String,
}

#[test]
fn tag_style_eq_should_work() {
	// https://github.com/polkadot-fellows/runtimes/releases/tag/v1.3.4.
	let polkadot_style =
		TagStyle::SemanticVersion(SemanticVersion { major: 3, minor: 3, patch: 1 });
	assert!(polkadot_style.eq("1.3.4", 1_003_004).unwrap());

	// https://github.com/AcalaNetwork/Acala/releases/tag/2.27.0.
	let acala_style = TagStyle::SemanticVersion(SemanticVersion { major: 1, minor: 2, patch: 1 });
	assert!(acala_style.eq("2.27.0", 2_270).unwrap());

	// https://github.com/darwinia-network/darwinia/releases/tag/v6.7.2.
	let darwinia_style =
		TagStyle::SemanticVersion(SemanticVersion { major: 1, minor: 1, patch: 2 });
	assert!(darwinia_style.eq("v6.7.2", 6_720).unwrap());

	// https://github.com/moonbeam-foundation/moonbeam/releases/tag/runtime-3300.
	let moonbeam_style = TagStyle::TailingSpecVersion(TailingSpecVersion { separator: "-".into() });
	assert!(moonbeam_style.eq("runtime-3300", 3_300).unwrap());
}
