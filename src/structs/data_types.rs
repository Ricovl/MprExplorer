use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum DataType {
	#[serde(rename = "DataTypes$BinaryType")]
	BinaryType(BinaryType),
	#[serde(rename = "DataTypes$BooleanType")]
	BooleanType(BooleanType),
	#[serde(rename = "DataTypes$DateTimeType")]
	DateTimeType(DateTimeType),
	#[serde(rename = "DataTypes$DecimalType")]
	DecimalType(DecimalType),
	#[serde(rename = "DataTypes$EnumerationType")]
	EnumerationType(EnumerationType),
	#[serde(rename = "DataTypes$IntegerType")]
	IntegerType(IntegerType),
	#[serde(rename = "DataTypes$ListType")]
	ListType(ListType),
	#[serde(rename = "DataTypes$ObjectType")]
	ObjectType(ObjectType),
	#[serde(rename = "DataTypes$StringType")]
	StringType(StringType),
	#[serde(rename = "DataTypes$UnknownType")]
	UnknownType(UnknownType),
	#[serde(rename = "DataTypes$VoidType")]
	VoidType(VoidType),
}

#[derive(Serialize, Deserialize)]
pub struct BinaryType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct BooleanType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct DateTimeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct DecimalType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct EnumerationType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
pub struct IntegerType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct ListType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	pub entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	pub entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct UnknownType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct VoidType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

