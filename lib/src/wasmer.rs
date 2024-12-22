//! Polkadot Runtime Releaser WASM component.

// std
use std::{fs, path::Path};
// crates.io
use parity_scale_codec::Decode;
use sc_executor::WasmExecutor;
use sp_core::traits::ReadRuntimeVersion;
use sp_maybe_compressed_blob::CODE_BLOB_BOMB_LIMIT;
use sp_state_machine::BasicExternalities;
use sp_version::RuntimeVersion;
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

	/// Get the size of the compressed code.
	pub fn compressed_size(&self) -> Result<usize> {
		let code = sp_maybe_compressed_blob::compress(&self.code, CODE_BLOB_BOMB_LIMIT)
			.ok_or(Error::OversizedCodeBlob(CODE_BLOB_BOMB_LIMIT))?;

		Ok(code.len())
	}

	/// Get the size of the decompressed code.
	pub fn decompressed_size(&self) -> Result<usize> {
		let code = sp_maybe_compressed_blob::decompress(&self.code, CODE_BLOB_BOMB_LIMIT)?;

		Ok(code.len())
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
}
