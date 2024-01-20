#[derive(Serialize, Deserialize)]
struct DecimalType {
}

#[derive(Serialize, Deserialize)]
struct BooleanType {
}

#[derive(Serialize, Deserialize)]
struct BinaryType {
}

#[derive(Serialize, Deserialize)]
struct IntegerType {
}

#[derive(Serialize, Deserialize)]
struct ListType {
	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
struct VoidType {
}

#[derive(Serialize, Deserialize)]
struct EnumerationType {
	#[serde(rename = "Enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
struct UnknownType {
}

#[derive(Serialize, Deserialize)]
struct ObjectType {
	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
struct DateTimeType {
}

#[derive(Serialize, Deserialize)]
struct StringType {
}

