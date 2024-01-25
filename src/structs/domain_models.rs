use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct AccessRule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowCreate")]
	allow_create: bool,
	#[serde(rename = "AllowDelete")]
	allow_delete: bool,
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "DefaultMemberAccessRights")]
	default_member_access_rights: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "MemberAccesses")]
	member_accesses: Vec<domain_models::MemberAccess, >,
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
	child_pointer: Binary,
	#[serde(rename = "DeleteBehavior")]
	delete_behavior: domain_models::DeleteBehavior,
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
	source: Empty,
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
	guid: Binary,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NewType")]
	new_type: domain_models::StringAttributeType,
	#[serde(rename = "Value")]
	value: domain_models::StoredValue,
}

#[derive(Serialize, Deserialize)]
pub struct AttributeRef {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "EntityRef")]
	entity_ref: Empty,
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
	delete_behavior: domain_models::DeleteBehavior,
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
	source: Empty,
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
	child_error_message: Empty,
	#[serde(rename = "ParentDeleteBehavior")]
	parent_delete_behavior: String,
	#[serde(rename = "ParentErrorMessage")]
	parent_error_message: Empty,
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

	#[serde(rename = "Annotations")]
	annotations: Vec<domain_models::Annotation, >,
	#[serde(rename = "Associations")]
	associations: Vec<domain_models::Association, >,
	#[serde(rename = "CrossAssociations")]
	cross_associations: Vec<domain_models::CrossAssociation, >,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Entities")]
	entities: Vec<domain_models::EntityImpl, >,
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

	#[serde(rename = "AccessRules")]
	access_rules: Vec<domain_models::AccessRule, >,
	#[serde(rename = "Attributes")]
	attributes: Vec<domain_models::Attribute, >,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Events")]
	events: Vec<domain_models::EntityEvent, >,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "GUID")]
	guid: Binary,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "ImageData")]
	image_data: Binary,
	#[serde(rename = "Indexes")]
	indexes: Vec<domain_models::EntityIndex, >,
	#[serde(rename = "Location")]
	location: String,
	#[serde(rename = "MaybeGeneralization")]
	maybe_generalization: domain_models::NoGeneralization,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Source")]
	source: Empty,
	#[serde(rename = "ValidationRules")]
	validation_rules: Vec<domain_models::ValidationRule, >,
}

#[derive(Serialize, Deserialize)]
pub struct EntityIndex {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attributes")]
	attributes: Vec<domain_models::IndexedAttribute, >,
	#[serde(rename = "GUID")]
	guid: Binary,
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
	attribute_pointer: Binary,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndirectEntityRef {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Steps")]
	steps: Vec<domain_models::EntityRefStep, >,
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
	message: texts::Text,
	#[serde(rename = "RuleInfo")]
	rule_info: domain_models::UniqueRuleInfo,
}

