#[derive(Serialize, Deserialize)]
struct Translation {
	#[serde(rename = "LanguageCode")]
	language_code: String,
	#[serde(rename = "Text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
struct Text {
	#[serde(rename = "Items")]
	items: Vec<3, ["Texts$Translation"]>,
}

#[derive(Serialize, Deserialize)]
struct SystemText {
	#[serde(rename = "InternalKey")]
	internal_key: String,
	#[serde(rename = "Text")]
	text: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct Language {
	#[serde(rename = "CheckCompleteness")]
	check_completeness: Bool,
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
struct SystemTextCollection {
	#[serde(rename = "SystemTexts")]
	system_texts: Vec<2, ["Texts$SystemText"]>,
}

