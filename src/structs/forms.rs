use bson::Binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use self::microflows::TextTemplate;

use super::*;


#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum Widgets {
	#[serde(rename = "Forms$ActionButton")]
	ActionButton(forms::ActionButton),
	#[serde(rename = "Forms$CheckBox")]
	CheckBox(forms::CheckBox),
	#[serde(rename = "CustomWidgets$CustomWidget")]
	CustomWidget(custom_widgets::CustomWidget), 
	#[serde(rename = "Forms$DataGrid")]
	DataGrid(forms::DataGrid),
	#[serde(rename = "Forms$DataView")]
	DataView(forms::DataView),
	#[serde(rename = "Forms$DataGridExportToCSVButton")]
	DataGridExportToCSVButton(forms::DataGridExportToCSVButton),
	#[serde(rename = "Forms$DataGridExportToExcelButton")]
	DataGridExportToExcelButton(forms::DataGridExportToExcelButton),
	#[serde(rename = "Forms$DatePicker")]
	DatePicker(forms::DatePicker),
	#[serde(rename = "Forms$DivContainer")]
	DivContainer(forms::DivContainer),
	#[serde(rename = "Forms$DropDown")]
	DropDown(forms::DropDown),
	#[serde(rename = "Forms$DynamicText")]
	DynamicText(forms::DynamicText),
	#[serde(rename = "Forms$FileManager")]
	FileManager(forms::FileManager),
	#[serde(rename = "Forms$FormForSpecialization")]
	FormForSpecialization(forms::FormForSpecialization),
	#[serde(rename = "Forms$GroupBox")]
	GroupBox(forms::GroupBox),
	#[serde(rename = "Forms$Header")]
	Header(forms::Header), 
	#[serde(rename = "Forms$ImageViewer")]
	ImageViewer(forms::ImageViewer),
	#[serde(rename = "Forms$InputReferenceSetSelector")]
	InputReferenceSetSelector(forms::InputReferenceSetSelector),
	#[serde(rename = "Forms$Label")]
	Label(forms::Label), 
	#[serde(rename = "Forms$LayoutGrid")]
	LayoutGrid(forms::LayoutGrid),
	#[serde(rename = "Forms$ListView")]
	ListView(forms::ListView),
	#[serde(rename = "Forms$LoginButton")]
	LoginButton(forms::LoginButton),
	#[serde(rename = "Forms$LoginIdTextBox")]
	LoginIdTextBox(forms::LoginIdTextBox),
	#[serde(rename = "Forms$MenuBar")]
	MenuBar(forms::MenuBar), 
	#[serde(rename = "Forms$NavigationTree")]
	NavigationTree(forms::NavigationTree),
	#[serde(rename = "Forms$PageParameterMapping")]
	PageParameterMapping(forms::PageParameterMapping),
	#[serde(rename = "Forms$PasswordTextBox")]
	PasswordTextBox(forms::PasswordTextBox),
	#[serde(rename = "Forms$Placeholder")]
	Placeholder(forms::Placeholder), 
	#[serde(rename = "Forms$RadioButtonGroup")]
	RadioButtonGroup(forms::RadioButtonGroup),
	#[serde(rename = "Forms$ReferenceSelector")]
	ReferenceSelector(forms::ReferenceSelector),
	#[serde(rename = "Forms$ReferenceSetSelector")]
	ReferenceSetSelector(forms::ReferenceSetSelector),
	#[serde(rename = "Forms$ScrollContainer")]
	ScrollContainer(forms::ScrollContainer),
	#[serde(rename = "Forms$SidebarToggleButton")]
	SidebarToggleButton(forms::SidebarToggleButton), 
	#[serde(rename = "Forms$SimpleMenuBar")]
	SimpleMenuBar(forms::SimpleMenuBar), 
	#[serde(rename = "Forms$SnippetCallWidget")]
	SnippetCallWidget(forms::SnippetCallWidget),
	#[serde(rename = "Forms$StaticImageViewer")]
	StaticImageViewer(forms::StaticImageViewer), 
	#[serde(rename = "Forms$TabControl")]
	TabControl(forms::TabControl),
	#[serde(rename = "Forms$Table")]
	Table(forms::Table),
	#[serde(rename = "Forms$TemplateGrid")]
	TemplateGrid(forms::TemplateGrid),
	#[serde(rename = "Forms$TextArea")]
	TextArea(forms::TextArea),
	#[serde(rename = "Forms$TextBox")]
	TextBox(forms::TextBox),
	#[serde(rename = "Forms$Title")]
	Title(forms::Title),
	#[serde(rename = "Forms$ValidationMessage")]
	ValidationMessage(forms::ValidationMessage),


	// Buttons?
	#[serde(rename = "Forms$ComparisonSearchField")]
	ComparisonSearchField(forms::ComparisonSearchField),
	#[serde(rename = "Forms$DataGridSelectButton")]
	DataGridSelectButton(forms::DataGridSelectButton),
	#[serde(rename = "Forms$DropDownSearchField")]
	DropDownSearchField(forms::DropDownSearchField),
	#[serde(rename = "Forms$GridActionButton")]
	GridActionButton(forms::GridActionButton),
	#[serde(rename = "Forms$GridNewButton")]
	GridNewButton(forms::GridNewButton),
	#[serde(rename = "Forms$GridSearchButton")]
	GridSearchButton(forms::GridSearchButton),
	#[serde(rename = "Forms$GridSelectAllButton")]
	GridSelectAllButton(forms::GridSelectAllButton),
	#[serde(rename = "Forms$RangeSearchField")]
	RangeSearchField(forms::RangeSearchField),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum ButtonAction {
	#[serde(rename = "Forms$FormAction")]
	FormAction(forms::FormAction),
	#[serde(rename = "Forms$OpenLinkClientAction")]
	OpenLinkClientAction(forms::OpenLinkClientAction),
	#[serde(rename = "Forms$CancelChangesClientAction")]
	CancelChangesClientAction(forms::CancelChangesClientAction),
	#[serde(rename = "Forms$DeleteClientAction")]
	DeleteClientAction(forms::DeleteClientAction),
	#[serde(rename = "Forms$SaveChangesClientAction")]
	SaveChangesClientAction(forms::SaveChangesClientAction),
	#[serde(rename = "Forms$MicroflowAction")]
	MicroflowAction(forms::MicroflowAction),
	#[serde(rename = "Forms$NoAction")]
	NoAction(forms::NoAction),
	#[serde(rename = "Forms$CreateObjectClientAction")]
	CreateObjectClientAction(forms::CreateObjectClientAction),
	#[serde(rename = "Forms$CallNanoflowClientAction")]
	CallNanoflowClientAction(forms::CallNanoflowClientAction),
	#[serde(rename = "Forms$ClosePageClientAction")]
	ClosePageClientAction(forms::ClosePageClientAction),
	#[serde(rename = "Forms$SignOutClientAction")]
	SignOutClientAction(forms::SignOutClientAction)
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum OnAction {
	#[serde(rename = "Forms$MicroflowAction")]
	MicroflowAction(forms::MicroflowAction),
	#[serde(rename = "Forms$NoAction")]
	NoAction(forms::NoAction),
	#[serde(rename = "Forms$SaveChangesClientAction")]
	SaveChangesClientAction(forms::SaveChangesClientAction),
	#[serde(rename = "Forms$CallNanoflowClientAction")]
	CallNanoflowClientAction(forms::CallNanoflowClientAction),
	#[serde(rename = "Forms$SignOutClientAction")]
	SignOutClientAction(forms::SignOutClientAction),
	#[serde(rename = "Forms$CancelChangesClientAction")]
	CancelChangesClientAction(forms::CancelChangesClientAction),
	#[serde(rename = "Forms$ClosePageClientAction")]
	ClosePageClientAction(forms::ClosePageClientAction),
	#[serde(rename = "Forms$FormAction")]
	FormAction(forms::FormAction),
	#[serde(rename = "Forms$DeleteClientAction")]
	DeleteClientAction(forms::DeleteClientAction),
	#[serde(rename = "Forms$OpenLinkClientAction")]
	OpenLinkClientAction(forms::OpenLinkClientAction),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum DataSource {
	#[serde(rename = "Forms$AssociationSource")]
	AssociationSource(forms::AssociationSource),
	#[serde(rename = "Forms$DataViewSource")]
	DataViewSource(forms::DataViewSource),
	#[serde(rename = "Forms$GridXPathSource")]
	GridXPathSource(forms::GridXPathSource),
	#[serde(rename = "Forms$ListenTargetSource")]
	ListenTargetSource(forms::ListenTargetSource),
	#[serde(rename = "Forms$ListViewXPathSource")]
	ListViewXPathSource(forms::ListViewXPathSource),
	#[serde(rename = "Forms$MicroflowSource")]
	MicroflowSource(forms::MicroflowSource),
	#[serde(rename = "Forms$NanoflowSource")]
	NanoflowSource(forms::NanoflowSource),
	#[serde(rename = "Forms$NewGridDatabaseSource")]
	NewGridDatabaseSource(forms::NewGridDatabaseSource),
	#[serde(rename = "Forms$NewListViewDatabaseSource")]
	NewListViewDatabaseSource(forms::NewListViewDatabaseSource),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum MenuSource {
	#[serde(rename = "Forms$NavigationSource")]
	NavigationSource(forms::NavigationSource),
	#[serde(rename = "Forms$MenuDocumentSource")]
	MenuDocumentSource(forms::MenuDocumentSource),
}


#[derive(Serialize, Deserialize)]
pub struct ActionButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	action: ButtonAction,
	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AriaRole")]
	aria_role: String,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct Appearance {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Class")]
	class: String,
	#[serde(rename = "DesignProperties", deserialize_with = "deserialize_settings")]
	design_properties: Vec<forms::DesignPropertyValue>,
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
	entity_ref: Option<domain_models::IndirectEntityRef>,
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
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<forms::ActionButton>,
}

#[derive(Serialize, Deserialize)]
pub struct CallNanoflowClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ConfirmationInfo")]
	confirmation_info: Option<Empty>,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<UnknownType>,
	#[serde(rename = "ProgressBar")]
	progress_bar: String,
	#[serde(rename = "ProgressMessage")]
	progress_message: Option<Empty>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelPosition")]
	label_position: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "NativeRenderMode")]
	native_render_mode: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: OnAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: OnAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Option<forms::WidgetValidation>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientTemplate {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Fallback")]
	fallback: Option<texts::Text>,
	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<forms::ClientTemplateParameter>,
	#[serde(rename = "Template")]
	template: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientTemplateParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Option<forms::FormattingInfo>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
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
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "Caption")]
	caption: Option<texts::Text>,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operator")]
	operator: String,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalEditabilitySettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Conditions", deserialize_with = "deserialize_settings")]
	conditions: Vec<enumerations::Condition>,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalVisibilitySettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Conditions", deserialize_with = "deserialize_settings")]
	conditions: Vec<enumerations::Condition>,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "IgnoreSecurity")]
	ignore_security: bool,
	#[serde(rename = "ModuleRoles", deserialize_with = "deserialize_settings")]
	module_roles: Vec<String>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfirmationInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "CancelButtonCaption")]
	cancel_button_caption: Option<texts::Text>,
	#[serde(rename = "ProceedButtonCaption")]
	proceed_button_caption: Option<texts::Text>,
	#[serde(rename = "Question")]
	question: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateObjectClientAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "EntityRef")]
	entity_ref: Option<domain_models::DirectEntityRef>,
	#[serde(rename = "NumberOfPagesToClose2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "PageSettings")]
	page_settings: Option<forms::FormSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct DataGrid {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Columns", deserialize_with = "deserialize_settings")]
	columns: Vec<forms::DataGridColumn>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "ControlBar")]
	control_bar: Option<forms::GridControlBar>,
	#[serde(rename = "DataSource")]
	data_source: DataSource,
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
	aggregate_caption: Option<texts::Text>,
	#[serde(rename = "AggregateFunction")]
	aggregate_function: String,
	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "Caption")]
	caption: Option<texts::Text>,
	#[serde(rename = "Editable")]
	editable: bool,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Option<forms::FormattingInfo>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ShowTooltip")]
	show_tooltip: bool,
	#[serde(rename = "WidthValue")]
	width_value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DataGridExportToCSVButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "DecimalSeparator")]
	decimal_separator: String,
	#[serde(rename = "Delimiter")]
	delimiter: String,
	#[serde(rename = "GenerateExcelHint")]
	generate_excel_hint: bool,
	#[serde(rename = "GroupSeparator")]
	group_separator: String,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "MaxNumberOfRows")]
	max_number_of_rows: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
	#[serde(rename = "UseGridDateFormat")]
	use_grid_date_format: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DataGridExportToExcelButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: forms::Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: forms::ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "MaxNumberOfRows")]
	max_number_of_rows: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: texts::Text,
	#[serde(rename = "UseExcelDateType")]
	use_excel_date_type: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DataGridSelectButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct DataView {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "DataSource")]
	data_source: DataSource,
	#[serde(rename = "Editable")]
	editable: bool,
	#[serde(rename = "FooterWidgets", deserialize_with = "deserialize_settings")]
	footer_widgets: Vec<Widgets>,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NoEntityMessage")]
	no_entity_message: Option<texts::Text>,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ShowFooter")]
	show_footer: bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct DataViewSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: Option<domain_models::EntityRef>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Option<forms::FormattingInfo>,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: OnAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: OnAction,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Option<forms::WidgetValidation>,
}

