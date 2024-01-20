#[derive(Serialize, Deserialize)]
struct JsonElement {
	#[serde(rename = "Children")]
	children: Vec<2, []>,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "ExposedItemName")]
	exposed_item_name: String,
	#[serde(rename = "ExposedName")]
	exposed_name: String,
	#[serde(rename = "FractionDigits")]
	fraction_digits: i64,
	#[serde(rename = "IsDefaultType")]
	is_default_type: Bool,
	#[serde(rename = "MaxLength")]
	max_length: i64,
	#[serde(rename = "MaxOccurs")]
	max_occurs: i64,
	#[serde(rename = "MinOccurs")]
	min_occurs: i64,
	#[serde(rename = "Nillable")]
	nillable: Bool,
	#[serde(rename = "OriginalValue")]
	original_value: String,
	#[serde(rename = "Path")]
	path: String,
	#[serde(rename = "PrimitiveType")]
	primitive_type: String,
	#[serde(rename = "TotalDigits")]
	total_digits: i64,
	#[serde(rename = "WarningMessage")]
	warning_message: String,
}

#[derive(Serialize, Deserialize)]
struct JsonStructure {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Elements")]
	elements: Vec<2, ["JsonStructures$JsonElement"]>,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "JsonSnippet")]
	json_snippet: String,
	#[serde(rename = "Name")]
	name: String,
}

