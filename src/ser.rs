use chrono::{DateTime, Local};
use serde::Serializer;

pub fn serialize_timestamp<'de, S: Serializer>(
    date_time: &DateTime<Local>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    ser.serialize_i64(date_time.timestamp())
}
