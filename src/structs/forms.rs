use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ActionButton {
	#[serde(rename = "action")]
	action: forms::MicroflowAction,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "aria_role")]
	aria_role: String,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "icon")]
	icon: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "render_type")]
	render_type: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct Appearance {
	#[serde(rename = "class")]
	class: String,
	#[serde(rename = "design_properties")]
	design_properties: Vec<UnknownType>,
	#[serde(rename = "dynamic_classes")]
	dynamic_classes: String,
	#[serde(rename = "style")]
	style: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssociationSource {
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::IndirectEntityRef,
}

#[derive(Serialize, Deserialize)]
pub struct BuildingBlock {
	#[serde(rename = "canvas_height")]
	canvas_height: i64,
	#[serde(rename = "canvas_width")]
	canvas_width: i64,
	#[serde(rename = "display_name")]
	display_name: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "documentation_url")]
	documentation_url: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "image_data")]
	image_data: Binary,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "platform")]
	platform: String,
	#[serde(rename = "template_category")]
	template_category: String,
	#[serde(rename = "template_category_weight")]
	template_category_weight: i64,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::ActionButton>,
}

#[derive(Serialize, Deserialize)]
pub struct CallNanoflowClientAction {
	#[serde(rename = "confirmation_info")]
	confirmation_info: NULL,
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
	#[serde(rename = "nanoflow")]
	nanoflow: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<UnknownType>,
	#[serde(rename = "progress_bar")]
	progress_bar: String,
	#[serde(rename = "progress_message")]
	progress_message: NULL,
}

#[derive(Serialize, Deserialize)]
pub struct CancelChangesClientAction {
	#[serde(rename = "close_page")]
	close_page: bool,
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CheckBox {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "aria_required")]
	aria_required: bool,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "label_position")]
	label_position: String,
	#[serde(rename = "label_template")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "native_render_mode")]
	native_render_mode: String,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "on_enter_action")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "on_leave_action")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "source_variable")]
	source_variable: forms::PageVariable,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct ClientTemplate {
	#[serde(rename = "fallback")]
	fallback: texts::Text,
	#[serde(rename = "parameters")]
	parameters: Vec<UnknownType>,
	#[serde(rename = "template")]
	template: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct ClientTemplateParameter {
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "formatting_info")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "source_variable")]
	source_variable: NULL,
}

#[derive(Serialize, Deserialize)]
pub struct ClosePageClientAction {
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
	#[serde(rename = "number_of_pages_to_close")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct ComparisonSearchField {
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "custom_date_format")]
	custom_date_format: String,
	#[serde(rename = "default_value")]
	default_value: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "operator")]
	operator: String,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalEditabilitySettings {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "conditions")]
	conditions: Vec<enumerations::Condition>,
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "source_variable")]
	source_variable: forms::PageVariable,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalVisibilitySettings {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "conditions")]
	conditions: Vec<UnknownType>,
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "ignore_security")]
	ignore_security: bool,
	#[serde(rename = "module_roles")]
	module_roles: Vec<UnknownType>,
	#[serde(rename = "source_variable")]
	source_variable: NULL,
}

