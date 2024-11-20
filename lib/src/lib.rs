#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

//! Polkadot Runtime Releaser library.

pub mod api;
pub mod docker;
pub mod error;
pub mod hasher;
pub mod system;
pub mod ver_cmp;
pub mod wasmer;

mod prelude {
	pub use crate::error::Error;

	pub type Result<T, E = Error> = std::result::Result<T, E>;
}
