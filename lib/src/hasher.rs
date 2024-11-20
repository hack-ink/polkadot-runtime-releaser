//! Polkadot Runtime Releaser hasher component.

// crates.io
use blake2::{digest::consts::U32, Blake2b, Digest as _};
use md5::Md5;

/// Hashes the given bytes using the Blake2 256-bit algorithm.
pub fn blake2_256<B>(bytes: B) -> [u8; 32]
where
	B: AsRef<[u8]>,
{
	let bytes = bytes.as_ref();
	let mut hasher = <Blake2b<U32>>::new();

	hasher.update(bytes);

	let res = hasher.finalize();

	res.into()
}

/// Hashes the given bytes using the MD5 algorithm.
pub fn md5<B>(bytes: B) -> [u8; 16]
where
	B: AsRef<[u8]>,
{
	let bytes = bytes.as_ref();
	let mut hasher = Md5::new();

	hasher.update(bytes);

	let res = hasher.finalize();

	res.into()
}