#[derive(Serialize, Deserialize)]
pub struct DbTableCell {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "IsHeader")]
	is_header: bool,
	#[serde(rename = "LeftColumnIndex")]
	left_column_index: i64,
	#[serde(rename = "TopRowIndex")]
	top_row_index: i64,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "OnClickAction")]
	on_click_action: OnAction,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "ScreenReaderHidden")]
	screen_reader_hidden: bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct DropDown {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "EmptyOptionCaption")]
	empty_option_caption: Option<texts::Text>,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: OnAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: OnAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<forms::PageVariable>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Option<forms::WidgetValidation>,
}

#[derive(Serialize, Deserialize)]
pub struct DropDownSearchField {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowMultiSelect")]
	allow_multi_select: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "Caption")]
	caption: Option<texts::Text>,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operator")]
	operator: String,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Content")]
	content: Option<forms::ClientTemplate>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "MaxFileSize")]
	max_file_size: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
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
	form_settings: Option<forms::FormSettings>,
	#[serde(rename = "NumberOfPagesToClose2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "PagesForSpecializations", deserialize_with = "deserialize_settings")]
	pages_for_specializations: Vec<forms::FormForSpecialization>,
}

#[derive(Serialize, Deserialize)]
pub struct FormCallArgument {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct FormForSpecialization {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "FormSettings")]
	form_settings: Option<forms::FormSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct FormSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<forms::PageParameterMapping>,
	#[serde(rename = "TitleOverride")]
	title_override: Option<microflows::TextTemplate>,
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
	action: OnAction,
	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "MaintainSelectionAfterMicroflow")]
	maintain_selection_after_microflow: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct GridControlBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DefaultButtonPointer")]
	default_button_pointer: Uuid,
	#[serde(rename = "NewButtons", deserialize_with = "deserialize_settings")]
	new_buttons: Vec<Widgets>, // Vec<forms::GridSelectAllButton, forms::GridActionButton, forms::GridSearchButton, forms::GridNewButton, forms::DataGridSelectButton>,
}