#[derive(Serialize, Deserialize)]
pub struct ConfirmationInfo {
	#[serde(rename = "cancel_button_caption")]
	cancel_button_caption: texts::Text,
	#[serde(rename = "proceed_button_caption")]
	proceed_button_caption: texts::Text,
	#[serde(rename = "question")]
	question: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct CreateObjectClientAction {
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "number_of_pages_to_close_2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "page_settings")]
	page_settings: forms::FormSettings,
}

#[derive(Serialize, Deserialize)]
pub struct DataGrid {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "columns")]
	columns: Vec<forms::DataGridColumn>,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "control_bar")]
	control_bar: forms::GridControlBar,
	#[serde(rename = "data_source")]
	data_source: forms::GridXPathSource,
	#[serde(rename = "default_button_trigger")]
	default_button_trigger: String,
	#[serde(rename = "is_control_bar_visible")]
	is_control_bar_visible: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "number_of_rows")]
	number_of_rows: i64,
	#[serde(rename = "refresh_time")]
	refresh_time: i64,
	#[serde(rename = "select_first")]
	select_first: bool,
	#[serde(rename = "selection_mode")]
	selection_mode: String,
	#[serde(rename = "show_empty_rows")]
	show_empty_rows: bool,
	#[serde(rename = "show_paging_bar")]
	show_paging_bar: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "tooltip_form")]
	tooltip_form: String,
	#[serde(rename = "width_unit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataGridColumn {
	#[serde(rename = "aggregate_caption")]
	aggregate_caption: texts::Text,
	#[serde(rename = "aggregate_function")]
	aggregate_function: String,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "editable")]
	editable: bool,
	#[serde(rename = "formatting_info")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "show_tooltip")]
	show_tooltip: bool,
	#[serde(rename = "width_value")]
	width_value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DataGridSelectButton {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "icon")]
	icon: forms::ImageIcon,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct DataView {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "data_source")]
	data_source: forms::DataViewSource,
	#[serde(rename = "editable")]
	editable: bool,
	#[serde(rename = "footer_widgets")]
	footer_widgets: Vec<forms::ActionButton>,
	#[serde(rename = "label_width")]
	label_width: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "no_entity_message")]
	no_entity_message: texts::Text,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "show_footer")]
	show_footer: bool,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::FileManager>,
}

#[derive(Serialize, Deserialize)]
pub struct DataViewSource {
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "page_parameter")]
	page_parameter: String,
	#[serde(rename = "snippet_parameter")]
	snippet_parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatePicker {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "aria_required")]
	aria_required: bool,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "formatting_info")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "label_template")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "on_enter_action")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "on_leave_action")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "source_variable")]
	source_variable: NULL,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct DbTableCell {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "height")]
	height: i64,
	#[serde(rename = "is_header")]
	is_header: bool,
	#[serde(rename = "left_column_index")]
	left_column_index: i64,
	#[serde(rename = "top_row_index")]
	top_row_index: i64,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::ActionButton>,
	#[serde(rename = "width")]
	width: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteClientAction {
	#[serde(rename = "close_page")]
	close_page: bool,
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DesignPropertyValue {
	#[serde(rename = "boolean_value")]
	boolean_value: bool,
	#[serde(rename = "key")]
	key: String,
	#[serde(rename = "string_value")]
	string_value: String,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DivContainer {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "on_click_action")]
	on_click_action: forms::NoAction,
	#[serde(rename = "render_mode")]
	render_mode: String,
	#[serde(rename = "screen_reader_hidden")]
	screen_reader_hidden: bool,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "widgets")]
	widgets: Vec<["Forms$DataView", "Forms$ActionButton"]>,
}

#[derive(Serialize, Deserialize)]
pub struct DropDown {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "aria_required")]
	aria_required: bool,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: forms::ConditionalVisibilitySettings,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "empty_option_caption")]
	empty_option_caption: texts::Text,
	#[serde(rename = "label_template")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "on_enter_action")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "on_leave_action")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "source_variable")]
	source_variable: forms::PageVariable,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct DropDownSearchField {
	#[serde(rename = "allow_multi_select")]
	allow_multi_select: bool,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "custom_date_format")]
	custom_date_format: String,
	#[serde(rename = "default_value")]
	default_value: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "operator")]
	operator: String,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "type")]
	type: String,
	#[serde(rename = "x_path_constraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct DynamicText {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "content")]
	content: forms::ClientTemplate,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "native_text_style")]
	native_text_style: String,
	#[serde(rename = "render_mode")]
	render_mode: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FileManager {
	#[serde(rename = "allowed_extensions")]
	allowed_extensions: String,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "label_template")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "max_file_size")]
	max_file_size: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "show_file_in_browser")]
	show_file_in_browser: bool,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormAction {
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
	#[serde(rename = "form_settings")]
	form_settings: forms::FormSettings,
	#[serde(rename = "number_of_pages_to_close_2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "pages_for_specializations")]
	pages_for_specializations: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct FormCallArgument {
	#[serde(rename = "parameter")]
	parameter: String,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::LayoutGrid>,
}

#[derive(Serialize, Deserialize)]
pub struct FormForSpecialization {
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "form_settings")]
	form_settings: forms::FormSettings,
}

