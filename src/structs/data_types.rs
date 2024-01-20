use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct UnknownType {
}

#[derive(Serialize, Deserialize)]
pub struct ObjectType {
	#[serde(rename = "entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateTimeType {
}

#[derive(Serialize, Deserialize)]
pub struct BooleanType {
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationType {
	#[serde(rename = "enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringType {
}

#[derive(Serialize, Deserialize)]
pub struct VoidType {
}

#[derive(Serialize, Deserialize)]
pub struct IntegerType {
}

#[derive(Serialize, Deserialize)]
pub struct BinaryType {
}

#[derive(Serialize, Deserialize)]
pub struct ListType {
	#[serde(rename = "entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct DecimalType {
}