#[derive(Serialize, Deserialize)]
pub struct GridNewButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "EditLocation")]
	edit_location: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "FormSettings")]
	form_settings: Option<forms::FormSettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct GridSearchButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct GridSelectAllButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "SelectionType")]
	selection_type: String,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
}

#[derive(Serialize, Deserialize)]
pub struct GridSortBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "SortItems", deserialize_with = "deserialize_settings")]
	sort_items: Vec<forms::GridSortItem>,
}

#[derive(Serialize, Deserialize)]
pub struct GridSortItem {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "SortOrder")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
pub struct GridXPathSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: Option<domain_models::EntityRef>,
	#[serde(rename = "SearchBar")]
	search_bar: Option<forms::SearchBar>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Collapsible")]
	collapsible: String,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "HeaderMode")]
	header_mode: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "LeftWidgets", deserialize_with = "deserialize_settings")]
	left_widgets: Vec<Widgets>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RightWidgets", deserialize_with = "deserialize_settings")]
	right_widgets: Vec<Widgets>,
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
	alternative_text: Option<forms::ClientTemplate>,
	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ClickAction")]
	click_action: OnAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "DataSource")]
	data_source: Option<forms::ImageViewerSource>,
	#[serde(rename = "DefaultImage")]
	default_image: String,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "HeightUnit")]
	height_unit: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
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
	entity_ref: Option<domain_models::DirectEntityRef>,
}

