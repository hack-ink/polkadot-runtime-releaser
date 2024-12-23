//! Polkadot Runtime Releaser hasher component.

// crates.io
use blake2::{digest::consts::U32, Blake2b, Digest as _};
use md5::Md5;
use sha2::Sha256;

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

/// Hashes the given bytes using the SHA-256 algorithm.
pub fn sha256<B>(bytes: B) -> [u8; 32]
where
	B: AsRef<[u8]>,
{
	let bytes = bytes.as_ref();
	let mut hasher = Sha256::new();

	hasher.update(bytes);

	let res = hasher.finalize();

	res.into()
}

#[test]
fn hasher_should_work() {
	assert_eq!(
		blake2_256(b"Polkadot Runtime Releaser"),
		[
			234, 178, 61, 198, 178, 59, 248, 116, 129, 239, 62, 194, 176, 128, 195, 232, 135, 24,
			218, 190, 207, 111, 67, 191, 41, 219, 174, 94, 210, 205, 120, 142
		]
	);
	assert_eq!(
		md5(b"Polkadot Runtime Releaser"),
		[19, 255, 133, 129, 21, 247, 16, 140, 45, 65, 168, 163, 105, 49, 38, 183]
	);
	assert_eq!(
		sha256(b"Polkadot Runtime Releaser"),
		[
			11, 199, 244, 174, 102, 140, 114, 106, 198, 60, 127, 17, 127, 53, 77, 25, 205, 75, 130,
			141, 131, 168, 240, 76, 122, 68, 28, 227, 5, 175, 229, 51
		]
	);
}
