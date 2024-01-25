use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ActionButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	action: forms::MicroflowAction,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AriaRole")]
	aria_role: String,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: forms::ConditionalVisibilitySettings,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct Appearance {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Class")]
	class: String,
	#[serde(rename = "DesignProperties")]
	design_properties: Vec<forms::DesignPropertyValue, >,
	#[serde(rename = "DynamicClasses")]
	dynamic_classes: String,
	#[serde(rename = "Style")]
	style: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssociationSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::IndirectEntityRef,
}

#[derive(Serialize, Deserialize)]
pub struct BuildingBlock {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "DisplayName")]
	display_name: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "DocumentationUrl")]
	documentation_url: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ImageData")]
	image_data: Binary,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Platform")]
	platform: String,
	#[serde(rename = "TemplateCategory")]
	template_category: String,
	#[serde(rename = "TemplateCategoryWeight")]
	template_category_weight: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::ActionButton, >,
}

#[derive(Serialize, Deserialize)]
pub struct CallNanoflowClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ConfirmationInfo")]
	confirmation_info: Empty,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<UnknownType>,
	#[serde(rename = "ProgressBar")]
	progress_bar: String,
	#[serde(rename = "ProgressMessage")]
	progress_message: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct CancelChangesClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ClosePage")]
	close_page: bool,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CheckBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: forms::ConditionalVisibilitySettings,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelPosition")]
	label_position: String,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "NativeRenderMode")]
	native_render_mode: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct ClientTemplate {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Fallback")]
	fallback: texts::Text,
	#[serde(rename = "Parameters")]
	parameters: Vec<forms::ClientTemplateParameter, >,
	#[serde(rename = "Template")]
	template: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct ClientTemplateParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct ClosePageClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct ComparisonSearchField {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operator")]
	operator: String,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalEditabilitySettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Conditions")]
	conditions: Vec<enumerations::Condition, >,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalVisibilitySettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Conditions")]
	conditions: Vec<enumerations::Condition, >,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "IgnoreSecurity")]
	ignore_security: bool,
	#[serde(rename = "ModuleRoles")]
	module_roles: Vec<String>,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct ConfirmationInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "CancelButtonCaption")]
	cancel_button_caption: texts::Text,
	#[serde(rename = "ProceedButtonCaption")]
	proceed_button_caption: texts::Text,
	#[serde(rename = "Question")]
	question: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct CreateObjectClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "NumberOfPagesToClose2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "PageSettings")]
	page_settings: forms::FormSettings,
}

#[derive(Serialize, Deserialize)]
pub struct DataGrid {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "Columns")]
	columns: Vec<forms::DataGridColumn, >,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "ControlBar")]
	control_bar: forms::GridControlBar,
	#[serde(rename = "DataSource")]
	data_source: forms::GridXPathSource,
	#[serde(rename = "DefaultButtonTrigger")]
	default_button_trigger: String,
	#[serde(rename = "IsControlBarVisible")]
	is_control_bar_visible: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfRows")]
	number_of_rows: i64,
	#[serde(rename = "RefreshTime")]
	refresh_time: i64,
	#[serde(rename = "SelectFirst")]
	select_first: bool,
	#[serde(rename = "SelectionMode")]
	selection_mode: String,
	#[serde(rename = "ShowEmptyRows")]
	show_empty_rows: bool,
	#[serde(rename = "ShowPagingBar")]
	show_paging_bar: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TooltipForm")]
	tooltip_form: String,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataGridColumn {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AggregateCaption")]
	aggregate_caption: texts::Text,
	#[serde(rename = "AggregateFunction")]
	aggregate_function: String,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "Editable")]
	editable: bool,
	#[serde(rename = "FormattingInfo")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ShowTooltip")]
	show_tooltip: bool,
	#[serde(rename = "WidthValue")]
	width_value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DataGridSelectButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct DataView {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "DataSource")]
	data_source: forms::DataViewSource,
	#[serde(rename = "Editable")]
	editable: bool,
	#[serde(rename = "FooterWidgets")]
	footer_widgets: Vec<forms::CheckBox, forms::ActionButton, forms::DataView, forms::LayoutGrid, forms::DivContainer, >,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NoEntityMessage")]
	no_entity_message: texts::Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ShowFooter")]
	show_footer: bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::DataView, forms::ScrollContainer, forms::ReferenceSelector, forms::TemplateGrid, forms::Table, forms::LayoutGrid, forms::DropDown, forms::ListView, forms::ImageViewer, forms::TextBox, forms::DivContainer, custom_widgets::CustomWidget, forms::ActionButton, forms::DatePicker, forms::DynamicText, forms::CheckBox, forms::SnippetCallWidget, forms::GroupBox, forms::FileManager, forms::RadioButtonGroup, forms::DataGrid, forms::Title, forms::TabControl, forms::TextArea, forms::InputReferenceSetSelector, forms::Label, >,
}

