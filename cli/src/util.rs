// std
use std::time::SystemTime;
// crates.io
use chrono::{DateTime, Utc};
use serde::Serializer;

pub fn ser_system_time<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let date = <DateTime<Utc>>::from(*time);

	serializer.serialize_str(&date.to_rfc3339())
}

pub fn ser_size_mb<S>(size: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(&format!("{:.2} MB", *size as f64 / 1024. / 1024.))
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
		r#"{"time":"2021-08-01T00:00:00+00:00","size":"1.00 MB"}"#
	)
}
