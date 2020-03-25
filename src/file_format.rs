use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Consider writing a stateful serializer/deserializer so we don't have to buffer the whole JSON file in memory
// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// struct Space {
//     #[serde(flatten)]
//     pages: Vec<Page>,
// }

// use chrono::serde::ts_seconds::{
//     deserialize as from_ts,
//     serialize as to_ts,
// };

// pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
//     where D: serde::de::Deserializer<'de>
// {
//     Ok(d.deserialize_i64(SecondsTimestampVisitor)?)
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Page {
    pub title: String,
    #[serde(default)]
    pub children: Option<Vec<Item>>,
    #[serde(rename = "edit-time", default)]
    pub edit_time: Option<u64>, // DateTime<Utc>,
    #[serde(rename = "edit-email")]
    pub edit_email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Item {
    pub string: String,
    #[serde(rename = "create-email", default)]
    pub create_email: Option<String>,
    #[serde(rename = "create-time", default)] // deserialize_with = "from_ts",
    pub create_time: Option<u64>, // DateTime<Utc>>,
    pub uid: String,
    #[serde(rename = "edit-time", default)]
    pub edit_time: Option<u64>, // DateTime<Utc>,
    #[serde(rename = "edit-email")]
    pub edit_email: String,
    #[serde(default)]
    pub children: Option<Vec<Item>>,
}
