// std
use std::path::PathBuf;
// crates.io
use clap::Parser;
// self
use crate::{cli::Run, prelude::*};
use polkadot_runtime_releaser_lib::{hasher, wasmer::Wasmer};

#[derive(Debug, Parser)]
pub struct InspectCommand {
	/// Path to the WASM runtime.
	#[arg(value_name = "PATH")]
	path: PathBuf,
	/// Whether to check the runtime version in the `parachainSystem::authorized_upgrade` call.
	#[arg(long, default_value_t = true)]
	check_version: bool,
	/// Whether to print verbose output.
	#[arg(short, long)]
	verbose: bool,
}
impl Run for InspectCommand {
	fn run(self) -> Result<()> {
		let Self { path, check_version, verbose } = self;
		let wasmer = Wasmer::load(&path)?;
		let blake2_256 = array_bytes::bytes2hex("0x", hasher::blake2_256(&wasmer.code));
		let md5 = array_bytes::bytes2hex("0x", hasher::md5(&wasmer.code));
		let compressed_size = util::format_size_mb(wasmer.compressed_size()?);
		let depressed_size = util::format_size_mb(wasmer.decompressed_size()?);
		let ver = {
			let mut ver = serde_json::to_value(wasmer.runtime_version()?)?;

			if !verbose {
				if let Some(ver) = ver.as_object_mut() {
					ver.remove("apis");
				}
			}

			ver
		};
		let set_code_call_hash = array_bytes::bytes2hex(
			"0x",
			hasher::blake2_256(
				[[0x00, 0x02].as_slice(), &hasher::blake2_256(&wasmer.code)].concat(),
			),
		);
		let authorized_upgrade_call_hash = array_bytes::bytes2hex(
			"0x",
			hasher::blake2_256(
				[[0x01, 0x02].as_slice(), &hasher::blake2_256(&wasmer.code), &[check_version as _]]
					.concat(),
			),
		);
		let json = serde_json::json!({
			"blake2-256": blake2_256,
			"md5": md5,
			"compressedSize": compressed_size,
			"decompressedSize": depressed_size,
			"version": ver,
			"setCodeCallHash": set_code_call_hash,
			"authorizedUpgradeCallHash": authorized_upgrade_call_hash,
		});

		println!("{json}");

		Ok(())
	}
}
