#[derive(Serialize, Deserialize)]
struct DecimalType {
}

#[derive(Serialize, Deserialize)]
struct StringType {
}

#[derive(Serialize, Deserialize)]
struct MicroflowActionInfo {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Category")]
	category: String,
	#[serde(rename = "IconData")]
	icon_data: Binary,
	#[serde(rename = "IconDataDark")]
	icon_data_dark: Binary,
	#[serde(rename = "ImageData")]
	image_data: Binary,
	#[serde(rename = "ImageDataDark")]
	image_data_dark: Binary,
}

#[derive(Serialize, Deserialize)]
struct TypeParameter {
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct ParameterizedEntityType {
	#[serde(rename = "TypeParameterPointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
struct EntityTypeParameterType {
	#[serde(rename = "TypeParameterPointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
struct BooleanType {
}

#[derive(Serialize, Deserialize)]
struct IntegerType {
}

#[derive(Serialize, Deserialize)]
struct DateTimeType {
}

#[derive(Serialize, Deserialize)]
struct BasicParameterType {
	#[serde(rename = "Type")]
	type: CodeActions$StringType,
}

#[derive(Serialize, Deserialize)]
struct StringTemplateParameterType {
	#[serde(rename = "Grammar")]
	grammar: String,
}

#[derive(Serialize, Deserialize)]
struct ConcreteEntityType {
	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
struct VoidType {
}

#[derive(Serialize, Deserialize)]
struct ListType {
	#[serde(rename = "Parameter")]
	parameter: CodeActions$ConcreteEntityType,
}

#[derive(Serialize, Deserialize)]
struct EnumerationType {
	#[serde(rename = "Enumeration")]
	enumeration: String,
}

