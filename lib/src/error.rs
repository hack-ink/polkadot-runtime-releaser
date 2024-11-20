//! Polkadot Runtime Releaser error collection.

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	ParseInt(#[from] std::num::ParseIntError),

	#[error(transparent)]
	Reqwew(#[from] reqwew::error::Error),

	#[error("unsupported tag style, {0}")]
	UnsupportedTagStyle(String),
}