#[derive(Serialize, Deserialize)]
pub struct InputReferenceSetSelector {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "PopupFormSettings")]
	popup_form_settings: Option<forms::FormSettings>,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SelectorSource")]
	selector_source: Option<forms::SelectorXPathSource>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Caption")]
	caption: Option<texts::Text>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum LayoutContent {
	#[serde(rename = "Forms$WebLayoutContent")]
	WebLayoutContent(forms::WebLayoutContent), 
	#[serde(rename = "Forms$NativeLayoutContent")]
	NativeLayoutContent(forms::NativeLayoutContent),
}

#[derive(Serialize, Deserialize)]
pub struct Layout {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Content")]
	content: LayoutContent,
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

	#[serde(rename = "Arguments", deserialize_with = "deserialize_settings")]
	arguments: Vec<forms::FormCallArgument>,
	#[serde(rename = "Form")]
	form: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGrid {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Rows", deserialize_with = "deserialize_settings")]
	rows: Vec<forms::LayoutGridRow>,
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
	appearance: Option<forms::Appearance>,
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
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutGridRow {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Columns", deserialize_with = "deserialize_settings")]
	columns: Vec<forms::LayoutGridColumn>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ClickAction")]
	click_action: OnAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "DataSource")]
	data_source: DataSource,
	#[serde(rename = "Editable")]
	editable: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfColumns")]
	number_of_columns: i64,
	#[serde(rename = "PageSize")]
	page_size: i64,
	#[serde(rename = "PullDownAction")]
	pull_down_action: Option<forms::NoAction>,
	#[serde(rename = "ScrollDirection")]
	scroll_direction: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Templates", deserialize_with = "deserialize_settings")]
	templates: Vec<UnknownType>,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct ListViewSearch {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "SearchRefs", deserialize_with = "deserialize_settings")]
	search_refs: Vec<domain_models::AttributeRef>,
}

