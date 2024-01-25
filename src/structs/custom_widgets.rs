use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CustomWidget {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Object")]
	object: custom_widgets::WidgetObject,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Type")]
	var_type: custom_widgets::CustomWidgetType,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidgetDatabaseSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidgetType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "HelpUrl")]
	help_url: String,
	#[serde(rename = "ObjectType")]
	object_type: custom_widgets::WidgetObjectType,
	#[serde(rename = "OfflineCapable")]
	offline_capable: bool,
	#[serde(rename = "StudioCategory")]
	studio_category: String,
	#[serde(rename = "StudioProCategory")]
	studio_pro_category: String,
	#[serde(rename = "SupportedPlatform")]
	supported_platform: String,
	#[serde(rename = "WidgetDescription")]
	widget_description: String,
	#[serde(rename = "WidgetId")]
	widget_id: String,
	#[serde(rename = "WidgetName")]
	widget_name: String,
	#[serde(rename = "WidgetNeedsEntityContext")]
	widget_needs_entity_context: bool,
	#[serde(rename = "WidgetPhoneGapEnabled")]
	widget_phone_gap_enabled: bool,
	#[serde(rename = "WidgetPluginWidget")]
	widget_plugin_widget: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetEnumerationValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "_Key")]
	key: String,
	#[serde(rename = "Caption")]
	caption: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetObject {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Properties")]
	properties: Vec<custom_widgets::WidgetProperty, >,
	#[serde(rename = "TypePointer")]
	type_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetObjectType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "PropertyTypes")]
	property_types: Vec<custom_widgets::WidgetPropertyType, >,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetProperty {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "TypePointer")]
	type_pointer: Binary,
	#[serde(rename = "Value")]
	value: custom_widgets::WidgetValue,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetPropertyType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Category")]
	category: String,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "IsDefault")]
	is_default: bool,
	#[serde(rename = "PropertyKey")]
	property_key: String,
	#[serde(rename = "ValueType")]
	value_type: custom_widgets::WidgetValueType,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetReturnType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AssignableTo")]
	assignable_to: String,
	#[serde(rename = "EntityProperty")]
	entity_property: String,
	#[serde(rename = "IsList")]
	is_list: bool,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetTranslation {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "LanguageCode")]
	language_code: String,
	#[serde(rename = "Text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	action: forms::NoAction,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Empty,
	#[serde(rename = "DataSource")]
	data_source: Empty,
	#[serde(rename = "EntityRef")]
	entity_ref: Empty,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "Objects")]
	objects: Vec<custom_widgets::WidgetObject, >,
	#[serde(rename = "PrimitiveValue")]
	primitive_value: String,
	#[serde(rename = "Selection")]
	selection: String,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
	#[serde(rename = "TextTemplate")]
	text_template: Empty,
	#[serde(rename = "TranslatableValue")]
	translatable_value: Empty,
	#[serde(rename = "TypePointer")]
	type_pointer: Binary,
	#[serde(rename = "Widgets")]
	widgets: Vec<custom_widgets::CustomWidget, forms::LayoutGrid, forms::DivContainer, >,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValueType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowedTypes")]
	allowed_types: Vec<String>,
	#[serde(rename = "AllowNonPersistableEntities")]
	allow_non_persistable_entities: bool,
	#[serde(rename = "AssociationTypes")]
	association_types: Vec<UnknownType>,
	#[serde(rename = "DataSourceProperty")]
	data_source_property: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "EntityProperty")]
	entity_property: String,
	#[serde(rename = "EnumerationValues")]
	enumeration_values: Vec<custom_widgets::WidgetEnumerationValue, >,
	#[serde(rename = "IsList")]
	is_list: bool,
	#[serde(rename = "IsPath")]
	is_path: String,
	#[serde(rename = "Multiline")]
	multiline: bool,
	#[serde(rename = "ObjectType")]
	object_type: Empty,
	#[serde(rename = "OnChangeProperty")]
	on_change_property: String,
	#[serde(rename = "ParameterIsList")]
	parameter_is_list: bool,
	#[serde(rename = "PathType")]
	path_type: String,
	#[serde(rename = "Required")]
	required: bool,
	#[serde(rename = "ReturnType")]
	return_type: Empty,
	#[serde(rename = "SelectableObjectsProperty")]
	selectable_objects_property: String,
	#[serde(rename = "SelectionTypes")]
	selection_types: Vec<String>,
	#[serde(rename = "Translations")]
	translations: Vec<custom_widgets::WidgetTranslation, >,
	#[serde(rename = "Type")]
	var_type: String,
}