#[derive(Serialize, Deserialize)]
pub struct FormSettings {
	#[serde(rename = "form")]
	form: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<forms::PageParameterMapping>,
	#[serde(rename = "title_override")]
	title_override: NULL,
}

#[derive(Serialize, Deserialize)]
pub struct FormattingInfo {
	#[serde(rename = "custom_date_format")]
	custom_date_format: String,
	#[serde(rename = "date_format")]
	date_format: String,
	#[serde(rename = "decimal_precision")]
	decimal_precision: i64,
	#[serde(rename = "enum_format")]
	enum_format: String,
	#[serde(rename = "group_digits")]
	group_digits: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GlyphIcon {
	#[serde(rename = "code")]
	code: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GridActionButton {
	#[serde(rename = "action")]
	action: forms::MicroflowAction,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "icon")]
	icon: forms::GlyphIcon,
	#[serde(rename = "maintain_selection_after_microflow")]
	maintain_selection_after_microflow: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridControlBar {
	#[serde(rename = "default_button_pointer")]
	default_button_pointer: Binary,
	#[serde(rename = "new_buttons")]
	new_buttons: Vec<["Forms$GridSearchButton", "Forms$GridNewButton", "Forms$GridActionButton"]>,
}

#[derive(Serialize, Deserialize)]
pub struct GridNewButton {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "edit_location")]
	edit_location: String,
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "form_settings")]
	form_settings: forms::FormSettings,
	#[serde(rename = "icon")]
	icon: forms::GlyphIcon,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridSearchButton {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "icon")]
	icon: forms::GlyphIcon,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridSelectAllButton {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "icon")]
	icon: forms::ImageIcon,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "selection_type")]
	selection_type: String,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridSortBar {
	#[serde(rename = "sort_items")]
	sort_items: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct GridSortItem {
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "sort_order")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
pub struct GridXPathSource {
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "search_bar")]
	search_bar: forms::SearchBar,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "x_path_constraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupBox {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "collapsible")]
	collapsible: String,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "header_mode")]
	header_mode: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::DataView>,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "left_widgets")]
	left_widgets: Vec<forms::SidebarToggleButton>,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "right_widgets")]
	right_widgets: Vec<forms::Placeholder>,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IconCollectionIcon {
	#[serde(rename = "image")]
	image: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageIcon {
	#[serde(rename = "image")]
	image: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageViewer {
	#[serde(rename = "alternative_text")]
	alternative_text: forms::ClientTemplate,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "click_action")]
	click_action: forms::NoAction,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "data_source")]
	data_source: forms::ImageViewerSource,
	#[serde(rename = "default_image")]
	default_image: String,
	#[serde(rename = "height")]
	height: i64,
	#[serde(rename = "height_unit")]
	height_unit: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "on_click_enlarge")]
	on_click_enlarge: bool,
	#[serde(rename = "responsive")]
	responsive: bool,
	#[serde(rename = "show_as_thumbnail")]
	show_as_thumbnail: bool,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "width")]
	width: i64,
	#[serde(rename = "width_unit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageViewerSource {
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
}