#[derive(Serialize, Deserialize)]
pub struct ListViewXPathSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "EntityRef")]
	entity_ref: Option<domain_models::EntityRef>,
	#[serde(rename = "Search")]
	search: Option<forms::ListViewSearch>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
	#[serde(rename = "ValidationMessageWidget")]
	validation_message_widget: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginIdTextBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Label")]
	label: Option<texts::Text>,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MenuBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "MenuSource")]
	menu_source: MenuSource,
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
	microflow_settings: Option<forms::MicroflowSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: Option<forms::PageVariable>,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Asynchronous")]
	asynchronous: bool,
	#[serde(rename = "ConfirmationInfo")]
	confirmation_info: Option<Empty>,
	#[serde(rename = "FormValidations")]
	form_validations: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<forms::MicroflowParameterMapping>,
	#[serde(rename = "ProgressBar")]
	progress_bar: String,
	#[serde(rename = "ProgressMessage")]
	progress_message: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "MicroflowSettings")]
	microflow_settings: Option<forms::MicroflowSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: Option<forms::PageVariable>,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<forms::NanoflowParameterMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct NativeLayoutContent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "LayoutType")]
	layout_type: String,
	#[serde(rename = "RightHeaderPlaceholder")]
	right_header_placeholder: Option<Empty>,
	#[serde(rename = "ShowBottomBar")]
	show_bottom_bar: bool,
	#[serde(rename = "Sidebar")]
	sidebar: bool,
	#[serde(rename = "SidebarWidgets", deserialize_with = "deserialize_settings")]
	sidebar_widgets: Vec<forms::Placeholder>,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "MenuSource")]
	menu_source: MenuSource,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NewGridDatabaseSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DatabaseConstraints", deserialize_with = "deserialize_settings")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "EntityRef")]
	entity_ref: Option<domain_models::EntityRef>,
	#[serde(rename = "SearchBar")]
	search_bar: Option<forms::SearchBar>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
}