#[derive(Serialize, Deserialize)]
pub struct DataViewSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "PageParameter")]
	page_parameter: String,
	#[serde(rename = "SnippetParameter")]
	snippet_parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatePicker {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct DbTableCell {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "IsHeader")]
	is_header: bool,
	#[serde(rename = "LeftColumnIndex")]
	left_column_index: i64,
	#[serde(rename = "TopRowIndex")]
	top_row_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::TextBox, forms::DatePicker, forms::Table, forms::TabControl, forms::FileManager, forms::CheckBox, forms::TextArea, forms::DynamicText, forms::DivContainer, forms::DataView, forms::DropDown, forms::ReferenceSelector, forms::ActionButton, forms::Label, forms::ReferenceSetSelector, forms::DataGrid, >,
	#[serde(rename = "Width")]
	width: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ClosePage")]
	close_page: bool,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DesignPropertyValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "BooleanValue")]
	boolean_value: bool,
	#[serde(rename = "Key")]
	key: String,
	#[serde(rename = "StringValue")]
	string_value: String,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DivContainer {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "OnClickAction")]
	on_click_action: forms::NoAction,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "ScreenReaderHidden")]
	screen_reader_hidden: bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::ListView, forms::DropDown, forms::TextArea, forms::ValidationMessage, forms::StaticImageViewer, forms::TextBox, forms::Label, forms::ImageViewer, forms::ReferenceSetSelector, forms::FileManager, forms::Placeholder, forms::ActionButton, forms::LayoutGrid, custom_widgets::CustomWidget, forms::DataGrid, forms::CheckBox, forms::TabControl, forms::LoginButton, forms::LoginIdTextBox, forms::PasswordTextBox, forms::RadioButtonGroup, forms::MenuBar, forms::SnippetCallWidget, forms::InputReferenceSetSelector, forms::ReferenceSelector, forms::Title, forms::DatePicker, forms::DataView, forms::DivContainer, forms::TemplateGrid, forms::SidebarToggleButton, forms::DynamicText, >,
}

#[derive(Serialize, Deserialize)]
pub struct DropDown {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "EmptyOptionCaption")]
	empty_option_caption: texts::Text,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "SourceVariable")]
	source_variable: forms::PageVariable,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct DropDownSearchField {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowMultiSelect")]
	allow_multi_select: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operator")]
	operator: String,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct DynamicText {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Content")]
	content: forms::ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "NativeTextStyle")]
	native_text_style: String,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FileManager {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowedExtensions")]
	allowed_extensions: String,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "MaxFileSize")]
	max_file_size: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "ShowFileInBrowser")]
	show_file_in_browser: bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "FormSettings")]
	form_settings: forms::FormSettings,
	#[serde(rename = "NumberOfPagesToClose2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "PagesForSpecializations")]
	pages_for_specializations: Vec<forms::FormForSpecialization, >,
}

