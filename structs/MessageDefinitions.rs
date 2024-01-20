#[derive(Serialize, Deserialize)]
struct EntityMessageDefinition {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExposedEntity")]
	exposed_entity: MessageDefinitions$ExposedEntity,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct ExposedAttribute {
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Children")]
	children: Vec<2, []>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "Example")]
	example: String,
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
	#[serde(rename = "OriginalName")]
	original_name: String,
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
struct ExposedEntity {
	#[serde(rename = "Children")]
	children: Vec<2, ["MessageDefinitions$ExposedAttribute", "MessageDefinitions$ExposedAssociation"]>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "Example")]
	example: String,
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
	#[serde(rename = "OriginalName")]
	original_name: String,
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
struct MessageDefinitionCollection {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "MessageDefinitions")]
	message_definitions: Vec<2, ["MessageDefinitions$EntityMessageDefinition"]>,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct ExposedAssociation {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Children")]
	children: Vec<2, ["MessageDefinitions$ExposedAttribute"]>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "Example")]
	example: String,
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
	#[serde(rename = "OriginalName")]
	original_name: String,
	#[serde(rename = "Path")]
	path: String,
	#[serde(rename = "PrimitiveType")]
	primitive_type: String,
	#[serde(rename = "TotalDigits")]
	total_digits: i64,
	#[serde(rename = "WarningMessage")]
	warning_message: String,
}

