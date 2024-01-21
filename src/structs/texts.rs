use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Language {
	#[serde(rename = "check_completeness")]
	check_completeness: bool,
	#[serde(rename = "code")]
	code: String,
	#[serde(rename = "custom_date_format")]
	custom_date_format: String,
	#[serde(rename = "custom_date_time_format")]
	custom_date_time_format: String,
	#[serde(rename = "custom_time_format")]
	custom_time_format: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemText {
	#[serde(rename = "internal_key")]
	internal_key: String,
	#[serde(rename = "text")]
	text: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct SystemTextCollection {
	#[serde(rename = "system_texts")]
	system_texts: Vec<texts::SystemText>,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
	#[serde(rename = "items")]
	items: Vec<texts::Translation>,
}

#[derive(Serialize, Deserialize)]
pub struct Translation {
	#[serde(rename = "language_code")]
	language_code: String,
	#[serde(rename = "text")]
	text: String,
}

