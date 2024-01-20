use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ObjectMappingElement {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Children")]
	children: Vec<2, ["ImportMappings$ValueMappingElement", "ImportMappings$ObjectMappingElement"]>,
	#[serde(rename = "CustomHandlerCall")]
	custom_handler_call: Null,
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
	type: DataTypes$IntegerType,
	#[serde(rename = "XmlPath")]
	xml_path: String,
	#[serde(rename = "XmlPrimitiveType")]
	xml_primitive_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportMapping {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Elements")]
	elements: Vec<ImportMappings$ObjectMappingElement>,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "JsonStructure")]
	json_structure: String,
	#[serde(rename = "MessageDefinition")]
	message_definition: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OperationName")]
	operation_name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: DataTypes$UnknownType,
	#[serde(rename = "PublicName")]
	public_name: String,
	#[serde(rename = "ServiceName")]
	service_name: String,
	#[serde(rename = "UseSubtransactionsForMicroflows")]
	use_subtransactions_for_microflows: bool,
	#[serde(rename = "WsdlFile")]
	wsdl_file: String,
	#[serde(rename = "XmlSchema")]
	xml_schema: String,
	#[serde(rename = "XsdRootElementName")]
	xsd_root_element_name: String,
}

