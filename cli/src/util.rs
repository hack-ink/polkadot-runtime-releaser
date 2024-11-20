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