#[derive(Serialize, Deserialize)]
pub struct InputReferenceSetSelector {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "label_template")]
	label_template: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "popup_form_settings")]
	popup_form_settings: forms::FormSettings,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "selector_source")]
	selector_source: forms::SelectorXPathSource,
	#[serde(rename = "source_variable")]
	source_variable: NULL,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Layout {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "canvas_height")]
	canvas_height: i64,
	#[serde(rename = "canvas_width")]
	canvas_width: i64,
	#[serde(rename = "content")]
	content: forms::WebLayoutContent,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutCall {
	#[serde(rename = "arguments")]
	arguments: Vec<forms::FormCallArgument>,
	#[serde(rename = "form")]
	form: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGrid {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "rows")]
	rows: Vec<forms::LayoutGridRow>,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "width")]
	width: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGridColumn {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "phone_weight")]
	phone_weight: i64,
	#[serde(rename = "preview_width")]
	preview_width: i64,
	#[serde(rename = "tablet_weight")]
	tablet_weight: i64,
	#[serde(rename = "vertical_alignment")]
	vertical_alignment: String,
	#[serde(rename = "weight")]
	weight: i64,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::DataView>,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGridRow {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "columns")]
	columns: Vec<forms::LayoutGridColumn>,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "horizontal_alignment")]
	horizontal_alignment: String,
	#[serde(rename = "spacing_between_columns")]
	spacing_between_columns: bool,
	#[serde(rename = "vertical_alignment")]
	vertical_alignment: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListView {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "click_action")]
	click_action: forms::NoAction,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "data_source")]
	data_source: forms::MicroflowSource,
	#[serde(rename = "editable")]
	editable: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "number_of_columns")]
	number_of_columns: i64,
	#[serde(rename = "page_size")]
	page_size: i64,
	#[serde(rename = "pull_down_action")]
	pull_down_action: forms::NoAction,
	#[serde(rename = "scroll_direction")]
	scroll_direction: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "templates")]
	templates: Vec<UnknownType>,
	#[serde(rename = "widgets")]
	widgets: Vec<["Forms$DataView", "Forms$DivContainer"]>,
}

