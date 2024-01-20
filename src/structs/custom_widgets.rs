use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct WidgetReturnType {
	#[serde(rename = "AssignableTo")]
	assignable_to: String,
	#[serde(rename = "EntityProperty")]
	entity_property: String,
	#[serde(rename = "IsList")]
	is_list: bool,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetProperty {
	#[serde(rename = "TypePointer")]
	type_pointer: Binary,
	#[serde(rename = "Value")]
	value: CustomWidgets$WidgetValue,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValue {
	#[serde(rename = "Action")]
	action: Forms$NoAction,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Null,
	#[serde(rename = "DataSource")]
	data_source: Null,
	#[serde(rename = "EntityRef")]
	entity_ref: Null,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "Objects")]
	objects: Vec<2, []>,
	#[serde(rename = "PrimitiveValue")]
	primitive_value: String,
	#[serde(rename = "Selection")]
	selection: String,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
	#[serde(rename = "TextTemplate")]
	text_template: Null,
	#[serde(rename = "TranslatableValue")]
	translatable_value: Null,
	#[serde(rename = "TypePointer")]
	type_pointer: Binary,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, []>,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetEnumerationValue {
	#[serde(rename = "_Key")]
	key: String,
	#[serde(rename = "Caption")]
	caption: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetObjectType {
	#[serde(rename = "PropertyTypes")]
	property_types: Vec<CustomWidgets$WidgetPropertyType>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidgetType {
	#[serde(rename = "HelpUrl")]
	help_url: String,
	#[serde(rename = "ObjectType")]
	object_type: CustomWidgets$WidgetObjectType,
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
pub struct WidgetPropertyType {
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
	value_type: CustomWidgets$WidgetValueType,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetTranslation {
	#[serde(rename = "LanguageCode")]
	language_code: String,
	#[serde(rename = "Text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidgetDatabaseSource {
	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<2, []>,
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValueType {
	#[serde(rename = "AllowedTypes")]
	allowed_types: Vec<1, []>,
	#[serde(rename = "AllowNonPersistableEntities")]
	allow_non_persistable_entities: bool,
	#[serde(rename = "AssociationTypes")]
	association_types: Vec<1, []>,
	#[serde(rename = "DataSourceProperty")]
	data_source_property: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "EntityProperty")]
	entity_property: String,
	#[serde(rename = "EnumerationValues")]
	enumeration_values: Vec<2, []>,
	#[serde(rename = "IsList")]
	is_list: bool,
	#[serde(rename = "IsPath")]
	is_path: String,
	#[serde(rename = "Multiline")]
	multiline: bool,
	#[serde(rename = "ObjectType")]
	object_type: Null,
	#[serde(rename = "OnChangeProperty")]
	on_change_property: String,
	#[serde(rename = "ParameterIsList")]
	parameter_is_list: bool,
	#[serde(rename = "PathType")]
	path_type: String,
	#[serde(rename = "Required")]
	required: bool,
	#[serde(rename = "ReturnType")]
	return_type: Null,
	#[serde(rename = "SelectableObjectsProperty")]
	selectable_objects_property: String,
	#[serde(rename = "SelectionTypes")]
	selection_types: Vec<1, []>,
	#[serde(rename = "Translations")]
	translations: Vec<2, []>,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomWidget {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Object")]
	object: CustomWidgets$WidgetObject,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Type")]
	type: CustomWidgets$CustomWidgetType,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetObject {
	#[serde(rename = "Properties")]
	properties: Vec<CustomWidgets$WidgetProperty>,
	#[serde(rename = "TypePointer")]
	type_pointer: Binary,
}

