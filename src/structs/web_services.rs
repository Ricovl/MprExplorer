use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct DataAssociationImpl {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "child_members")]
	child_members: Vec<web_services::DataAssociationImpl, web_services::DataAttributeImpl, >,
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "element_name")]
	element_name: String,
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "exposed_association_name")]
	exposed_association_name: String,
	#[serde(rename = "is_key")]
	is_key: bool,
	#[serde(rename = "is_nillable")]
	is_nillable: bool,
	#[serde(rename = "is_optional")]
	is_optional: bool,
	#[serde(rename = "object_element_name")]
	object_element_name: String,
	#[serde(rename = "summary")]
	summary: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataAttributeImpl {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "element_name")]
	element_name: String,
	#[serde(rename = "enumeration_as_string")]
	enumeration_as_string: bool,
	#[serde(rename = "filterable")]
	filterable: bool,
	#[serde(rename = "is_key")]
	is_key: bool,
	#[serde(rename = "is_nillable")]
	is_nillable: bool,
	#[serde(rename = "is_optional")]
	is_optional: bool,
	#[serde(rename = "sortable")]
	sortable: bool,
	#[serde(rename = "summary")]
	summary: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataEntityImpl {
	#[serde(rename = "child_members")]
	child_members: Vec<web_services::DataAttributeImpl, web_services::DataAssociationImpl, >,
	#[serde(rename = "element_name")]
	element_name: String,
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "is_key")]
	is_key: bool,
	#[serde(rename = "is_nillable")]
	is_nillable: bool,
	#[serde(rename = "is_optional")]
	is_optional: bool,
	#[serde(rename = "object_element_name")]
	object_element_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportedServiceImpl {
	#[serde(rename = "description")]
	description: web_services::WsdlDescriptionImpl,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "use_mtom")]
	use_mtom: bool,
	#[serde(rename = "wsdl_url")]
	wsdl_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct OperationInfoImpl {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "request_body_element_name")]
	request_body_element_name: String,
	#[serde(rename = "request_body_encoded")]
	request_body_encoded: bool,
	#[serde(rename = "request_body_part_encodings")]
	request_body_part_encodings: Vec<web_services::PartEncodingImpl, >,
	#[serde(rename = "request_body_rpc_element")]
	request_body_rpc_element: web_services::RpcOperationElementImpl,
	#[serde(rename = "request_header_element_name")]
	request_header_element_name: String,
	#[serde(rename = "request_header_encoded")]
	request_header_encoded: bool,
	#[serde(rename = "request_header_part_encoding")]
	request_header_part_encoding: web_services::PartEncodingImpl,
	#[serde(rename = "request_header_rpc_element")]
	request_header_rpc_element: web_services::RpcOperationElementImpl,
	#[serde(rename = "response_body_element_name")]
	response_body_element_name: String,
	#[serde(rename = "response_body_rpc_element")]
	response_body_rpc_element: web_services::RpcOperationElementImpl,
	#[serde(rename = "soap_action")]
	soap_action: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartEncodingImpl {
	#[serde(rename = "part_name")]
	part_name: String,
	#[serde(rename = "part_xsd_type")]
	part_xsd_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublishedOperationImpl {
	#[serde(rename = "data_entity")]
	data_entity: web_services::DataEntityImpl,
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "image_id")]
	image_id: String,
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "operation_return_type")]
	operation_return_type: data_types::ObjectType,
	#[serde(rename = "parameters")]
	parameters: Vec<UnknownType>,
	#[serde(rename = "return_element_name")]
	return_element_name: String,
	#[serde(rename = "return_type_is_nillable")]
	return_type_is_nillable: bool,
	#[serde(rename = "return_type_is_optional")]
	return_type_is_optional: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PublishedService {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "versioned_web_services")]
	versioned_web_services: Vec<web_services::VersionedServiceImpl, >,
}

#[derive(Serialize, Deserialize)]
pub struct RpcOperationElementImpl {
	#[serde(rename = "message_part_elements")]
	message_part_elements: Vec<UnknownType>,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceInfoImpl {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "location")]
	location: String,
	#[serde(rename = "location_constant")]
	location_constant: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "operations")]
	operations: Vec<web_services::OperationInfoImpl, >,
	#[serde(rename = "port_name")]
	port_name: String,
	#[serde(rename = "soap_version")]
	soap_version: String,
	#[serde(rename = "using_addressing")]
	using_addressing: bool,
}

#[derive(Serialize, Deserialize)]
pub struct VersionedServiceImpl {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "header_authentication")]
	header_authentication: String,
	#[serde(rename = "header_microflow")]
	header_microflow: String,
	#[serde(rename = "image")]
	image: String,
	#[serde(rename = "import_mapping")]
	import_mapping: String,
	#[serde(rename = "object_handling_backup")]
	object_handling_backup: String,
	#[serde(rename = "operations")]
	operations: Vec<web_services::PublishedOperationImpl, >,
	#[serde(rename = "optimized_xml")]
	optimized_xml: bool,
	#[serde(rename = "target_namespace")]
	target_namespace: String,
	#[serde(rename = "validate")]
	validate: bool,
	#[serde(rename = "version_number")]
	version_number: i64,
}

#[derive(Serialize, Deserialize)]
pub struct WsdlDescriptionImpl {
	#[serde(rename = "imports_have_locations")]
	imports_have_locations: bool,
	#[serde(rename = "schema_contentss")]
	schema_contentss: Vec<xml_schemas::XmlSchemaContents, >,
	#[serde(rename = "services")]
	services: Vec<web_services::ServiceInfoImpl, >,
	#[serde(rename = "target_namespace")]
	target_namespace: String,
	#[serde(rename = "wsdl_contentss")]
	wsdl_contentss: Vec<web_services::WsdlEntryImpl, >,
}

#[derive(Serialize, Deserialize)]
pub struct WsdlEntryImpl {
	#[serde(rename = "contents")]
	contents: String,
	#[serde(rename = "localized_contents_format")]
	localized_contents_format: String,
	#[serde(rename = "localized_location_format")]
	localized_location_format: String,
	#[serde(rename = "location")]
	location: String,
}