#[derive(Serialize, Deserialize)]
pub struct ListViewSearch {
	#[serde(rename = "search_refs")]
	search_refs: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct ListViewXPathSource {
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "search")]
	search: forms::ListViewSearch,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "x_path_constraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListenTargetSource {
	#[serde(rename = "listen_target")]
	listen_target: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginButton {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "icon")]
	icon: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "render_type")]
	render_type: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
	#[serde(rename = "validation_message_widget")]
	validation_message_widget: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginIdTextBox {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "label")]
	label: texts::Text,
	#[serde(rename = "label_width")]
	label_width: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MenuBar {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "menu_source")]
	menu_source: forms::NavigationSource,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MenuDocumentSource {
	#[serde(rename = "menu")]
	menu: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowAction {
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
	#[serde(rename = "microflow_settings")]
	microflow_settings: forms::MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameterMapping {
	#[serde(rename = "parameter")]
	parameter: String,
	#[serde(rename = "variable")]
	variable: forms::PageVariable,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowSettings {
	#[serde(rename = "asynchronous")]
	asynchronous: bool,
	#[serde(rename = "confirmation_info")]
	confirmation_info: NULL,
	#[serde(rename = "form_validations")]
	form_validations: String,
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<UnknownType>,
	#[serde(rename = "progress_bar")]
	progress_bar: String,
	#[serde(rename = "progress_message")]
	progress_message: NULL,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowSource {
	#[serde(rename = "microflow_settings")]
	microflow_settings: forms::MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowParameterMapping {
	#[serde(rename = "parameter")]
	parameter: String,
	#[serde(rename = "variable")]
	variable: forms::PageVariable,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowSource {
	#[serde(rename = "nanoflow")]
	nanoflow: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<forms::NanoflowParameterMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct NativeLayoutContent {
	#[serde(rename = "layout_type")]
	layout_type: String,
	#[serde(rename = "right_header_placeholder")]
	right_header_placeholder: forms::Placeholder,
	#[serde(rename = "show_bottom_bar")]
	show_bottom_bar: bool,
	#[serde(rename = "sidebar")]
	sidebar: bool,
	#[serde(rename = "sidebar_widgets")]
	sidebar_widgets: Vec<UnknownType>,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::Placeholder>,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationSource {
	#[serde(rename = "navigation_profile")]
	navigation_profile: String,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationTree {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "menu_source")]
	menu_source: forms::NavigationSource,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NewGridDatabaseSource {
	#[serde(rename = "database_constraints")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::IndirectEntityRef,
	#[serde(rename = "search_bar")]
	search_bar: forms::SearchBar,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct NewListViewDatabaseSource {
	#[serde(rename = "database_constraints")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "search")]
	search: forms::ListViewSearch,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct NewSelectorDatabaseSource {
	#[serde(rename = "database_constraints")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct NoAction {
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OpenLinkClientAction {
	#[serde(rename = "address")]
	address: forms::StaticOrDynamicString,
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
	#[serde(rename = "link_type")]
	link_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
	#[serde(rename = "allowed_module_roles")]
	allowed_module_roles: Vec<UnknownType>,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "canvas_height")]
	canvas_height: i64,
	#[serde(rename = "canvas_width")]
	canvas_width: i64,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "form_call")]
	form_call: forms::LayoutCall,
	#[serde(rename = "mark_as_used")]
	mark_as_used: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameters")]
	parameters: Vec<forms::PageParameter>,
	#[serde(rename = "popup_close_action")]
	popup_close_action: String,
	#[serde(rename = "popup_height")]
	popup_height: i64,
	#[serde(rename = "popup_resizable")]
	popup_resizable: bool,
	#[serde(rename = "popup_width")]
	popup_width: i64,
	#[serde(rename = "title")]
	title: texts::Text,
	#[serde(rename = "url")]
	url: String,
}

#[derive(Serialize, Deserialize)]
pub struct PageParameter {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameter_type")]
	parameter_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct PageParameterMapping {
	#[serde(rename = "argument")]
	argument: String,
	#[serde(rename = "parameter")]
	parameter: String,
	#[serde(rename = "variable")]
	variable: forms::PageVariable,
}

#[derive(Serialize, Deserialize)]
pub struct PageVariable {
	#[serde(rename = "page_parameter")]
	page_parameter: String,
	#[serde(rename = "snippet_parameter")]
	snippet_parameter: String,
	#[serde(rename = "use_all_pages")]
	use_all_pages: bool,
	#[serde(rename = "widget")]
	widget: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordTextBox {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "label")]
	label: texts::Text,
	#[serde(rename = "label_width")]
	label_width: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Placeholder {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RadioButtonGroup {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "aria_required")]
	aria_required: bool,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "label_template")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "on_enter_action")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "on_leave_action")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "render_horizontal")]
	render_horizontal: bool,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "source_variable")]
	source_variable: forms::PageVariable,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct RangeSearchField {
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "custom_date_format")]
	custom_date_format: String,
	#[serde(rename = "default_value")]
	default_value: String,
	#[serde(rename = "include_lower")]
	include_lower: bool,
	#[serde(rename = "include_upper")]
	include_upper: bool,
	#[serde(rename = "lower_bound_ref")]
	lower_bound_ref: domain_models::AttributeRef,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "type")]
	type: String,
	#[serde(rename = "upper_bound_ref")]
	upper_bound_ref: domain_models::AttributeRef,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSelector {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "empty_option_caption")]
	empty_option_caption: texts::Text,
	#[serde(rename = "formatting_info")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "goto_form_settings")]
	goto_form_settings: forms::FormSettings,
	#[serde(rename = "label_template")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "popup_form_settings")]
	popup_form_settings: forms::FormSettings,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "render_mode")]
	render_mode: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "selector_source")]
	selector_source: forms::NewSelectorDatabaseSource,
	#[serde(rename = "source_variable")]
	source_variable: NULL,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSetSelector {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "columns")]
	columns: Vec<forms::DataGridColumn>,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "constrained_by_refs")]
	constrained_by_refs: Vec<UnknownType>,
	#[serde(rename = "control_bar")]
	control_bar: forms::GridControlBar,
	#[serde(rename = "data_source")]
	data_source: forms::ReferenceSetSource,
	#[serde(rename = "default_button_trigger")]
	default_button_trigger: String,
	#[serde(rename = "is_control_bar_visible")]
	is_control_bar_visible: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "number_of_rows")]
	number_of_rows: i64,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "refresh_time")]
	refresh_time: i64,
	#[serde(rename = "selectable_x_path_constraint")]
	selectable_x_path_constraint: String,
	#[serde(rename = "select_first")]
	select_first: bool,
	#[serde(rename = "selection_mode")]
	selection_mode: String,
	#[serde(rename = "show_empty_rows")]
	show_empty_rows: bool,
	#[serde(rename = "show_paging_bar")]
	show_paging_bar: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "tooltip_form")]
	tooltip_form: String,
	#[serde(rename = "width_unit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSetSource {
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::IndirectEntityRef,
	#[serde(rename = "search_bar")]
	search_bar: forms::SearchBar,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct SaveChangesClientAction {
	#[serde(rename = "close_page")]
	close_page: bool,
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
	#[serde(rename = "sync_automatically")]
	sync_automatically: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ScrollContainer {
	#[serde(rename = "alignment")]
	alignment: String,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "bottom")]
	bottom: forms::ScrollContainerRegion,
	#[serde(rename = "center_region")]
	center_region: forms::ScrollContainerRegion,
	#[serde(rename = "layout_mode")]
	layout_mode: String,
	#[serde(rename = "left")]
	left: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_hide_scrollbars")]
	native_hide_scrollbars: bool,
	#[serde(rename = "right")]
	right: NULL,
	#[serde(rename = "scroll_behavior")]
	scroll_behavior: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "top")]
	top: forms::ScrollContainerRegion,
	#[serde(rename = "width")]
	width: i64,
	#[serde(rename = "width_mode")]
	width_mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScrollContainerRegion {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "size")]
	size: i64,
	#[serde(rename = "size_mode")]
	size_mode: String,
	#[serde(rename = "toggle_mode")]
	toggle_mode: String,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::ScrollContainer>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchBar {
	#[serde(rename = "new_buttons")]
	new_buttons: Vec<["Forms$ComparisonSearchField", "Forms$DropDownSearchField"]>,
	#[serde(rename = "type")]
	type: String,
	#[serde(rename = "wait_for_search")]
	wait_for_search: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorMicroflowSource {
	#[serde(rename = "data_source_microflow_settings")]
	data_source_microflow_settings: forms::MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorXPathSource {
	#[serde(rename = "constrained_by_refs")]
	constrained_by_refs: Vec<UnknownType>,
	#[serde(rename = "sort_bar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "x_path_constraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct SidebarToggleButton {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "button_style")]
	button_style: String,
	#[serde(rename = "caption_template")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "icon")]
	icon: forms::IconCollectionIcon,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "render_type")]
	render_type: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct SignOutClientAction {
	#[serde(rename = "disabled_during_execution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleMenuBar {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "menu_source")]
	menu_source: forms::MenuDocumentSource,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "orientation")]
	orientation: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Snippet {
	#[serde(rename = "canvas_height")]
	canvas_height: i64,
	#[serde(rename = "canvas_width")]
	canvas_width: i64,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameters")]
	parameters: Vec<forms::SnippetParameter>,
	#[serde(rename = "type")]
	type: String,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::LayoutGrid>,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetCall {
	#[serde(rename = "form")]
	form: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetCallWidget {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "form_call")]
	form_call: forms::SnippetCall,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetParameter {
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameter_type")]
	parameter_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct StaticImageViewer {
	#[serde(rename = "alternative_text")]
	alternative_text: forms::ClientTemplate,
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "click_action")]
	click_action: forms::NoAction,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "height")]
	height: i64,
	#[serde(rename = "height_unit")]
	height_unit: String,
	#[serde(rename = "image")]
	image: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "responsive")]
	responsive: bool,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "width")]
	width: i64,
	#[serde(rename = "width_unit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct StaticOrDynamicString {
	#[serde(rename = "attribute_ref")]
	attribute_ref: NULL,
	#[serde(rename = "is_dynamic")]
	is_dynamic: bool,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct TabControl {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "default_page_pointer")]
	default_page_pointer: Binary,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "tab_pages")]
	tab_pages: Vec<forms::TabPage>,
}

