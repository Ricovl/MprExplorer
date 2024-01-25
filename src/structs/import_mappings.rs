use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ImportMapping {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "elements")]
	elements: Vec<import_mappings::ObjectMappingElement, >,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "json_structure")]
	json_structure: String,
	#[serde(rename = "message_definition")]
	message_definition: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "operation_name")]
	operation_name: String,
	#[serde(rename = "parameter_type")]
	parameter_type: data_types::UnknownType,
	#[serde(rename = "public_name")]
	public_name: String,
	#[serde(rename = "service_name")]
	service_name: String,
	#[serde(rename = "use_subtransactions_for_microflows")]
	use_subtransactions_for_microflows: bool,
	#[serde(rename = "wsdl_file")]
	wsdl_file: String,
	#[serde(rename = "xml_schema")]
	xml_schema: String,
	#[serde(rename = "xsd_root_element_name")]
	xsd_root_element_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectMappingElement {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "children")]
	children: Vec<import_mappings::ValueMappingElement, import_mappings::ObjectMappingElement, >,
	#[serde(rename = "custom_handler_call")]
	custom_handler_call: Empty,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "element_type")]
	element_type: String,
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "exposed_name")]
	exposed_name: String,
	#[serde(rename = "is_default_type")]
	is_default_type: bool,
	#[serde(rename = "json_path")]
	json_path: String,
	#[serde(rename = "max_occurs")]
	max_occurs: i64,
	#[serde(rename = "min_occurs")]
	min_occurs: i64,
	#[serde(rename = "nillable")]
	nillable: bool,
	#[serde(rename = "object_handling")]
	object_handling: String,
	#[serde(rename = "object_handling_backup")]
	object_handling_backup: String,
	#[serde(rename = "object_handling_backup_allow_override")]
	object_handling_backup_allow_override: bool,
	#[serde(rename = "xml_path")]
	xml_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValueMappingElement {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "converter")]
	converter: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "element_type")]
	element_type: String,
	#[serde(rename = "exposed_name")]
	exposed_name: String,
	#[serde(rename = "fraction_digits")]
	fraction_digits: i64,
	#[serde(rename = "is_content")]
	is_content: bool,
	#[serde(rename = "is_key")]
	is_key: bool,
	#[serde(rename = "is_xml_attribute")]
	is_xml_attribute: bool,
	#[serde(rename = "json_path")]
	json_path: String,
	#[serde(rename = "max_length")]
	max_length: i64,
	#[serde(rename = "max_occurs")]
	max_occurs: i64,
	#[serde(rename = "min_occurs")]
	min_occurs: i64,
	#[serde(rename = "nillable")]
	nillable: bool,
	#[serde(rename = "total_digits")]
	total_digits: i64,
	#[serde(rename = "_type")]
	_type: data_types::StringType,
	#[serde(rename = "xml_path")]
	xml_path: String,
	#[serde(rename = "xml_primitive_type")]
	xml_primitive_type: String,
}

