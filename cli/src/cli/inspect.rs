// std
use std::{fs, path::PathBuf, time::SystemTime};
// crates.io
use clap::Parser;
use serde::Serialize;
// self
use crate::{cli::Run, prelude::*};
use prr_lib::{hasher, runtime::Version, wasmer::Wasmer};

#[derive(Debug, Parser)]
pub struct InspectCmd {
	/// Path to the WASM runtime.
	#[arg(value_name = "PATH", verbatim_doc_comment)]
	path: PathBuf,
	/// Whether to check the runtime version in the `ParachainSystem::authorized_upgrade` call.
	#[arg(long, default_value_t = true, verbatim_doc_comment)]
	check_version: bool,
	/// Whether to print verbose output.
	#[arg(short, long, verbatim_doc_comment)]
	verbose: bool,
}
impl Run for InspectCmd {
	fn run(self) -> Result<()> {
		let Self { path, check_version, verbose } = self;
		let wasmer = Wasmer::load(&path)?;
		let built_at = fs::metadata(&path)?.created()?;
		let compressed = wasmer.compressed()?.len();
		let uncompressed = wasmer.decompressed()?.len();
		let size = Size { compressed, uncompressed };
		let md5 = hasher::md5(&wasmer.code);
		let sha256 = hasher::sha256(&wasmer.code);
		let blake2_256 = hasher::blake2_256(&wasmer.code);
		let ipfs = ipfs_cid::generate_cid_v0(&wasmer.code)?;
		let hash = Hash { md5, sha256, blake2_256, ipfs };
		let runtime = wasmer.runtime_version(verbose)?;
		let metadata = wasmer.metadata()?.version();
		let version = Ver { runtime, metadata };
		let call_hash = CallHash::of(&wasmer, check_version);
		let json = serde_json::to_string(&Output { built_at, size, hash, version, call_hash })?;

		println!("{json}");

		Ok(())
	}
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Output {
	#[serde(serialize_with = "util::ser_system_time")]
	built_at: SystemTime,
	size: Size,
	hash: Hash,
	version: Ver,
	call_hash: CallHash,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Size {
	#[serde(serialize_with = "util::ser_size_mb")]
	compressed: usize,
	#[serde(serialize_with = "util::ser_size_mb")]
	uncompressed: usize,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Hash {
	#[serde(serialize_with = "array_bytes::ser_hex")]
	md5: [u8; 16],
	#[serde(serialize_with = "array_bytes::ser_hex")]
	sha256: [u8; 32],
	#[serde(serialize_with = "array_bytes::ser_hex")]
	blake2_256: [u8; 32],
	ipfs: String,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Ver {
	runtime: Version,
	metadata: u32,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct CallHash {
	#[serde(serialize_with = "array_bytes::ser_hex")]
	set_code: [u8; 32],
	#[serde(serialize_with = "array_bytes::ser_hex")]
	authorized_upgrade: [u8; 32],
}
impl CallHash {
	fn of(wasmer: &Wasmer, check_version: bool) -> Self {
		let set_code = hasher::blake2_256(
			[
				// `System::set_code`.
				[0x00, 0x01].as_slice(),
				[0x00, 0x02].as_slice(),
				&hasher::blake2_256(&wasmer.code),
			]
			.concat(),
		);
		let authorized_upgrade = hasher::blake2_256(
			[
				// `ParachainSystem::authorized_upgrade`.
				[0x01, 0x02].as_slice(),
				&hasher::blake2_256(&wasmer.code),
				&[check_version as _],
			]
			.concat(),
		);

		Self { set_code, authorized_upgrade }
	}
}
