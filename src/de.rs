use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Deserializer};

/// Deserialize Unix timestamp to a [`DateTime<Local>`][chrono::DateTime]
pub fn deserialize_timestamp<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<DateTime<Local>, D::Error> {
    let timestamp: i64 = Deserialize::deserialize(de)?;
    Ok(Local
        .timestamp_opt(timestamp, 0)
        .latest()
        .ok_or(serde::de::Error::custom("failed to resolve unix timestamp"))?)
}
