//! Polkadot Runtime Releaser error collection.

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	ParseInt(#[from] std::num::ParseIntError),

	#[error(transparent)]
	Codec(#[from] parity_scale_codec::Error),
	#[error(transparent)]
	Compress(#[from] sp_maybe_compressed_blob::Error),
	#[error(transparent)]
	Executor(#[from] sc_executor::error::Error),
	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error),
	#[error(transparent)]
	Wasm(#[from] sc_executor_common::error::WasmError),

	#[error("{0}")]
	Custom(String),
	#[error("oversized code blob, limit is {0}")]
	OversizedCodeBlob(usize),
	#[error("unsupported tag style, {0}")]
	UnsupportedTagStyle(String),
}
