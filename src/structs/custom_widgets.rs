use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct WidgetObjectType {
	#[serde(rename = "property_types")]
	property_types: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetTranslation {
	#[serde(rename = "language_code")]
	language_code: String,
	#[serde(rename = "text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetPropertyType {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "category")]
	category: String,
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "is_default")]
	is_default: bool,
	#[serde(rename = "property_key")]
	property_key: String,
	#[serde(rename = "value_type")]
	value_type: custom_widgets::WidgetValueType,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetReturnType {
	#[serde(rename = "assignable_to")]
	assignable_to: String,
	#[serde(rename = "entity_property")]
	entity_property: String,
	#[serde(rename = "is_list")]
	is_list: bool,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidgetDatabaseSource {
	#[serde(rename = "database_constraints")]
	database_constraints: Vec<>,
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValueType {
	#[serde(rename = "allowed_types")]
	allowed_types: Vec<>,
	#[serde(rename = "allow_non_persistable_entities")]
	allow_non_persistable_entities: bool,
	#[serde(rename = "association_types")]
	association_types: Vec<>,
	#[serde(rename = "data_source_property")]
	data_source_property: String,
	#[serde(rename = "default_value")]
	default_value: String,
	#[serde(rename = "entity_property")]
	entity_property: String,
	#[serde(rename = "enumeration_values")]
	enumeration_values: Vec<>,
	#[serde(rename = "is_list")]
	is_list: bool,
	#[serde(rename = "is_path")]
	is_path: String,
	#[serde(rename = "multiline")]
	multiline: bool,
	#[serde(rename = "object_type")]
	object_type: Null,
	#[serde(rename = "on_change_property")]
	on_change_property: String,
	#[serde(rename = "parameter_is_list")]
	parameter_is_list: bool,
	#[serde(rename = "path_type")]
	path_type: String,
	#[serde(rename = "required")]
	required: bool,
	#[serde(rename = "return_type")]
	return_type: Null,
	#[serde(rename = "selectable_objects_property")]
	selectable_objects_property: String,
	#[serde(rename = "selection_types")]
	selection_types: Vec<>,
	#[serde(rename = "translations")]
	translations: Vec<>,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetObject {
	#[serde(rename = "properties")]
	properties: Vec<>,
	#[serde(rename = "type_pointer")]
	type_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetEnumerationValue {
	#[serde(rename = "key")]
	key: String,
	#[serde(rename = "caption")]
	caption: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidget {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: Null,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "label_template")]
	label_template: Null,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "object")]
	object: custom_widgets::WidgetObject,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "type")]
	type: custom_widgets::CustomWidgetType,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValue {
	#[serde(rename = "action")]
	action: forms::NoAction,
	#[serde(rename = "attribute_ref")]
	attribute_ref: Null,
	#[serde(rename = "data_source")]
	data_source: Null,
	#[serde(rename = "entity_ref")]
	entity_ref: Null,
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "form")]
	form: String,
	#[serde(rename = "icon")]
	icon: Null,
	#[serde(rename = "image")]
	image: String,
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "nanoflow")]
	nanoflow: String,
	#[serde(rename = "objects")]
	objects: Vec<>,
	#[serde(rename = "primitive_value")]
	primitive_value: String,
	#[serde(rename = "selection")]
	selection: String,
	#[serde(rename = "source_variable")]
	source_variable: Null,
	#[serde(rename = "text_template")]
	text_template: Null,
	#[serde(rename = "translatable_value")]
	translatable_value: Null,
	#[serde(rename = "type_pointer")]
	type_pointer: Binary,
	#[serde(rename = "widgets")]
	widgets: Vec<>,
	#[serde(rename = "x_path_constraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidgetType {
	#[serde(rename = "help_url")]
	help_url: String,
	#[serde(rename = "object_type")]
	object_type: custom_widgets::WidgetObjectType,
	#[serde(rename = "offline_capable")]
	offline_capable: bool,
	#[serde(rename = "studio_category")]
	studio_category: String,
	#[serde(rename = "studio_pro_category")]
	studio_pro_category: String,
	#[serde(rename = "supported_platform")]
	supported_platform: String,
	#[serde(rename = "widget_description")]
	widget_description: String,
	#[serde(rename = "widget_id")]
	widget_id: String,
	#[serde(rename = "widget_name")]
	widget_name: String,
	#[serde(rename = "widget_needs_entity_context")]
	widget_needs_entity_context: bool,
	#[serde(rename = "widget_phone_gap_enabled")]
	widget_phone_gap_enabled: bool,
	#[serde(rename = "widget_plugin_widget")]
	widget_plugin_widget: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetProperty {
	#[serde(rename = "type_pointer")]
	type_pointer: Binary,
	#[serde(rename = "value")]
	value: custom_widgets::WidgetValue,
}