#[derive(Serialize, Deserialize)]
pub struct FormCallArgument {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::DataView, forms::DataGrid, forms::Label, forms::ScrollContainer, forms::LayoutGrid, forms::TabControl, forms::Placeholder, forms::TemplateGrid, forms::DivContainer, forms::SnippetCallWidget, forms::DynamicText, forms::Table, forms::ActionButton, forms::Title, >,
}

#[derive(Serialize, Deserialize)]
pub struct FormForSpecialization {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "FormSettings")]
	form_settings: forms::FormSettings,
}

#[derive(Serialize, Deserialize)]
pub struct FormSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<forms::PageParameterMapping, >,
	#[serde(rename = "TitleOverride")]
	title_override: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct FormattingInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DateFormat")]
	date_format: String,
	#[serde(rename = "DecimalPrecision")]
	decimal_precision: i64,
	#[serde(rename = "EnumFormat")]
	enum_format: String,
	#[serde(rename = "GroupDigits")]
	group_digits: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GlyphIcon {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Code")]
	code: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GridActionButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	action: forms::MicroflowAction,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "MaintainSelectionAfterMicroflow")]
	maintain_selection_after_microflow: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridControlBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DefaultButtonPointer")]
	default_button_pointer: Binary,
	#[serde(rename = "NewButtons")]
	new_buttons: Vec<forms::GridSelectAllButton, forms::GridActionButton, forms::GridSearchButton, forms::GridNewButton, forms::DataGridSelectButton, >,
}

#[derive(Serialize, Deserialize)]
pub struct GridNewButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "EditLocation")]
	edit_location: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "FormSettings")]
	form_settings: forms::FormSettings,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridSearchButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridSelectAllButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Icon")]
	icon: forms::ImageIcon,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "SelectionType")]
	selection_type: String,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct GridSortBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "SortItems")]
	sort_items: Vec<forms::GridSortItem, >,
}

#[derive(Serialize, Deserialize)]
pub struct GridSortItem {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "SortOrder")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
pub struct GridXPathSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "SearchBar")]
	search_bar: forms::SearchBar,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "Collapsible")]
	collapsible: String,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "HeaderMode")]
	header_mode: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::ListView, forms::DivContainer, forms::LayoutGrid, forms::TabControl, forms::DataGrid, forms::DataView, >,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "LeftWidgets")]
	left_widgets: Vec<forms::Placeholder, forms::SidebarToggleButton, >,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RightWidgets")]
	right_widgets: Vec<forms::Placeholder, forms::SidebarToggleButton, >,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IconCollectionIcon {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Image")]
	image: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageIcon {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Image")]
	image: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageViewer {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AlternativeText")]
	alternative_text: forms::ClientTemplate,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ClickAction")]
	click_action: forms::CallNanoflowClientAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "DataSource")]
	data_source: forms::ImageViewerSource,
	#[serde(rename = "DefaultImage")]
	default_image: String,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "HeightUnit")]
	height_unit: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "OnClickEnlarge")]
	on_click_enlarge: bool,
	#[serde(rename = "Responsive")]
	responsive: bool,
	#[serde(rename = "ShowAsThumbnail")]
	show_as_thumbnail: bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Width")]
	width: i64,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageViewerSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::DirectEntityRef,
}

