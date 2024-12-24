// std
use std::time::SystemTime;
// crates.io
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serializer;

pub fn ser_system_time<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let date = <DateTime<Utc>>::from(*time);

	serializer.serialize_str(&date.to_rfc3339_opts(SecondsFormat::Secs, true))
}

pub fn ser_size_mb<S>(size: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	fn format_with_commas(num: &usize) -> String {
		let s = num.to_string();

		s.as_bytes().rchunks(3).map(String::from_utf8_lossy).rev().collect::<Vec<_>>().join(",")
	}

	let mb = *size as f64 / (1024. * 1024.);
	let bytes = format_with_commas(size);

	serializer.serialize_str(&format!("{mb:.2} MB ({bytes} bytes)"))
}

#[test]
fn ser_should_work() {
	// std
	use std::time::Duration;
	// crates.io
	use serde::Serialize;

	#[derive(Serialize)]
	struct Test {
		#[serde(serialize_with = "ser_system_time")]
		time: SystemTime,
		#[serde(serialize_with = "ser_size_mb")]
		size: usize,
	}

	assert_eq!(
		serde_json::to_string(&Test {
			time: SystemTime::UNIX_EPOCH + Duration::from_secs(1627776000),
			size: 1024 * 1024
		})
		.unwrap(),
		r#"{"time":"2021-08-01T00:00:00Z","size":"1.00 MB (1,048,576 bytes)"}"#
	)
}
