use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Translation {
	#[serde(rename = "LanguageCode")]
	language_code: String,
	#[serde(rename = "Text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemTextCollection {
	#[serde(rename = "SystemTexts")]
	system_texts: Vec<Texts$SystemText>,
}

#[derive(Serialize, Deserialize)]
pub struct SystemText {
	#[serde(rename = "InternalKey")]
	internal_key: String,
	#[serde(rename = "Text")]
	text: Texts$Text,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
	#[serde(rename = "CheckCompleteness")]
	check_completeness: bool,
	#[serde(rename = "Code")]
	code: String,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "CustomDateTimeFormat")]
	custom_date_time_format: String,
	#[serde(rename = "CustomTimeFormat")]
	custom_time_format: String,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
	#[serde(rename = "Items")]
	items: Vec<Texts$Translation>,
}