#[derive(Serialize, Deserialize)]
pub struct InputReferenceSetSelector {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "PopupFormSettings")]
	popup_form_settings: forms::FormSettings,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "SelectorSource")]
	selector_source: forms::SelectorXPathSource,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Layout {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Content")]
	content: forms::WebLayoutContent,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Arguments")]
	arguments: Vec<forms::FormCallArgument, >,
	#[serde(rename = "Form")]
	form: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGrid {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Rows")]
	rows: Vec<forms::LayoutGridRow, >,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Width")]
	width: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGridColumn {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "PhoneWeight")]
	phone_weight: i64,
	#[serde(rename = "PreviewWidth")]
	preview_width: i64,
	#[serde(rename = "TabletWeight")]
	tablet_weight: i64,
	#[serde(rename = "VerticalAlignment")]
	vertical_alignment: String,
	#[serde(rename = "Weight")]
	weight: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::ListView, forms::Title, forms::MenuBar, forms::TemplateGrid, forms::DynamicText, forms::GroupBox, forms::TextArea, forms::LoginButton, forms::CheckBox, forms::DataGrid, forms::TabControl, forms::FileManager, forms::Label, forms::DivContainer, forms::NavigationTree, forms::Placeholder, forms::LoginIdTextBox, forms::DropDown, forms::ActionButton, forms::SnippetCallWidget, forms::DataView, forms::TextBox, custom_widgets::CustomWidget, forms::RadioButtonGroup, forms::InputReferenceSetSelector, forms::LayoutGrid, forms::StaticImageViewer, forms::PasswordTextBox, forms::ReferenceSelector, forms::DatePicker, >,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGridRow {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Columns")]
	columns: Vec<forms::LayoutGridColumn, >,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "HorizontalAlignment")]
	horizontal_alignment: String,
	#[serde(rename = "SpacingBetweenColumns")]
	spacing_between_columns: bool,
	#[serde(rename = "VerticalAlignment")]
	vertical_alignment: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListView {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ClickAction")]
	click_action: forms::NoAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "DataSource")]
	data_source: forms::MicroflowSource,
	#[serde(rename = "Editable")]
	editable: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfColumns")]
	number_of_columns: i64,
	#[serde(rename = "PageSize")]
	page_size: i64,
	#[serde(rename = "PullDownAction")]
	pull_down_action: forms::NoAction,
	#[serde(rename = "ScrollDirection")]
	scroll_direction: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Templates")]
	templates: Vec<UnknownType>,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::ActionButton, forms::GroupBox, forms::DivContainer, forms::DataView, forms::FileManager, forms::SnippetCallWidget, forms::CheckBox, forms::RadioButtonGroup, forms::DatePicker, forms::ListView, forms::DynamicText, forms::LayoutGrid, forms::TextArea, forms::TextBox, >,
}

#[derive(Serialize, Deserialize)]
pub struct ListViewSearch {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "SearchRefs")]
	search_refs: Vec<domain_models::AttributeRef, >,
}

#[derive(Serialize, Deserialize)]
pub struct ListViewXPathSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "Search")]
	search: forms::ListViewSearch,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListenTargetSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListenTarget")]
	listen_target: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
	#[serde(rename = "ValidationMessageWidget")]
	validation_message_widget: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginIdTextBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Label")]
	label: texts::Text,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MenuBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "MenuSource")]
	menu_source: forms::NavigationSource,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MenuDocumentSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Menu")]
	menu: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "MicroflowSettings")]
	microflow_settings: forms::MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: forms::PageVariable,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Asynchronous")]
	asynchronous: bool,
	#[serde(rename = "ConfirmationInfo")]
	confirmation_info: Empty,
	#[serde(rename = "FormValidations")]
	form_validations: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<forms::MicroflowParameterMapping, >,
	#[serde(rename = "ProgressBar")]
	progress_bar: String,
	#[serde(rename = "ProgressMessage")]
	progress_message: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "MicroflowSettings")]
	microflow_settings: forms::MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: forms::PageVariable,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<forms::NanoflowParameterMapping, >,
}

#[derive(Serialize, Deserialize)]
pub struct NativeLayoutContent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "LayoutType")]
	layout_type: String,
	#[serde(rename = "RightHeaderPlaceholder")]
	right_header_placeholder: Empty,
	#[serde(rename = "ShowBottomBar")]
	show_bottom_bar: bool,
	#[serde(rename = "Sidebar")]
	sidebar: bool,
	#[serde(rename = "SidebarWidgets")]
	sidebar_widgets: Vec<forms::Placeholder, >,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::Placeholder, custom_widgets::CustomWidget, >,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "NavigationProfile")]
	navigation_profile: String,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationTree {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "MenuSource")]
	menu_source: forms::NavigationSource,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NewGridDatabaseSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "SearchBar")]
	search_bar: forms::SearchBar,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct NewListViewDatabaseSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::IndirectEntityRef,
	#[serde(rename = "Search")]
	search: forms::ListViewSearch,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct NewSelectorDatabaseSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct NoAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OpenLinkClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Address")]
	address: forms::StaticOrDynamicString,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "LinkType")]
	link_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "FormCall")]
	form_call: forms::LayoutCall,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<forms::PageParameter, >,
	#[serde(rename = "PopupCloseAction")]
	popup_close_action: String,
	#[serde(rename = "PopupHeight")]
	popup_height: i64,
	#[serde(rename = "PopupResizable")]
	popup_resizable: bool,
	#[serde(rename = "PopupWidth")]
	popup_width: i64,
	#[serde(rename = "Title")]
	title: texts::Text,
	#[serde(rename = "Url")]
	url: String,
}