#[derive(Serialize, Deserialize)]
pub struct NewListViewDatabaseSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DatabaseConstraints", deserialize_with = "deserialize_settings")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "EntityRef")]
	entity_ref: Option<domain_models::EntityRef>,
	#[serde(rename = "Search")]
	search: Option<forms::ListViewSearch>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
}

#[derive(Serialize, Deserialize)]
pub struct NewSelectorDatabaseSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DatabaseConstraints", deserialize_with = "deserialize_settings")]
	database_constraints: Vec<UnknownType>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
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
	address: Option<forms::StaticOrDynamicString>,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: bool,
	#[serde(rename = "LinkType")]
	link_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowedModuleRoles", deserialize_with = "deserialize_settings")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
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
	form_call: Option<forms::LayoutCall>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<forms::PageParameter>,
	#[serde(rename = "PopupCloseAction")]
	popup_close_action: String,
	#[serde(rename = "PopupHeight")]
	popup_height: i64,
	#[serde(rename = "PopupResizable")]
	popup_resizable: bool,
	#[serde(rename = "PopupWidth")]
	popup_width: i64,
	#[serde(rename = "Title")]
	title: Option<texts::Text>,
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
	parameter_type: Option<data_types::ObjectType>,
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
	variable: Option<forms::PageVariable>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Label")]
	label: Option<texts::Text>,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Placeholder {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: OnAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: OnAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "RenderHorizontal")]
	render_horizontal: bool,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<forms::PageVariable>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Option<forms::WidgetValidation>,
}