#[derive(Serialize, Deserialize)]
pub struct TabPage {
	#[serde(rename = "badge")]
	badge: NULL,
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "refresh_on_show")]
	refresh_on_show: bool,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::DivContainer>,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "cells")]
	cells: Vec<forms::DbTableCell>,
	#[serde(rename = "column_widths")]
	column_widths: Vec<forms::TableColumn>,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "rows")]
	rows: Vec<forms::TableRow>,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "width_unit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct TableColumn {
	#[serde(rename = "value")]
	value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TableRow {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateGrid {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "contents")]
	contents: forms::TemplateGridContents,
	#[serde(rename = "control_bar")]
	control_bar: forms::GridControlBar,
	#[serde(rename = "data_source")]
	data_source: forms::GridXPathSource,
	#[serde(rename = "default_button_trigger")]
	default_button_trigger: String,
	#[serde(rename = "is_control_bar_visible")]
	is_control_bar_visible: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "number_of_columns")]
	number_of_columns: i64,
	#[serde(rename = "number_of_rows")]
	number_of_rows: i64,
	#[serde(rename = "refresh_time")]
	refresh_time: i64,
	#[serde(rename = "select_first")]
	select_first: bool,
	#[serde(rename = "selection_mode")]
	selection_mode: String,
	#[serde(rename = "show_paging_bar")]
	show_paging_bar: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateGridContents {
	#[serde(rename = "widgets")]
	widgets: Vec<forms::LayoutGrid>,
}