#[derive(Serialize, Deserialize)]
pub struct PageParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct PageParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: forms::PageVariable,
}

#[derive(Serialize, Deserialize)]
pub struct PageVariable {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "PageParameter")]
	page_parameter: String,
	#[serde(rename = "SnippetParameter")]
	snippet_parameter: String,
	#[serde(rename = "UseAllPages")]
	use_all_pages: bool,
	#[serde(rename = "Widget")]
	widget: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordTextBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Label")]
	label: texts::Text,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Placeholder {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RadioButtonGroup {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "RenderHorizontal")]
	render_horizontal: bool,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "SourceVariable")]
	source_variable: forms::PageVariable,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct RangeSearchField {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "IncludeLower")]
	include_lower: bool,
	#[serde(rename = "IncludeUpper")]
	include_upper: bool,
	#[serde(rename = "LowerBoundRef")]
	lower_bound_ref: domain_models::AttributeRef,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "UpperBoundRef")]
	upper_bound_ref: domain_models::AttributeRef,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSelector {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "EmptyOptionCaption")]
	empty_option_caption: texts::Text,
	#[serde(rename = "FormattingInfo")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "GotoFormSettings")]
	goto_form_settings: forms::FormSettings,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "PopupFormSettings")]
	popup_form_settings: forms::FormSettings,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "SelectorSource")]
	selector_source: forms::NewSelectorDatabaseSource,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSetSelector {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Columns")]
	columns: Vec<forms::DataGridColumn, >,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "ConstrainedByRefs")]
	constrained_by_refs: Vec<UnknownType>,
	#[serde(rename = "ControlBar")]
	control_bar: forms::GridControlBar,
	#[serde(rename = "DataSource")]
	data_source: forms::ReferenceSetSource,
	#[serde(rename = "DefaultButtonTrigger")]
	default_button_trigger: String,
	#[serde(rename = "IsControlBarVisible")]
	is_control_bar_visible: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfRows")]
	number_of_rows: i64,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "RefreshTime")]
	refresh_time: i64,
	#[serde(rename = "SelectableXPathConstraint")]
	selectable_x_path_constraint: String,
	#[serde(rename = "SelectFirst")]
	select_first: bool,
	#[serde(rename = "SelectionMode")]
	selection_mode: String,
	#[serde(rename = "ShowEmptyRows")]
	show_empty_rows: bool,
	#[serde(rename = "ShowPagingBar")]
	show_paging_bar: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TooltipForm")]
	tooltip_form: String,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSetSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: domain_models::IndirectEntityRef,
	#[serde(rename = "SearchBar")]
	search_bar: forms::SearchBar,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
}