#[derive(Serialize, Deserialize)]
pub struct RangeSearchField {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Caption")]
	caption: Option<texts::Text>,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "IncludeLower")]
	include_lower: bool,
	#[serde(rename = "IncludeUpper")]
	include_upper: bool,
	#[serde(rename = "LowerBoundRef")]
	lower_bound_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "UpperBoundRef")]
	upper_bound_ref: Option<domain_models::AttributeRef>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum SelectorSource {
	#[serde(rename = "Forms$SelectorMicroflowSource")]
	SelectorMicroflowSource(forms::SelectorMicroflowSource),
	#[serde(rename = "Forms$NewSelectorDatabaseSource")]
	NewSelectorDatabaseSource(forms::NewSelectorDatabaseSource),
	#[serde(rename = "Forms$SelectorXPathSource")]
	SelectorXPathSource(forms::SelectorXPathSource),
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSelector {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "EmptyOptionCaption")]
	empty_option_caption: Option<texts::Text>,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Option<forms::FormattingInfo>,
	#[serde(rename = "GotoFormSettings")]
	goto_form_settings: Option<forms::FormSettings>,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "PopupFormSettings")]
	popup_form_settings: Option<forms::FormSettings>,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SelectorSource")]
	selector_source: SelectorSource,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Option<forms::WidgetValidation>,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceSetSelector {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Columns", deserialize_with = "deserialize_settings")]
	columns: Vec<forms::DataGridColumn>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "ConstrainedByRefs", deserialize_with = "deserialize_settings")]
	constrained_by_refs: Vec<UnknownType>,
	#[serde(rename = "ControlBar")]
	control_bar: Option<forms::GridControlBar>,
	#[serde(rename = "DataSource")]
	data_source: Option<forms::ReferenceSetSource>,
	#[serde(rename = "DefaultButtonTrigger")]
	default_button_trigger: String,
	#[serde(rename = "IsControlBarVisible")]
	is_control_bar_visible: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfRows")]
	number_of_rows: i64,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
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
	entity_ref: Option<domain_models::IndirectEntityRef>,
	#[serde(rename = "SearchBar")]
	search_bar: Option<forms::SearchBar>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Bottom")]
	bottom: Option<forms::ScrollContainerRegion>,
	#[serde(rename = "CenterRegion")]
	center_region: Option<forms::ScrollContainerRegion>,
	#[serde(rename = "LayoutMode")]
	layout_mode: String,
	#[serde(rename = "Left")]
	left: Option<Empty>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeHideScrollbars")]
	native_hide_scrollbars: bool,
	#[serde(rename = "Right")]
	right: Option<Empty>,
	#[serde(rename = "ScrollBehavior")]
	scroll_behavior: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Top")]
	top: Option<forms::ScrollContainerRegion>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Size")]
	size: i64,
	#[serde(rename = "SizeMode")]
	size_mode: String,
	#[serde(rename = "ToggleMode")]
	toggle_mode: String,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchBar {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "NewButtons", deserialize_with = "deserialize_settings")]
	new_buttons: Vec<Widgets>,
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
	data_source_microflow_settings: Option<forms::MicroflowSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorXPathSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ConstrainedByRefs", deserialize_with = "deserialize_settings")]
	constrained_by_refs: Vec<domain_models::IndirectEntityRef>,
	#[serde(rename = "SortBar")]
	sort_bar: Option<forms::GridSortBar>,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum Icon {
	#[serde(rename = "Forms$GlyphIcon")]
	GlyphIcon(forms::GlyphIcon),
	#[serde(rename = "Forms$IconCollectionIcon")]
	IconCollectionIcon(forms::IconCollectionIcon),
	#[serde(rename = "Forms$ImageIcon")]
	ImageIcon(forms::ImageIcon),
}

#[derive(Serialize, Deserialize)]
pub struct SidebarToggleButton {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Option<forms::ClientTemplate>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Icon")]
	icon: Option<Icon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: Option<texts::Text>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "MenuSource")]
	menu_source: MenuSource,
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
	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<forms::SnippetParameter>,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetCallWidget {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "FormCall")]
	form_call: Option<forms::SnippetCall>,
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
	parameter_type: Option<data_types::ObjectType>,
}

#[derive(Serialize, Deserialize)]
pub struct StaticImageViewer {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AlternativeText")]
	alternative_text: Option<forms::ClientTemplate>,
	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ClickAction")]
	click_action: OnAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "HeightUnit")]
	height_unit: String,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
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
	attribute_ref: Option<domain_models::AttributeRef>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "DefaultPagePointer")]
	default_page_pointer: Uuid,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TabPages", deserialize_with = "deserialize_settings")]
	tab_pages: Vec<forms::TabPage>,
}

