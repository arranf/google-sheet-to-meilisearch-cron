use std::fmt::Debug;

use serde::Deserialize;

use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry { pub id: String, pub name: String, pub edition: Option<String>, pub format: Option<String>, pub system: Option<String>, pub r#type: Option<String>, pub pdf: bool, pub physical: bool }