#[derive(Serialize, Deserialize)]
pub struct SaveChangesClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ClosePage")]
	close_page: bool,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "SyncAutomatically")]
	sync_automatically: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ScrollContainer {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Alignment")]
	alignment: String,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Bottom")]
	bottom: forms::ScrollContainerRegion,
	#[serde(rename = "CenterRegion")]
	center_region: forms::ScrollContainerRegion,
	#[serde(rename = "LayoutMode")]
	layout_mode: String,
	#[serde(rename = "Left")]
	left: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeHideScrollbars")]
	native_hide_scrollbars: bool,
	#[serde(rename = "Right")]
	right: Empty,
	#[serde(rename = "ScrollBehavior")]
	scroll_behavior: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Top")]
	top: forms::ScrollContainerRegion,
	#[serde(rename = "Width")]
	width: i64,
	#[serde(rename = "WidthMode")]
	width_mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScrollContainerRegion {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Size")]
	size: i64,
	#[serde(rename = "SizeMode")]
	size_mode: String,
	#[serde(rename = "ToggleMode")]
	toggle_mode: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::SnippetCallWidget, forms::Placeholder, forms::Label, forms::Header, forms::DataView, forms::StaticImageViewer, forms::TabControl, custom_widgets::CustomWidget, forms::LayoutGrid, forms::ListView, forms::ScrollContainer, forms::DivContainer, forms::SimpleMenuBar, forms::MenuBar, forms::ActionButton, forms::SidebarToggleButton, forms::NavigationTree, >,
}

#[derive(Serialize, Deserialize)]
pub struct SearchBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "NewButtons")]
	new_buttons: Vec<forms::ComparisonSearchField, forms::DropDownSearchField, forms::RangeSearchField, >,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "WaitForSearch")]
	wait_for_search: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorMicroflowSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DataSourceMicroflowSettings")]
	data_source_microflow_settings: forms::MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorXPathSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ConstrainedByRefs")]
	constrained_by_refs: Vec<domain_models::IndirectEntityRef, >,
	#[serde(rename = "SortBar")]
	sort_bar: forms::GridSortBar,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct SidebarToggleButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Icon")]
	icon: forms::GlyphIcon,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct SignOutClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleMenuBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "MenuSource")]
	menu_source: forms::NavigationSource,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Orientation")]
	orientation: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Snippet {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<forms::SnippetParameter, >,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::ReferenceSelector, custom_widgets::CustomWidget, forms::StaticImageViewer, forms::FileManager, forms::TabControl, forms::ListView, forms::LayoutGrid, forms::ActionButton, forms::DataView, forms::TextBox, forms::TemplateGrid, forms::Label, forms::Table, forms::DatePicker, forms::CheckBox, forms::DynamicText, forms::DivContainer, forms::DropDown, forms::RadioButtonGroup, forms::DataGrid, forms::SnippetCallWidget, forms::GroupBox, forms::TextArea, >,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetCallWidget {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "FormCall")]
	form_call: forms::SnippetCall,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct StaticImageViewer {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AlternativeText")]
	alternative_text: forms::ClientTemplate,
	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ClickAction")]
	click_action: forms::NoAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "HeightUnit")]
	height_unit: String,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "Responsive")]
	responsive: bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Width")]
	width: i64,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct StaticOrDynamicString {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "IsDynamic")]
	is_dynamic: bool,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct TabControl {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "DefaultPagePointer")]
	default_page_pointer: Binary,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TabPages")]
	tab_pages: Vec<forms::TabPage, >,
}

