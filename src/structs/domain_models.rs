use bson::Binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum AttributeType {
	#[serde(rename = "DomainModels$IntegerAttributeType")]
	IntegerAttributeType(IntegerAttributeType),
	#[serde(rename = "DomainModels$DecimalAttributeType")]
	DecimalAttributeType(DecimalAttributeType),
	#[serde(rename = "DomainModels$BooleanAttributeType")]
	BooleanAttributeType(BooleanAttributeType),
	#[serde(rename = "DomainModels$EnumerationAttributeType")]
	EnumerationAttributeType(EnumerationAttributeType),
	#[serde(rename = "DomainModels$DateTimeAttributeType")]
	DateTimeAttributeType(DateTimeAttributeType),
	#[serde(rename = "DomainModels$StringAttributeType")]
	StringAttributeType(StringAttributeType),
	#[serde(rename = "DomainModels$AutoNumberAttributeType")]
	AutoNumberAttributeType(AutoNumberAttributeType),
	#[serde(rename = "DomainModels$LongAttributeType")]
	LongAttributeType(LongAttributeType)
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum AttributeValue {
	#[serde(rename = "DomainModels$StoredValue")]
	StoredValue(StoredValue),
	#[serde(rename = "DomainModels$CalculatedValue")]
	CalculatedValue(CalculatedValue),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum MaybeGeneralization {
	#[serde(rename = "DomainModels$NoGeneralization")]
	NoGeneralization(NoGeneralization), 
	#[serde(rename = "DomainModels$Generalization")]
	Generalization(Generalization),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum RuleInfo {
	#[serde(rename = "DomainModels$UniqueRuleInfo")]
	UniqueRuleInfo(UniqueRuleInfo),
	#[serde(rename = "DomainModels$RequiredRuleInfo")]
	RequiredRuleInfo(RequiredRuleInfo),
	#[serde(rename = "DomainModels$RangeRuleInfo")]
	RangeRuleInfo(RangeRuleInfo),
	#[serde(rename = "DomainModels$RegExRuleInfo")]
	RegExRuleInfo(RegExRuleInfo),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum EntityRef {
	#[serde(rename = "DomainModels$IndirectEntityRef")]
	IndirectEntityRef(domain_models::IndirectEntityRef), 
	#[serde(rename = "DomainModels$DirectEntityRef")]
	DirectEntityRef(domain_models::DirectEntityRef),
}


#[derive(Serialize, Deserialize)]
pub struct AccessRule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowCreate")]
	allow_create: bool,
	#[serde(rename = "AllowDelete")]
	allow_delete: bool,
	#[serde(rename = "AllowedModuleRoles", deserialize_with = "deserialize_settings")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "DefaultMemberAccessRights")]
	default_member_access_rights: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "MemberAccesses", deserialize_with = "deserialize_settings")]
	member_accesses: Vec<domain_models::MemberAccess>,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct Annotation {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct Association {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ChildConnection")]
	child_connection: String,
	#[serde(rename = "ChildPointer")]
	child_pointer: Uuid,
	#[serde(rename = "DeleteBehavior")]
	delete_behavior: Option<domain_models::DeleteBehavior>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Uuid,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Owner")]
	owner: String,
	#[serde(rename = "ParentConnection")]
	parent_connection: String,
	#[serde(rename = "ParentPointer")]
	parent_pointer: Uuid,
	#[serde(rename = "Source")]
	source: Option<Empty>,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Uuid,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NewType")]
	new_type: AttributeType,
	#[serde(rename = "Value")]
	value: AttributeValue,
}

#[derive(Serialize, Deserialize)]
pub struct AttributeRef {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "EntityRef")]
	entity_ref: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoNumberAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct BooleanAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct CalculatedValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "PassEntity")]
	pass_entity: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CrossAssociation {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Child")]
	child: String,
	#[serde(rename = "DeleteBehavior")]
	delete_behavior: Option<domain_models::DeleteBehavior>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Uuid,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Owner")]
	owner: String,
	#[serde(rename = "ParentPointer")]
	parent_pointer: Uuid,
	#[serde(rename = "Source")]
	source: Option<Empty>,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateTimeAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "LocalizeDate")]
	localize_date: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DecimalAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct DeleteBehavior {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ChildDeleteBehavior")]
	child_delete_behavior: String,
	#[serde(rename = "ChildErrorMessage")]
	child_error_message: Option<Empty>,
	#[serde(rename = "ParentDeleteBehavior")]
	parent_delete_behavior: String,
	#[serde(rename = "ParentErrorMessage")]
	parent_error_message: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct DirectEntityRef {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct DomainModel {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Annotations", deserialize_with = "deserialize_settings")]
	annotations: Vec<domain_models::Annotation>,
	#[serde(rename = "Associations", deserialize_with = "deserialize_settings")]
	associations: Vec<domain_models::Association>,
	#[serde(rename = "CrossAssociations", deserialize_with = "deserialize_settings")]
	cross_associations: Vec<domain_models::CrossAssociation>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Entities", deserialize_with = "deserialize_settings")]
	entities: Vec<domain_models::EntityImpl>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityEvent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Moment")]
	moment: String,
	#[serde(rename = "RaiseErrorOnFalse")]
	raise_error_on_false: bool,
	#[serde(rename = "SendInputParameter")]
	send_input_parameter: bool,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct EntityImpl {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AccessRules", deserialize_with = "deserialize_settings")]
	access_rules: Vec<domain_models::AccessRule>,
	#[serde(rename = "Attributes", deserialize_with = "deserialize_settings")]
	attributes: Vec<domain_models::Attribute>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Events", deserialize_with = "deserialize_settings")]
	events: Vec<domain_models::EntityEvent>,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Uuid,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "ImageData")]
	image_data: Binary,
	#[serde(rename = "Indexes", deserialize_with = "deserialize_settings")]
	indexes: Vec<domain_models::EntityIndex>,
	#[serde(rename = "Location")]
	location: String,
	#[serde(rename = "MaybeGeneralization")]
	maybe_generalization: MaybeGeneralization,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Source")]
	source: Option<Empty>,
	#[serde(rename = "ValidationRules", deserialize_with = "deserialize_settings")]
	validation_rules: Vec<domain_models::ValidationRule>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityIndex {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attributes", deserialize_with = "deserialize_settings")]
	attributes: Vec<domain_models::IndexedAttribute>,
	#[serde(rename = "GUID")]
	guid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct EntityRefStep {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "DestinationEntity")]
	destination_entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
pub struct Generalization {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Generalization")]
	generalization: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndexedAttribute {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Ascending")]
	ascending: bool,
	#[serde(rename = "AttributePointer")]
	attribute_pointer: Uuid,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndirectEntityRef {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Steps", deserialize_with = "deserialize_settings")]
	steps: Vec<domain_models::EntityRefStep>,
}

#[derive(Serialize, Deserialize)]
pub struct IntegerAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct LongAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct MemberAccess {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AccessRights")]
	access_rights: String,
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoGeneralization {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "HasChangedByAttr")]
	has_changed_by_attr: bool,
	#[serde(rename = "HasChangedDateAttr")]
	has_changed_date_attr: bool,
	#[serde(rename = "HasCreatedDateAttr")]
	has_created_date_attr: bool,
	#[serde(rename = "HasOwnerAttr")]
	has_owner_attr: bool,
	#[serde(rename = "Persistable")]
	persistable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RangeRuleInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
	use_max_value: bool,
	#[serde(rename = "UseMinValue")]
	use_min_value: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RegExRuleInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "RegExIdentifier")]
	reg_ex_identifier: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequiredRuleInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct StoredValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DefaultValue")]
	default_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringAttributeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Length")]
	length: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UniqueRuleInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct ValidationRule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Message")]
	message: Option<texts::Text>,
	#[serde(rename = "RuleInfo")]
	rule_info: RuleInfo,
}

