#[derive(Serialize, Deserialize)]
struct EntityRefStep {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "DestinationEntity")]
	destination_entity: String,
}

#[derive(Serialize, Deserialize)]
struct MemberAccess {
	#[serde(rename = "AccessRights")]
	access_rights: String,
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
}

#[derive(Serialize, Deserialize)]
struct StringAttributeType {
	#[serde(rename = "Length")]
	length: i64,
}

#[derive(Serialize, Deserialize)]
struct CalculatedValue {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "PassEntity")]
	pass_entity: Bool,
}

#[derive(Serialize, Deserialize)]
struct Association {
	#[serde(rename = "ChildConnection")]
	child_connection: String,
	#[serde(rename = "ChildPointer")]
	child_pointer: Binary,
	#[serde(rename = "DeleteBehavior")]
	delete_behavior: DomainModels$DeleteBehavior,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Binary,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Owner")]
	owner: String,
	#[serde(rename = "ParentConnection")]
	parent_connection: String,
	#[serde(rename = "ParentPointer")]
	parent_pointer: Binary,
	#[serde(rename = "Source")]
	source: Null,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct BooleanAttributeType {
}

#[derive(Serialize, Deserialize)]
struct CrossAssociation {
	#[serde(rename = "Child")]
	child: String,
	#[serde(rename = "DeleteBehavior")]
	delete_behavior: DomainModels$DeleteBehavior,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Binary,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Owner")]
	owner: String,
	#[serde(rename = "ParentPointer")]
	parent_pointer: Binary,
	#[serde(rename = "Source")]
	source: Null,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct StoredValue {
	#[serde(rename = "DefaultValue")]
	default_value: String,
}

#[derive(Serialize, Deserialize)]
struct RegExRuleInfo {
	#[serde(rename = "RegExIdentifier")]
	reg_ex_identifier: String,
}

#[derive(Serialize, Deserialize)]
struct RequiredRuleInfo {
}

#[derive(Serialize, Deserialize)]
struct EnumerationAttributeType {
	#[serde(rename = "Enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
struct Annotation {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Location")]
	location: String,
	#[serde(rename = "Width")]
	width: i64,
}

#[derive(Serialize, Deserialize)]
struct NoGeneralization {
	#[serde(rename = "HasChangedByAttr")]
	has_changed_by_attr: Bool,
	#[serde(rename = "HasChangedDateAttr")]
	has_changed_date_attr: Bool,
	#[serde(rename = "HasCreatedDateAttr")]
	has_created_date_attr: Bool,
	#[serde(rename = "HasOwnerAttr")]
	has_owner_attr: Bool,
	#[serde(rename = "Persistable")]
	persistable: Bool,
}

#[derive(Serialize, Deserialize)]
struct IndexedAttribute {
	#[serde(rename = "Ascending")]
	ascending: Bool,
	#[serde(rename = "AttributePointer")]
	attribute_pointer: Binary,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct DecimalAttributeType {
}

#[derive(Serialize, Deserialize)]
struct EntityImpl {
	#[serde(rename = "AccessRules")]
	access_rules: Vec<3, ["DomainModels$AccessRule"]>,
	#[serde(rename = "Attributes")]
	attributes: Vec<3, ["DomainModels$Attribute"]>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Events")]
	events: Vec<3, []>,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Binary,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "ImageData")]
	image_data: Binary,
	#[serde(rename = "Indexes")]
	indexes: Vec<3, []>,
	#[serde(rename = "Location")]
	location: String,
	#[serde(rename = "MaybeGeneralization")]
	maybe_generalization: DomainModels$NoGeneralization,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Source")]
	source: Null,
	#[serde(rename = "ValidationRules")]
	validation_rules: Vec<3, []>,
}

#[derive(Serialize, Deserialize)]
struct UniqueRuleInfo {
}

#[derive(Serialize, Deserialize)]
struct EntityIndex {
	#[serde(rename = "Attributes")]
	attributes: Vec<2, ["DomainModels$IndexedAttribute"]>,
	#[serde(rename = "GUID")]
	guid: Binary,
}

#[derive(Serialize, Deserialize)]
struct EntityEvent {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Moment")]
	moment: String,
	#[serde(rename = "RaiseErrorOnFalse")]
	raise_error_on_false: Bool,
	#[serde(rename = "SendInputParameter")]
	send_input_parameter: Bool,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct AttributeRef {
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "EntityRef")]
	entity_ref: Null,
}

#[derive(Serialize, Deserialize)]
struct RangeRuleInfo {
	#[serde(rename = "MaxAttribute")]
	max_attribute: String,
	#[serde(rename = "MaxValue")]
	max_value: String,
	#[serde(rename = "MinAttribute")]
	min_attribute: String,
	#[serde(rename = "MinValue")]
	min_value: String,
	#[serde(rename = "TypeOfRange")]
	type_of_range: String,
	#[serde(rename = "UseMaxValue")]
	use_max_value: Bool,
	#[serde(rename = "UseMinValue")]
	use_min_value: Bool,
}

#[derive(Serialize, Deserialize)]
struct DomainModel {
	#[serde(rename = "Annotations")]
	annotations: Vec<3, ["DomainModels$Annotation"]>,
	#[serde(rename = "Associations")]
	associations: Vec<3, ["DomainModels$Association"]>,
	#[serde(rename = "CrossAssociations")]
	cross_associations: Vec<3, []>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Entities")]
	entities: Vec<3, ["DomainModels$EntityImpl"]>,
}

#[derive(Serialize, Deserialize)]
struct ValidationRule {
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Message")]
	message: Texts$Text,
	#[serde(rename = "RuleInfo")]
	rule_info: DomainModels$UniqueRuleInfo,
}

#[derive(Serialize, Deserialize)]
struct DeleteBehavior {
	#[serde(rename = "ChildDeleteBehavior")]
	child_delete_behavior: String,
	#[serde(rename = "ChildErrorMessage")]
	child_error_message: Null,
	#[serde(rename = "ParentDeleteBehavior")]
	parent_delete_behavior: String,
	#[serde(rename = "ParentErrorMessage")]
	parent_error_message: Null,
}

#[derive(Serialize, Deserialize)]
struct IntegerAttributeType {
}

#[derive(Serialize, Deserialize)]
struct AccessRule {
	#[serde(rename = "AllowCreate")]
	allow_create: Bool,
	#[serde(rename = "AllowDelete")]
	allow_delete: Bool,
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<1, ["String"]>,
	#[serde(rename = "DefaultMemberAccessRights")]
	default_member_access_rights: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "MemberAccesses")]
	member_accesses: Vec<3, ["DomainModels$MemberAccess"]>,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
struct LongAttributeType {
}

#[derive(Serialize, Deserialize)]
struct DirectEntityRef {
	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
struct AutoNumberAttributeType {
}

#[derive(Serialize, Deserialize)]
struct DateTimeAttributeType {
	#[serde(rename = "LocalizeDate")]
	localize_date: Bool,
}

#[derive(Serialize, Deserialize)]
struct IndirectEntityRef {
	#[serde(rename = "Steps")]
	steps: Vec<2, ["DomainModels$EntityRefStep"]>,
}

#[derive(Serialize, Deserialize)]
struct Generalization {
	#[serde(rename = "Generalization")]
	generalization: String,
}

#[derive(Serialize, Deserialize)]
struct Attribute {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Binary,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NewType")]
	new_type: DomainModels$StringAttributeType,
	#[serde(rename = "Value")]
	value: DomainModels$StoredValue,
}

