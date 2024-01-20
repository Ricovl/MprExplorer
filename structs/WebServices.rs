#[derive(Serialize, Deserialize)]
struct OperationInfoImpl {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RequestBodyElementName")]
	request_body_element_name: String,
	#[serde(rename = "RequestBodyEncoded")]
	request_body_encoded: Bool,
	#[serde(rename = "RequestBodyPartEncodings")]
	request_body_part_encodings: Vec<2, ["WebServices$PartEncodingImpl"]>,
	#[serde(rename = "RequestBodyRpcElement")]
	request_body_rpc_element: WebServices$RpcOperationElementImpl,
	#[serde(rename = "RequestHeaderElementName")]
	request_header_element_name: String,
	#[serde(rename = "RequestHeaderEncoded")]
	request_header_encoded: Bool,
	#[serde(rename = "RequestHeaderPartEncoding")]
	request_header_part_encoding: WebServices$PartEncodingImpl,
	#[serde(rename = "RequestHeaderRpcElement")]
	request_header_rpc_element: WebServices$RpcOperationElementImpl,
	#[serde(rename = "ResponseBodyElementName")]
	response_body_element_name: String,
	#[serde(rename = "ResponseBodyRpcElement")]
	response_body_rpc_element: WebServices$RpcOperationElementImpl,
	#[serde(rename = "SoapAction")]
	soap_action: String,
}

#[derive(Serialize, Deserialize)]
struct PublishedService {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "VersionedWebServices")]
	versioned_web_services: Vec<2, ["WebServices$VersionedServiceImpl"]>,
}

#[derive(Serialize, Deserialize)]
struct DataAttributeImpl {
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "ElementName")]
	element_name: String,
	#[serde(rename = "EnumerationAsString")]
	enumeration_as_string: Bool,
	#[serde(rename = "Filterable")]
	filterable: Bool,
	#[serde(rename = "IsKey")]
	is_key: Bool,
	#[serde(rename = "IsNillable")]
	is_nillable: Bool,
	#[serde(rename = "IsOptional")]
	is_optional: Bool,
	#[serde(rename = "Sortable")]
	sortable: Bool,
	#[serde(rename = "Summary")]
	summary: String,
}

#[derive(Serialize, Deserialize)]
struct ServiceInfoImpl {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Location")]
	location: String,
	#[serde(rename = "LocationConstant")]
	location_constant: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operations")]
	operations: Vec<2, ["WebServices$OperationInfoImpl"]>,
	#[serde(rename = "PortName")]
	port_name: String,
	#[serde(rename = "SoapVersion")]
	soap_version: String,
	#[serde(rename = "UsingAddressing")]
	using_addressing: Bool,
}

#[derive(Serialize, Deserialize)]
struct PublishedOperationImpl {
	#[serde(rename = "DataEntity")]
	data_entity: WebServices$DataEntityImpl,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ImageId")]
	image_id: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OperationReturnType")]
	operation_return_type: DataTypes$ObjectType,
	#[serde(rename = "Parameters")]
	parameters: Vec<2, []>,
	#[serde(rename = "ReturnElementName")]
	return_element_name: String,
	#[serde(rename = "ReturnTypeIsNillable")]
	return_type_is_nillable: Bool,
	#[serde(rename = "ReturnTypeIsOptional")]
	return_type_is_optional: Bool,
}

#[derive(Serialize, Deserialize)]
struct RpcOperationElementImpl {
	#[serde(rename = "MessagePartElements")]
	message_part_elements: Vec<2, []>,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct VersionedServiceImpl {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "HeaderAuthentication")]
	header_authentication: String,
	#[serde(rename = "HeaderMicroflow")]
	header_microflow: String,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "ImportMapping")]
	import_mapping: String,
	#[serde(rename = "ObjectHandlingBackup")]
	object_handling_backup: String,
	#[serde(rename = "Operations")]
	operations: Vec<3, ["WebServices$PublishedOperationImpl"]>,
	#[serde(rename = "OptimizedXml")]
	optimized_xml: Bool,
	#[serde(rename = "TargetNamespace")]
	target_namespace: String,
	#[serde(rename = "Validate")]
	validate: Bool,
	#[serde(rename = "VersionNumber")]
	version_number: i64,
}

#[derive(Serialize, Deserialize)]
struct WsdlDescriptionImpl {
	#[serde(rename = "ImportsHaveLocations")]
	imports_have_locations: Bool,
	#[serde(rename = "SchemaContentss")]
	schema_contentss: Vec<2, ["XmlSchemas$XmlSchemaContents"]>,
	#[serde(rename = "Services")]
	services: Vec<2, ["WebServices$ServiceInfoImpl"]>,
	#[serde(rename = "TargetNamespace")]
	target_namespace: String,
	#[serde(rename = "WsdlContentss")]
	wsdl_contentss: Vec<2, ["WebServices$WsdlEntryImpl"]>,
}

#[derive(Serialize, Deserialize)]
struct DataAssociationImpl {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "ChildMembers")]
	child_members: Vec<2, ["WebServices$DataAttributeImpl"]>,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "ElementName")]
	element_name: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ExposedAssociationName")]
	exposed_association_name: String,
	#[serde(rename = "IsKey")]
	is_key: Bool,
	#[serde(rename = "IsNillable")]
	is_nillable: Bool,
	#[serde(rename = "IsOptional")]
	is_optional: Bool,
	#[serde(rename = "ObjectElementName")]
	object_element_name: String,
	#[serde(rename = "Summary")]
	summary: String,
}

#[derive(Serialize, Deserialize)]
struct PartEncodingImpl {
	#[serde(rename = "PartName")]
	part_name: String,
	#[serde(rename = "PartXsdType")]
	part_xsd_type: String,
}

#[derive(Serialize, Deserialize)]
struct WsdlEntryImpl {
	#[serde(rename = "Contents")]
	contents: String,
	#[serde(rename = "LocalizedContentsFormat")]
	localized_contents_format: String,
	#[serde(rename = "LocalizedLocationFormat")]
	localized_location_format: String,
	#[serde(rename = "Location")]
	location: String,
}

#[derive(Serialize, Deserialize)]
struct DataEntityImpl {
	#[serde(rename = "ChildMembers")]
	child_members: Vec<2, ["WebServices$DataAttributeImpl", "WebServices$DataAssociationImpl"]>,
	#[serde(rename = "ElementName")]
	element_name: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "IsKey")]
	is_key: Bool,
	#[serde(rename = "IsNillable")]
	is_nillable: Bool,
	#[serde(rename = "IsOptional")]
	is_optional: Bool,
	#[serde(rename = "ObjectElementName")]
	object_element_name: String,
}

#[derive(Serialize, Deserialize)]
struct ImportedServiceImpl {
	#[serde(rename = "Description")]
	description: WebServices$WsdlDescriptionImpl,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "UseMtom")]
	use_mtom: Bool,
	#[serde(rename = "WsdlUrl")]
	wsdl_url: String,
}

