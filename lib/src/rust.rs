//! Polkadot Runtime Releaser Rust component.

// std
use std::{fs, path::Path};
// self
use crate::prelude::*;

const TOOLCHAIN: &str = r#"[toolchain]
channel    = "stable"
components = ["cargo", "clippy", "rust-src", "rustc", "rustfmt"]
profile    = "minimal"
targets    = ["wasm32-unknown-unknown"]"#;

/// Generate a rust-toolchain.toml file with the given version under the given workdir.
pub fn gen_toolchain_config<S, P>(version: Option<S>, workdir: P) -> Result<()>
where
	S: AsRef<str>,
	P: AsRef<Path>,
{
	let workdir = workdir.as_ref();
	let toml = workdir.join("rust-toolchain.toml");

	if toml.exists() || workdir.join(".rust-toolchain.toml").exists() {
		return Ok(());
	}

	tracing::info!("creating toolchain config {}", toml.display());

	let toolchain = version
		.map_or_else(|| TOOLCHAIN.into(), |ver| TOOLCHAIN.replacen("stable", ver.as_ref(), 1));

	fs::write(&toml, toolchain)?;

	Ok(())
}