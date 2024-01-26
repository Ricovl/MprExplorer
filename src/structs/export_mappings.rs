use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum ExportMappingElement {
	#[serde(rename = "ExportMappings$ObjectMappingElement")]
	ObjectMappingElement(export_mappings::ObjectMappingElement),
	#[serde(rename = "ExportMappings$ValueMappingElement")]
	ValueMappingElement(export_mappings::ValueMappingElement),
}

#[derive(Serialize, Deserialize)]
pub struct ExportMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Elements")]
	elements: Vec<export_mappings::ObjectMappingElement>,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "IsHeaderParameter")]
	is_header_parameter: bool,
	#[serde(rename = "JsonStructure")]
	json_structure: String,
	#[serde(rename = "MessageDefinition")]
	message_definition: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NullValueOption")]
	null_value_option: String,
	#[serde(rename = "OperationName")]
	operation_name: String,
	#[serde(rename = "ParameterName")]
	parameter_name: String,
	#[serde(rename = "PublicName")]
	public_name: String,
	#[serde(rename = "ServiceName")]
	service_name: String,
	#[serde(rename = "WsdlFile")]
	wsdl_file: String,
	#[serde(rename = "XmlSchema")]
	xml_schema: String,
	#[serde(rename = "XsdRootElementName")]
	xsd_root_element_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectMappingElement {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Children")]
	children: Vec<ExportMappingElement>,
	#[serde(rename = "CustomHandlerCall")]
	custom_handler_call: Empty,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ExposedName")]
	exposed_name: String,
	#[serde(rename = "IsDefaultType")]
	is_default_type: bool,
	#[serde(rename = "JsonPath")]
	json_path: String,
	#[serde(rename = "MaxOccurs")]
	max_occurs: i64,
	#[serde(rename = "MinOccurs")]
	min_occurs: i64,
	#[serde(rename = "Nillable")]
	nillable: bool,
	#[serde(rename = "ObjectHandling")]
	object_handling: String,
	#[serde(rename = "ObjectHandlingBackup")]
	object_handling_backup: String,
	#[serde(rename = "ObjectHandlingBackupAllowOverride")]
	object_handling_backup_allow_override: bool,
	#[serde(rename = "XmlPath")]
	xml_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValueMappingElement {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Converter")]
	converter: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "ExposedName")]
	exposed_name: String,
	#[serde(rename = "FractionDigits")]
	fraction_digits: i64,
	#[serde(rename = "IsContent")]
	is_content: bool,
	#[serde(rename = "IsKey")]
	is_key: bool,
	#[serde(rename = "IsXmlAttribute")]
	is_xml_attribute: bool,
	#[serde(rename = "JsonPath")]
	json_path: String,
	#[serde(rename = "MaxLength")]
	max_length: i64,
	#[serde(rename = "MaxOccurs")]
	max_occurs: i64,
	#[serde(rename = "MinOccurs")]
	min_occurs: i64,
	#[serde(rename = "Nillable")]
	nillable: bool,
	#[serde(rename = "TotalDigits")]
	total_digits: i64,
	#[serde(rename = "Type")]
	var_type: data_types::DataType,
	#[serde(rename = "XmlPath")]
	xml_path: String,
	#[serde(rename = "XmlPrimitiveType")]
	xml_primitive_type: String,
}

