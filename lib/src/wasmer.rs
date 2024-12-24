//! Polkadot Runtime Releaser WASM component.

// std
use std::{borrow::Cow, fs, path::Path};
// crates.io
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use parity_scale_codec::Decode;
use sc_executor::{RuntimeVersion, WasmExecutor};
use sc_executor_common::runtime_blob::RuntimeBlob;
use sp_core::traits::ReadRuntimeVersion;
use sp_maybe_compressed_blob::CODE_BLOB_BOMB_LIMIT;
use sp_state_machine::BasicExternalities;
// self
use crate::{prelude::*, runtime::Version};

/// WASMer.
pub struct Wasmer {
	/// WASM code bytes.
	pub code: Vec<u8>,
	executor: WasmExecutor,
}
impl Wasmer {
	/// Load WASM runtime from the given path.
	pub fn load<P>(path: P) -> Result<Self>
	where
		P: AsRef<Path>,
	{
		let code = fs::read(path)?;
		let executor = WasmExecutor::default();
		let wasmer = Self { code, executor };

		Ok(wasmer)
	}

	/// Compress the code.
	pub fn compressed(&self) -> Result<Vec<u8>> {
		let code = sp_maybe_compressed_blob::compress(&self.code, CODE_BLOB_BOMB_LIMIT)
			.ok_or(Error::OversizedCodeBlob(CODE_BLOB_BOMB_LIMIT))?;

		Ok(code)
	}

	/// Decompress the code.
	pub fn decompressed(&self) -> Result<Cow<[u8]>> {
		let code = sp_maybe_compressed_blob::decompress(&self.code, CODE_BLOB_BOMB_LIMIT)?;

		Ok(code)
	}

	/// Read the runtime version.
	pub fn runtime_version(&self, simplify: bool) -> Result<Version, Error> {
		let ver = self
			.executor
			.read_runtime_version(&self.code, &mut BasicExternalities::default())
			.map_err(Error::Custom)?;
		let ver = RuntimeVersion::decode(&mut &ver[..])?;
		let ver = Version::load(ver, simplify)?;

		Ok(ver)
	}

	/// Read the runtime metadata.
	pub fn metadata(&self) -> Result<RuntimeMetadata> {
		let metadata_compressed = self.executor.uncached_call(
			RuntimeBlob::uncompress_if_needed(&self.code)?,
			&mut BasicExternalities::default(),
			true,
			"Metadata_metadata",
			&[],
		)?;
		let metadata_encoded = <Vec<u8>>::decode(&mut &*metadata_compressed)?;
		let metadata = RuntimeMetadataPrefixed::decode(&mut &*metadata_encoded)?;

		Ok(metadata.1)
	}
}