#[derive(Serialize, Deserialize)]
pub struct TextArea {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "aria_required")]
	aria_required: bool,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "autocomplete")]
	autocomplete: bool,
	#[serde(rename = "auto_focus")]
	auto_focus: bool,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "counter_message")]
	counter_message: texts::Text,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "label_template")]
	label_template: NULL,
	#[serde(rename = "max_length_code")]
	max_length_code: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "number_of_lines")]
	number_of_lines: i64,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "on_enter_action")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "on_leave_action")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "source_variable")]
	source_variable: NULL,
	#[serde(rename = "submit_behaviour")]
	submit_behaviour: String,
	#[serde(rename = "submit_on_input_delay")]
	submit_on_input_delay: i64,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "text_too_long_message")]
	text_too_long_message: texts::Text,
	#[serde(rename = "validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct TextBox {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "aria_required")]
	aria_required: bool,
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "autocomplete")]
	autocomplete: bool,
	#[serde(rename = "autocomplete_purpose")]
	autocomplete_purpose: String,
	#[serde(rename = "auto_focus")]
	auto_focus: bool,
	#[serde(rename = "conditional_editability_settings")]
	conditional_editability_settings: NULL,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: forms::ConditionalVisibilitySettings,
	#[serde(rename = "editable")]
	editable: String,
	#[serde(rename = "formatting_info")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "input_mask")]
	input_mask: String,
	#[serde(rename = "is_password_box")]
	is_password_box: bool,
	#[serde(rename = "keyboard_type")]
	keyboard_type: String,
	#[serde(rename = "label_template")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "max_length_code")]
	max_length_code: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "on_change_action")]
	on_change_action: forms::NoAction,
	#[serde(rename = "on_enter_action")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "on_enter_key_press_action")]
	on_enter_key_press_action: forms::NoAction,
	#[serde(rename = "on_leave_action")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "read_only_style")]
	read_only_style: String,
	#[serde(rename = "screen_reader_label")]
	screen_reader_label: NULL,
	#[serde(rename = "source_variable")]
	source_variable: forms::PageVariable,
	#[serde(rename = "submit_behaviour")]
	submit_behaviour: String,
	#[serde(rename = "submit_on_input_delay")]
	submit_on_input_delay: i64,
	#[serde(rename = "tab_index")]
	tab_index: i64,
	#[serde(rename = "validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "conditional_visibility_settings")]
	conditional_visibility_settings: NULL,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "native_accessibility_settings")]
	native_accessibility_settings: NULL,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationMessage {
	#[serde(rename = "appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tab_index")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct WebLayoutContent {
	#[serde(rename = "layout_call")]
	layout_call: NULL,
	#[serde(rename = "layout_type")]
	layout_type: String,
	#[serde(rename = "widgets")]
	widgets: Vec<forms::ScrollContainer>,
}

#[derive(Serialize, Deserialize)]
pub struct WebUIProjectSettingsPart {
	#[serde(rename = "enable_download_resources")]
	enable_download_resources: bool,
	#[serde(rename = "enable_microflow_reachability_analysis")]
	enable_microflow_reachability_analysis: bool,
	#[serde(rename = "enable_widget_bundling")]
	enable_widget_bundling: bool,
	#[serde(rename = "theme_module_order")]
	theme_module_order: Vec<settings::ThemeModuleEntry>,
	#[serde(rename = "use_optimized_client")]
	use_optimized_client: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValidation {
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "message")]
	message: texts::Text,
}

