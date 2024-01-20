use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct AccessRule {
	#[serde(rename = "allow_create")]
	allow_create: bool,
	#[serde(rename = "allow_delete")]
	allow_delete: bool,
	#[serde(rename = "allowed_module_roles")]
	allowed_module_roles: Vec<>,
	#[serde(rename = "default_member_access_rights")]
	default_member_access_rights: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "member_accesses")]
	member_accesses: Vec<>,
	#[serde(rename = "x_path_constraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct Annotation {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "location")]
	location: String,
	#[serde(rename = "width")]
	width: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Association {
	#[serde(rename = "child_connection")]
	child_connection: String,
	#[serde(rename = "child_pointer")]
	child_pointer: Binary,
	#[serde(rename = "delete_behavior")]
	delete_behavior: domain_models::DeleteBehavior,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "guid")]
	guid: Binary,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "owner")]
	owner: String,
	#[serde(rename = "parent_connection")]
	parent_connection: String,
	#[serde(rename = "parent_pointer")]
	parent_pointer: Binary,
	#[serde(rename = "source")]
	source: Null,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "guid")]
	guid: Binary,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "new_type")]
	new_type: domain_models::IntegerAttributeType,
	#[serde(rename = "value")]
	value: domain_models::StoredValue,
}

#[derive(Serialize, Deserialize)]
pub struct AttributeRef {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "entity_ref")]
	entity_ref: Null,
}

#[derive(Serialize, Deserialize)]
pub struct AutoNumberAttributeType {
}

#[derive(Serialize, Deserialize)]
pub struct BooleanAttributeType {
}

#[derive(Serialize, Deserialize)]
pub struct CalculatedValue {
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "pass_entity")]
	pass_entity: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CrossAssociation {
	#[serde(rename = "child")]
	child: String,
	#[serde(rename = "delete_behavior")]
	delete_behavior: domain_models::DeleteBehavior,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "guid")]
	guid: Binary,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "owner")]
	owner: String,
	#[serde(rename = "parent_pointer")]
	parent_pointer: Binary,
	#[serde(rename = "source")]
	source: Null,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateTimeAttributeType {
	#[serde(rename = "localize_date")]
	localize_date: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DecimalAttributeType {
}

#[derive(Serialize, Deserialize)]
pub struct DeleteBehavior {
	#[serde(rename = "child_delete_behavior")]
	child_delete_behavior: String,
	#[serde(rename = "child_error_message")]
	child_error_message: Null,
	#[serde(rename = "parent_delete_behavior")]
	parent_delete_behavior: String,
	#[serde(rename = "parent_error_message")]
	parent_error_message: Null,
}

#[derive(Serialize, Deserialize)]
pub struct DirectEntityRef {
	#[serde(rename = "entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct DomainModel {
	#[serde(rename = "annotations")]
	annotations: Vec<>,
	#[serde(rename = "associations")]
	associations: Vec<>,
	#[serde(rename = "cross_associations")]
	cross_associations: Vec<>,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "entities")]
	entities: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityEvent {
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "moment")]
	moment: String,
	#[serde(rename = "raise_error_on_false")]
	raise_error_on_false: bool,
	#[serde(rename = "send_input_parameter")]
	send_input_parameter: bool,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct EntityImpl {
	#[serde(rename = "access_rules")]
	access_rules: Vec<>,
	#[serde(rename = "attributes")]
	attributes: Vec<>,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "events")]
	events: Vec<>,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "guid")]
	guid: Binary,
	#[serde(rename = "image")]
	image: String,
	#[serde(rename = "image_data")]
	image_data: Binary,
	#[serde(rename = "indexes")]
	indexes: Vec<>,
	#[serde(rename = "location")]
	location: String,
	#[serde(rename = "maybe_generalization")]
	maybe_generalization: domain_models::NoGeneralization,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "source")]
	source: Null,
	#[serde(rename = "validation_rules")]
	validation_rules: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityIndex {
	#[serde(rename = "attributes")]
	attributes: Vec<>,
	#[serde(rename = "guid")]
	guid: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct EntityRefStep {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "destination_entity")]
	destination_entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationAttributeType {
	#[serde(rename = "enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
pub struct Generalization {
	#[serde(rename = "generalization")]
	generalization: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndexedAttribute {
	#[serde(rename = "ascending")]
	ascending: bool,
	#[serde(rename = "attribute_pointer")]
	attribute_pointer: Binary,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndirectEntityRef {
	#[serde(rename = "steps")]
	steps: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct IntegerAttributeType {
}

#[derive(Serialize, Deserialize)]
pub struct LongAttributeType {
}

#[derive(Serialize, Deserialize)]
pub struct MemberAccess {
	#[serde(rename = "access_rights")]
	access_rights: String,
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "attribute")]
	attribute: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoGeneralization {
	#[serde(rename = "has_changed_by_attr")]
	has_changed_by_attr: bool,
	#[serde(rename = "has_changed_date_attr")]
	has_changed_date_attr: bool,
	#[serde(rename = "has_created_date_attr")]
	has_created_date_attr: bool,
	#[serde(rename = "has_owner_attr")]
	has_owner_attr: bool,
	#[serde(rename = "persistable")]
	persistable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RangeRuleInfo {
	#[serde(rename = "max_attribute")]
	max_attribute: String,
	#[serde(rename = "max_value")]
	max_value: String,
	#[serde(rename = "min_attribute")]
	min_attribute: String,
	#[serde(rename = "min_value")]
	min_value: String,
	#[serde(rename = "type_of_range")]
	type_of_range: String,
	#[serde(rename = "use_max_value")]
	use_max_value: bool,
	#[serde(rename = "use_min_value")]
	use_min_value: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RegExRuleInfo {
	#[serde(rename = "reg_ex_identifier")]
	reg_ex_identifier: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequiredRuleInfo {
}

#[derive(Serialize, Deserialize)]
pub struct StoredValue {
	#[serde(rename = "default_value")]
	default_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringAttributeType {
	#[serde(rename = "length")]
	length: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UniqueRuleInfo {
}

#[derive(Serialize, Deserialize)]
pub struct ValidationRule {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "message")]
	message: texts::Text,
	#[serde(rename = "rule_info")]
	rule_info: domain_models::RequiredRuleInfo,
}