#[derive(Serialize, Deserialize)]
pub struct TabPage {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Badge")]
	badge: Option<Empty>,
	#[serde(rename = "Caption")]
	caption: Option<texts::Text>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RefreshOnShow")]
	refresh_on_show: bool,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "Cells", deserialize_with = "deserialize_settings")]
	cells: Vec<forms::DbTableCell>,
	#[serde(rename = "ColumnWidths", deserialize_with = "deserialize_settings")]
	column_widths: Vec<forms::TableColumn>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Rows", deserialize_with = "deserialize_settings")]
	rows: Vec<forms::TableRow>,
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
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateGrid {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Contents")]
	contents: Option<forms::TemplateGridContents>,
	#[serde(rename = "ControlBar")]
	control_bar: Option<forms::GridControlBar>,
	#[serde(rename = "DataSource")]
	data_source: DataSource,
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

	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
}

#[derive(Serialize, Deserialize)]
pub struct TextArea {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "Autocomplete")]
	autocomplete: bool,
	#[serde(rename = "AutoFocus")]
	auto_focus: bool,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "CounterMessage")]
	counter_message: Option<texts::Text>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<Empty>,
	#[serde(rename = "MaxLengthCode")]
	max_length_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "NumberOfLines")]
	number_of_lines: i64,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: OnAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: OnAction,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<Empty>,
	#[serde(rename = "SubmitBehaviour")]
	submit_behaviour: String,
	#[serde(rename = "SubmitOnInputDelay")]
	submit_on_input_delay: i64,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TextTooLongMessage")]
	text_too_long_message: Option<texts::Text>,
	#[serde(rename = "Validation")]
	validation: Option<forms::WidgetValidation>,
}

#[derive(Serialize, Deserialize)]
pub struct TextBox {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "AriaRequired")]
	aria_required: bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
	#[serde(rename = "Autocomplete")]
	autocomplete: bool,
	#[serde(rename = "AutocompletePurpose")]
	autocomplete_purpose: String,
	#[serde(rename = "AutoFocus")]
	auto_focus: bool,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Option<ConditionalEditabilitySettings>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Option<forms::FormattingInfo>,
	#[serde(rename = "InputMask")]
	input_mask: String,
	#[serde(rename = "IsPasswordBox")]
	is_password_box: bool,
	#[serde(rename = "KeyboardType")]
	keyboard_type: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Option<forms::ClientTemplate>,
	#[serde(rename = "MaxLengthCode")]
	max_length_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "OnChangeAction")]
	on_change_action: OnAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: OnAction,
	#[serde(rename = "OnEnterKeyPressAction")]
	on_enter_key_press_action: OnAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: OnAction,
	#[serde(rename = "Placeholder")]
	placeholder: Option<texts::Text>,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Option<ClientTemplate>,
	#[serde(rename = "SourceVariable")]
	source_variable: Option<forms::PageVariable>,
	#[serde(rename = "SubmitBehaviour")]
	submit_behaviour: String,
	#[serde(rename = "SubmitOnInputDelay")]
	submit_on_input_delay: i64,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Option<forms::WidgetValidation>,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Option<ConditionalVisibilitySettings>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Option<Empty>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationMessage {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Appearance")]
	appearance: Option<forms::Appearance>,
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
	layout_call: Option<LayoutCall>, // Todo: get rid of Option?
	#[serde(rename = "LayoutType")]
	layout_type: String,
	#[serde(rename = "Widgets", deserialize_with = "deserialize_settings")]
	widgets: Vec<Widgets>,
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
	#[serde(rename = "ThemeModuleOrder", deserialize_with = "deserialize_settings")]
	theme_module_order: Vec<settings::ThemeModuleEntry>,
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
	message: Option<texts::Text>,
}