#[derive(Serialize, Deserialize)]
pub struct TabPage {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Badge")]
	badge: Empty,
	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RefreshOnShow")]
	refresh_on_show: bool,
	#[serde(rename = "Widgets")]
	widgets: Vec<forms::DataGrid, forms::DataView, forms::DynamicText, forms::SnippetCallWidget, forms::LayoutGrid, forms::RadioButtonGroup, forms::TemplateGrid, forms::TabControl, forms::DivContainer, forms::ListView, forms::TextBox, forms::ActionButton, forms::DatePicker, forms::ScrollContainer, forms::Table, forms::TextArea, >,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Cells")]
	cells: Vec<forms::DbTableCell, >,
	#[serde(rename = "ColumnWidths")]
	column_widths: Vec<forms::TableColumn, >,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Rows")]
	rows: Vec<forms::TableRow, >,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct TableColumn {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Value")]
	value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TableRow {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateGrid {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Contents")]
	contents: forms::TemplateGridContents,
	#[serde(rename = "ControlBar")]
	control_bar: forms::GridControlBar,
	#[serde(rename = "DataSource")]
	data_source: forms::MicroflowSource,
	#[serde(rename = "DefaultButtonTrigger")]
	default_button_trigger: String,
	#[serde(rename = "IsControlBarVisible")]
	is_control_bar_visible: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfColumns")]
	number_of_columns: i64,
	#[serde(rename = "NumberOfRows")]
	number_of_rows: i64,
	#[serde(rename = "RefreshTime")]
	refresh_time: i64,
	#[serde(rename = "SelectFirst")]
	select_first: bool,
	#[serde(rename = "SelectionMode")]
	selection_mode: String,
	#[serde(rename = "ShowPagingBar")]
	show_paging_bar: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateGridContents {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Widgets")]
	widgets: Vec<forms::StaticImageViewer, forms::DivContainer, forms::LayoutGrid, forms::SnippetCallWidget, forms::DataView, >,
}

#[derive(Serialize, Deserialize)]
pub struct TextArea {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "Autocomplete")]
	autocomplete: bool,
	#[serde(rename = "AutoFocus")]
	auto_focus: bool,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Empty,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "CounterMessage")]
	counter_message: texts::Text,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Empty,
	#[serde(rename = "MaxLengthCode")]
	max_length_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "NumberOfLines")]
	number_of_lines: i64,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: forms::ClientTemplate,
	#[serde(rename = "SourceVariable")]
	source_variable: Empty,
	#[serde(rename = "SubmitBehaviour")]
	submit_behaviour: String,
	#[serde(rename = "SubmitOnInputDelay")]
	submit_on_input_delay: i64,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TextTooLongMessage")]
	text_too_long_message: texts::Text,
	#[serde(rename = "Validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct TextBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "Autocomplete")]
	autocomplete: bool,
	#[serde(rename = "AutocompletePurpose")]
	autocomplete_purpose: String,
	#[serde(rename = "AutoFocus")]
	auto_focus: bool,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: forms::ConditionalEditabilitySettings,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: forms::FormattingInfo,
	#[serde(rename = "InputMask")]
	input_mask: String,
	#[serde(rename = "IsPasswordBox")]
	is_password_box: bool,
	#[serde(rename = "KeyboardType")]
	keyboard_type: String,
	#[serde(rename = "LabelTemplate")]
	label_template: forms::ClientTemplate,
	#[serde(rename = "MaxLengthCode")]
	max_length_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "OnChangeAction")]
	on_change_action: forms::MicroflowAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: forms::NoAction,
	#[serde(rename = "OnEnterKeyPressAction")]
	on_enter_key_press_action: forms::NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: forms::NoAction,
	#[serde(rename = "Placeholder")]
	placeholder: texts::Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Empty,
	#[serde(rename = "SourceVariable")]
	source_variable: forms::PageVariable,
	#[serde(rename = "SubmitBehaviour")]
	submit_behaviour: String,
	#[serde(rename = "SubmitOnInputDelay")]
	submit_on_input_delay: i64,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: forms::WidgetValidation,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Empty,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationMessage {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct WebLayoutContent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "LayoutCall")]
	layout_call: Empty,
	#[serde(rename = "LayoutType")]
	layout_type: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<custom_widgets::CustomWidget, forms::ScrollContainer, forms::Placeholder, >,
}

#[derive(Serialize, Deserialize)]
pub struct WebUIProjectSettingsPart {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EnableDownloadResources")]
	enable_download_resources: bool,
	#[serde(rename = "EnableMicroflowReachabilityAnalysis")]
	enable_microflow_reachability_analysis: bool,
	#[serde(rename = "EnableWidgetBundling")]
	enable_widget_bundling: bool,
	#[serde(rename = "ThemeModuleOrder")]
	theme_module_order: Vec<settings::ThemeModuleEntry, >,
	#[serde(rename = "UseOptimizedClient")]
	use_optimized_client: String,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetValidation {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "Message")]
	message: texts::Text,
}

