#[derive(Serialize, Deserialize)]
struct SidebarToggleButton {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Icon")]
	icon: Forms$GlyphIcon,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct ImageIcon {
	#[serde(rename = "Image")]
	image: String,
}

#[derive(Serialize, Deserialize)]
struct NewListViewDatabaseSource {
	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<2, []>,
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$IndirectEntityRef,
	#[serde(rename = "Search")]
	search: Forms$ListViewSearch,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
}

#[derive(Serialize, Deserialize)]
struct GridSelectAllButton {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Icon")]
	icon: Forms$ImageIcon,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "SelectionType")]
	selection_type: String,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct MenuDocumentSource {
	#[serde(rename = "Menu")]
	menu: String,
}

#[derive(Serialize, Deserialize)]
struct Label {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct PasswordTextBox {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Label")]
	label: Texts$Text,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct Appearance {
	#[serde(rename = "Class")]
	class: String,
	#[serde(rename = "DesignProperties")]
	design_properties: Vec<3, []>,
	#[serde(rename = "DynamicClasses")]
	dynamic_classes: String,
	#[serde(rename = "Style")]
	style: String,
}

#[derive(Serialize, Deserialize)]
struct LayoutCall {
	#[serde(rename = "Arguments")]
	arguments: Vec<2, ["Forms$FormCallArgument"]>,
	#[serde(rename = "Form")]
	form: String,
}

#[derive(Serialize, Deserialize)]
struct GridXPathSource {
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "SearchBar")]
	search_bar: Forms$SearchBar,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
struct Snippet {
	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<2, ["Forms$SnippetParameter"]>,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$LayoutGrid"]>,
}

#[derive(Serialize, Deserialize)]
struct Table {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Cells")]
	cells: Vec<2, ["Forms$DbTableCell"]>,
	#[serde(rename = "ColumnWidths")]
	column_widths: Vec<2, ["Forms$TableColumn"]>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Rows")]
	rows: Vec<2, ["Forms$TableRow"]>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
struct GridControlBar {
	#[serde(rename = "DefaultButtonPointer")]
	default_button_pointer: Binary,
	#[serde(rename = "NewButtons")]
	new_buttons: Vec<3, ["Forms$GridSearchButton", "Forms$GridActionButton"]>,
}

#[derive(Serialize, Deserialize)]
struct ConditionalVisibilitySettings {
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Conditions")]
	conditions: Vec<2, ["Enumerations$Condition"]>,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "IgnoreSecurity")]
	ignore_security: Bool,
	#[serde(rename = "ModuleRoles")]
	module_roles: Vec<1, []>,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
}

#[derive(Serialize, Deserialize)]
struct NewSelectorDatabaseSource {
	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<2, []>,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
}

#[derive(Serialize, Deserialize)]
struct SelectorXPathSource {
	#[serde(rename = "ConstrainedByRefs")]
	constrained_by_refs: Vec<2, []>,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
struct SimpleMenuBar {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "MenuSource")]
	menu_source: Forms$NavigationSource,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Orientation")]
	orientation: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct PageParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: Forms$PageVariable,
}

#[derive(Serialize, Deserialize)]
struct GroupBox {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "Collapsible")]
	collapsible: String,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "HeaderMode")]
	header_mode: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$TabControl"]>,
}

#[derive(Serialize, Deserialize)]
struct MicroflowParameterMapping {
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: Forms$PageVariable,
}

#[derive(Serialize, Deserialize)]
struct NavigationSource {
	#[serde(rename = "NavigationProfile")]
	navigation_profile: String,
}

#[derive(Serialize, Deserialize)]
struct Header {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "LeftWidgets")]
	left_widgets: Vec<2, ["Forms$Placeholder"]>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RightWidgets")]
	right_widgets: Vec<2, ["Forms$Placeholder"]>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct NewGridDatabaseSource {
	#[serde(rename = "DatabaseConstraints")]
	database_constraints: Vec<2, []>,
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "SearchBar")]
	search_bar: Forms$SearchBar,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
}

#[derive(Serialize, Deserialize)]
struct GridSortItem {
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "SortOrder")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
struct TemplateGridContents {
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$StaticImageViewer", "Forms$DivContainer"]>,
}

#[derive(Serialize, Deserialize)]
struct ConditionalEditabilitySettings {
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Conditions")]
	conditions: Vec<2, []>,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
}

#[derive(Serialize, Deserialize)]
struct SaveChangesClientAction {
	#[serde(rename = "ClosePage")]
	close_page: Bool,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
	#[serde(rename = "SyncAutomatically")]
	sync_automatically: Bool,
}

#[derive(Serialize, Deserialize)]
struct ReferenceSetSelector {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Columns")]
	columns: Vec<2, ["Forms$DataGridColumn"]>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "ConstrainedByRefs")]
	constrained_by_refs: Vec<2, []>,
	#[serde(rename = "ControlBar")]
	control_bar: Forms$GridControlBar,
	#[serde(rename = "DataSource")]
	data_source: Forms$ReferenceSetSource,
	#[serde(rename = "DefaultButtonTrigger")]
	default_button_trigger: String,
	#[serde(rename = "IsControlBarVisible")]
	is_control_bar_visible: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfRows")]
	number_of_rows: i64,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "RefreshTime")]
	refresh_time: i64,
	#[serde(rename = "SelectableXPathConstraint")]
	selectable_x_path_constraint: String,
	#[serde(rename = "SelectFirst")]
	select_first: Bool,
	#[serde(rename = "SelectionMode")]
	selection_mode: String,
	#[serde(rename = "ShowEmptyRows")]
	show_empty_rows: Bool,
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
struct FormForSpecialization {
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "FormSettings")]
	form_settings: Forms$FormSettings,
}

#[derive(Serialize, Deserialize)]
struct DeleteClientAction {
	#[serde(rename = "ClosePage")]
	close_page: Bool,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
}

#[derive(Serialize, Deserialize)]
struct WidgetValidation {
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "Message")]
	message: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct ClientTemplate {
	#[serde(rename = "Fallback")]
	fallback: Texts$Text,
	#[serde(rename = "Parameters")]
	parameters: Vec<2, []>,
	#[serde(rename = "Template")]
	template: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct TextBox {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: Bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "Autocomplete")]
	autocomplete: Bool,
	#[serde(rename = "AutocompletePurpose")]
	autocomplete_purpose: String,
	#[serde(rename = "AutoFocus")]
	auto_focus: Bool,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Forms$ConditionalEditabilitySettings,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Forms$FormattingInfo,
	#[serde(rename = "InputMask")]
	input_mask: String,
	#[serde(rename = "IsPasswordBox")]
	is_password_box: Bool,
	#[serde(rename = "KeyboardType")]
	keyboard_type: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "MaxLengthCode")]
	max_length_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$MicroflowAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: Forms$NoAction,
	#[serde(rename = "OnEnterKeyPressAction")]
	on_enter_key_press_action: Forms$NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: Forms$NoAction,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "SourceVariable")]
	source_variable: Forms$PageVariable,
	#[serde(rename = "SubmitBehaviour")]
	submit_behaviour: String,
	#[serde(rename = "SubmitOnInputDelay")]
	submit_on_input_delay: i64,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Forms$WidgetValidation,
}

#[derive(Serialize, Deserialize)]
struct PageParameter {
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: DataTypes$ObjectType,
}

#[derive(Serialize, Deserialize)]
struct LoginIdTextBox {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Label")]
	label: Texts$Text,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct RangeSearchField {
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "IncludeLower")]
	include_lower: Bool,
	#[serde(rename = "IncludeUpper")]
	include_upper: Bool,
	#[serde(rename = "LowerBoundRef")]
	lower_bound_ref: DomainModels$AttributeRef,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "UpperBoundRef")]
	upper_bound_ref: DomainModels$AttributeRef,
}

#[derive(Serialize, Deserialize)]
struct WebUIProjectSettingsPart {
	#[serde(rename = "EnableDownloadResources")]
	enable_download_resources: Bool,
	#[serde(rename = "EnableMicroflowReachabilityAnalysis")]
	enable_microflow_reachability_analysis: Bool,
	#[serde(rename = "EnableWidgetBundling")]
	enable_widget_bundling: Bool,
	#[serde(rename = "ThemeModuleOrder")]
	theme_module_order: Vec<3, ["Settings$ThemeModuleEntry"]>,
	#[serde(rename = "UseOptimizedClient")]
	use_optimized_client: String,
}

#[derive(Serialize, Deserialize)]
struct GridSortBar {
	#[serde(rename = "SortItems")]
	sort_items: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct FormattingInfo {
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DateFormat")]
	date_format: String,
	#[serde(rename = "DecimalPrecision")]
	decimal_precision: i64,
	#[serde(rename = "EnumFormat")]
	enum_format: String,
	#[serde(rename = "GroupDigits")]
	group_digits: Bool,
}

#[derive(Serialize, Deserialize)]
struct SearchBar {
	#[serde(rename = "NewButtons")]
	new_buttons: Vec<3, ["Forms$ComparisonSearchField"]>,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "WaitForSearch")]
	wait_for_search: Bool,
}

#[derive(Serialize, Deserialize)]
struct LayoutGridRow {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Columns")]
	columns: Vec<2, ["Forms$LayoutGridColumn"]>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "HorizontalAlignment")]
	horizontal_alignment: String,
	#[serde(rename = "SpacingBetweenColumns")]
	spacing_between_columns: Bool,
	#[serde(rename = "VerticalAlignment")]
	vertical_alignment: String,
}

#[derive(Serialize, Deserialize)]
struct SnippetParameter {
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: DataTypes$ObjectType,
}

#[derive(Serialize, Deserialize)]
struct GlyphIcon {
	#[serde(rename = "Code")]
	code: i64,
}

#[derive(Serialize, Deserialize)]
struct LayoutGridColumn {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
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
	widgets: Vec<2, ["Forms$DataView"]>,
}

#[derive(Serialize, Deserialize)]
struct DataGridColumn {
	#[serde(rename = "AggregateCaption")]
	aggregate_caption: Texts$Text,
	#[serde(rename = "AggregateFunction")]
	aggregate_function: String,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "Editable")]
	editable: Bool,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Forms$FormattingInfo,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ShowTooltip")]
	show_tooltip: Bool,
	#[serde(rename = "WidthValue")]
	width_value: i64,
}

#[derive(Serialize, Deserialize)]
struct ReferenceSelector {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "EmptyOptionCaption")]
	empty_option_caption: Texts$Text,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Forms$FormattingInfo,
	#[serde(rename = "GotoFormSettings")]
	goto_form_settings: Forms$FormSettings,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "PopupFormSettings")]
	popup_form_settings: Forms$FormSettings,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "SelectorSource")]
	selector_source: Forms$NewSelectorDatabaseSource,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Forms$WidgetValidation,
}

#[derive(Serialize, Deserialize)]
struct Title {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct ImageViewerSource {
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
}

#[derive(Serialize, Deserialize)]
struct GridActionButton {
	#[serde(rename = "Action")]
	action: Forms$MicroflowAction,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "MaintainSelectionAfterMicroflow")]
	maintain_selection_after_microflow: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct ListViewXPathSource {
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "Search")]
	search: Forms$ListViewSearch,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
struct BuildingBlock {
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
	excluded: Bool,
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
	widgets: Vec<2, ["Forms$ActionButton"]>,
}

#[derive(Serialize, Deserialize)]
struct NanoflowParameterMapping {
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Variable")]
	variable: Forms$PageVariable,
}

#[derive(Serialize, Deserialize)]
struct DataViewSource {
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "PageParameter")]
	page_parameter: String,
	#[serde(rename = "SnippetParameter")]
	snippet_parameter: String,
}

#[derive(Serialize, Deserialize)]
struct ComparisonSearchField {
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operator")]
	operator: String,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct StaticOrDynamicString {
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "IsDynamic")]
	is_dynamic: Bool,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
struct ClosePageClientAction {
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
struct ClientTemplateParameter {
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Forms$FormattingInfo,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
}

#[derive(Serialize, Deserialize)]
struct CancelChangesClientAction {
	#[serde(rename = "ClosePage")]
	close_page: Bool,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
}

#[derive(Serialize, Deserialize)]
struct DropDown {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: Bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "EmptyOptionCaption")]
	empty_option_caption: Texts$Text,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: Forms$NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: Forms$NoAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "SourceVariable")]
	source_variable: Forms$PageVariable,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Forms$WidgetValidation,
}

#[derive(Serialize, Deserialize)]
struct ConfirmationInfo {
	#[serde(rename = "CancelButtonCaption")]
	cancel_button_caption: Texts$Text,
	#[serde(rename = "ProceedButtonCaption")]
	proceed_button_caption: Texts$Text,
	#[serde(rename = "Question")]
	question: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct ListViewSearch {
	#[serde(rename = "SearchRefs")]
	search_refs: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct MicroflowSettings {
	#[serde(rename = "Asynchronous")]
	asynchronous: Bool,
	#[serde(rename = "ConfirmationInfo")]
	confirmation_info: Null,
	#[serde(rename = "FormValidations")]
	form_validations: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
	#[serde(rename = "ProgressBar")]
	progress_bar: String,
	#[serde(rename = "ProgressMessage")]
	progress_message: Null,
}

#[derive(Serialize, Deserialize)]
struct ValidationMessage {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct ScrollContainer {
	#[serde(rename = "Alignment")]
	alignment: String,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Bottom")]
	bottom: Forms$ScrollContainerRegion,
	#[serde(rename = "CenterRegion")]
	center_region: Forms$ScrollContainerRegion,
	#[serde(rename = "LayoutMode")]
	layout_mode: String,
	#[serde(rename = "Left")]
	left: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeHideScrollbars")]
	native_hide_scrollbars: Bool,
	#[serde(rename = "Right")]
	right: Null,
	#[serde(rename = "ScrollBehavior")]
	scroll_behavior: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Top")]
	top: Forms$ScrollContainerRegion,
	#[serde(rename = "Width")]
	width: i64,
	#[serde(rename = "WidthMode")]
	width_mode: String,
}

#[derive(Serialize, Deserialize)]
struct NanoflowSource {
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct Placeholder {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct DropDownSearchField {
	#[serde(rename = "AllowMultiSelect")]
	allow_multi_select: Bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "CustomDateFormat")]
	custom_date_format: String,
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operator")]
	operator: String,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "XPathConstraint")]
	x_path_constraint: String,
}

#[derive(Serialize, Deserialize)]
struct InputReferenceSetSelector {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "PopupFormSettings")]
	popup_form_settings: Forms$FormSettings,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "SelectorSource")]
	selector_source: Forms$SelectorXPathSource,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct OpenLinkClientAction {
	#[serde(rename = "Address")]
	address: Forms$StaticOrDynamicString,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
	#[serde(rename = "LinkType")]
	link_type: String,
}

#[derive(Serialize, Deserialize)]
struct FileManager {
	#[serde(rename = "AllowedExtensions")]
	allowed_extensions: String,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "MaxFileSize")]
	max_file_size: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "ShowFileInBrowser")]
	show_file_in_browser: Bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct FormSettings {
	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
	#[serde(rename = "TitleOverride")]
	title_override: Null,
}

#[derive(Serialize, Deserialize)]
struct DynamicText {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Content")]
	content: Forms$ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "NativeTextStyle")]
	native_text_style: String,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct TableRow {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
}

#[derive(Serialize, Deserialize)]
struct AssociationSource {
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$IndirectEntityRef,
}

#[derive(Serialize, Deserialize)]
struct CheckBox {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: Bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Forms$ConditionalVisibilitySettings,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelPosition")]
	label_position: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "NativeRenderMode")]
	native_render_mode: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: Forms$NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: Forms$NoAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Forms$WidgetValidation,
}

#[derive(Serialize, Deserialize)]
struct LayoutGrid {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Rows")]
	rows: Vec<2, ["Forms$LayoutGridRow"]>,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Width")]
	width: String,
}

#[derive(Serialize, Deserialize)]
struct CallNanoflowClientAction {
	#[serde(rename = "ConfirmationInfo")]
	confirmation_info: Null,
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
	#[serde(rename = "ProgressBar")]
	progress_bar: String,
	#[serde(rename = "ProgressMessage")]
	progress_message: Null,
}

#[derive(Serialize, Deserialize)]
struct ScrollContainerRegion {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Size")]
	size: i64,
	#[serde(rename = "SizeMode")]
	size_mode: String,
	#[serde(rename = "ToggleMode")]
	toggle_mode: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct NativeLayoutContent {
	#[serde(rename = "LayoutType")]
	layout_type: String,
	#[serde(rename = "RightHeaderPlaceholder")]
	right_header_placeholder: Null,
	#[serde(rename = "ShowBottomBar")]
	show_bottom_bar: Bool,
	#[serde(rename = "Sidebar")]
	sidebar: Bool,
	#[serde(rename = "SidebarWidgets")]
	sidebar_widgets: Vec<2, []>,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$Placeholder"]>,
}

#[derive(Serialize, Deserialize)]
struct ActionButton {
	#[serde(rename = "Action")]
	action: Forms$MicroflowAction,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AriaRole")]
	aria_role: String,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Forms$ConditionalVisibilitySettings,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct TabPage {
	#[serde(rename = "Badge")]
	badge: Null,
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RefreshOnShow")]
	refresh_on_show: Bool,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$LayoutGrid"]>,
}

#[derive(Serialize, Deserialize)]
struct DatePicker {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: Bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "FormattingInfo")]
	formatting_info: Forms$FormattingInfo,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: Forms$NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: Forms$NoAction,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Forms$WidgetValidation,
}

#[derive(Serialize, Deserialize)]
struct DataGridSelectButton {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct ListenTargetSource {
	#[serde(rename = "ListenTarget")]
	listen_target: String,
}

#[derive(Serialize, Deserialize)]
struct NavigationTree {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "MenuSource")]
	menu_source: Forms$NavigationSource,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct FormCallArgument {
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$LayoutGrid"]>,
}

#[derive(Serialize, Deserialize)]
struct StaticImageViewer {
	#[serde(rename = "AlternativeText")]
	alternative_text: Forms$ClientTemplate,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ClickAction")]
	click_action: Forms$NoAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "HeightUnit")]
	height_unit: String,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "Responsive")]
	responsive: Bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Width")]
	width: i64,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
struct MicroflowAction {
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
	#[serde(rename = "MicroflowSettings")]
	microflow_settings: Forms$MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
struct ReferenceSetSource {
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$IndirectEntityRef,
	#[serde(rename = "SearchBar")]
	search_bar: Forms$SearchBar,
	#[serde(rename = "SortBar")]
	sort_bar: Forms$GridSortBar,
}

#[derive(Serialize, Deserialize)]
struct PageVariable {
	#[serde(rename = "PageParameter")]
	page_parameter: String,
	#[serde(rename = "SnippetParameter")]
	snippet_parameter: String,
	#[serde(rename = "UseAllPages")]
	use_all_pages: Bool,
	#[serde(rename = "Widget")]
	widget: String,
}

#[derive(Serialize, Deserialize)]
struct MicroflowSource {
	#[serde(rename = "MicroflowSettings")]
	microflow_settings: Forms$MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
struct NoAction {
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
}

#[derive(Serialize, Deserialize)]
struct SnippetCallWidget {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "FormCall")]
	form_call: Forms$SnippetCall,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct ImageViewer {
	#[serde(rename = "AlternativeText")]
	alternative_text: Forms$ClientTemplate,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ClickAction")]
	click_action: Forms$CallNanoflowClientAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "DataSource")]
	data_source: Forms$ImageViewerSource,
	#[serde(rename = "DefaultImage")]
	default_image: String,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "HeightUnit")]
	height_unit: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "OnClickEnlarge")]
	on_click_enlarge: Bool,
	#[serde(rename = "Responsive")]
	responsive: Bool,
	#[serde(rename = "ShowAsThumbnail")]
	show_as_thumbnail: Bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Width")]
	width: i64,
	#[serde(rename = "WidthUnit")]
	width_unit: String,
}

#[derive(Serialize, Deserialize)]
struct LoginButton {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RenderType")]
	render_type: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
	#[serde(rename = "ValidationMessageWidget")]
	validation_message_widget: String,
}

#[derive(Serialize, Deserialize)]
struct DbTableCell {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "IsHeader")]
	is_header: Bool,
	#[serde(rename = "LeftColumnIndex")]
	left_column_index: i64,
	#[serde(rename = "TopRowIndex")]
	top_row_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$ActionButton"]>,
	#[serde(rename = "Width")]
	width: i64,
}

#[derive(Serialize, Deserialize)]
struct SelectorMicroflowSource {
	#[serde(rename = "DataSourceMicroflowSettings")]
	data_source_microflow_settings: Forms$MicroflowSettings,
}

#[derive(Serialize, Deserialize)]
struct IconCollectionIcon {
	#[serde(rename = "Image")]
	image: String,
}

#[derive(Serialize, Deserialize)]
struct ListView {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ClickAction")]
	click_action: Forms$NoAction,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "DataSource")]
	data_source: Forms$MicroflowSource,
	#[serde(rename = "Editable")]
	editable: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfColumns")]
	number_of_columns: i64,
	#[serde(rename = "PageSize")]
	page_size: i64,
	#[serde(rename = "PullDownAction")]
	pull_down_action: Forms$NoAction,
	#[serde(rename = "ScrollDirection")]
	scroll_direction: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Templates")]
	templates: Vec<2, []>,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$LayoutGrid"]>,
}

#[derive(Serialize, Deserialize)]
struct TextArea {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: Bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "Autocomplete")]
	autocomplete: Bool,
	#[serde(rename = "AutoFocus")]
	auto_focus: Bool,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "CounterMessage")]
	counter_message: Texts$Text,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Null,
	#[serde(rename = "MaxLengthCode")]
	max_length_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "NumberOfLines")]
	number_of_lines: i64,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: Forms$NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: Forms$NoAction,
	#[serde(rename = "Placeholder")]
	placeholder: Texts$Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Forms$ClientTemplate,
	#[serde(rename = "SourceVariable")]
	source_variable: Null,
	#[serde(rename = "SubmitBehaviour")]
	submit_behaviour: String,
	#[serde(rename = "SubmitOnInputDelay")]
	submit_on_input_delay: i64,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TextTooLongMessage")]
	text_too_long_message: Texts$Text,
	#[serde(rename = "Validation")]
	validation: Forms$WidgetValidation,
}

#[derive(Serialize, Deserialize)]
struct DataView {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "DataSource")]
	data_source: Forms$DataViewSource,
	#[serde(rename = "Editable")]
	editable: Bool,
	#[serde(rename = "FooterWidgets")]
	footer_widgets: Vec<2, ["Forms$ActionButton"]>,
	#[serde(rename = "LabelWidth")]
	label_width: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NoEntityMessage")]
	no_entity_message: Texts$Text,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "ShowFooter")]
	show_footer: Bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$DynamicText", "Forms$TextBox", "Forms$RadioButtonGroup"]>,
}

#[derive(Serialize, Deserialize)]
struct DivContainer {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NativeAccessibilitySettings")]
	native_accessibility_settings: Null,
	#[serde(rename = "OnClickAction")]
	on_click_action: Forms$NoAction,
	#[serde(rename = "RenderMode")]
	render_mode: String,
	#[serde(rename = "ScreenReaderHidden")]
	screen_reader_hidden: Bool,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$DynamicText"]>,
}

#[derive(Serialize, Deserialize)]
struct FormAction {
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
	#[serde(rename = "FormSettings")]
	form_settings: Forms$FormSettings,
	#[serde(rename = "NumberOfPagesToClose2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "PagesForSpecializations")]
	pages_for_specializations: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct DesignPropertyValue {
	#[serde(rename = "BooleanValue")]
	boolean_value: Bool,
	#[serde(rename = "Key")]
	key: String,
	#[serde(rename = "StringValue")]
	string_value: String,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct TemplateGrid {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Contents")]
	contents: Forms$TemplateGridContents,
	#[serde(rename = "ControlBar")]
	control_bar: Forms$GridControlBar,
	#[serde(rename = "DataSource")]
	data_source: Forms$MicroflowSource,
	#[serde(rename = "DefaultButtonTrigger")]
	default_button_trigger: String,
	#[serde(rename = "IsControlBarVisible")]
	is_control_bar_visible: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfColumns")]
	number_of_columns: i64,
	#[serde(rename = "NumberOfRows")]
	number_of_rows: i64,
	#[serde(rename = "RefreshTime")]
	refresh_time: i64,
	#[serde(rename = "SelectFirst")]
	select_first: Bool,
	#[serde(rename = "SelectionMode")]
	selection_mode: String,
	#[serde(rename = "ShowPagingBar")]
	show_paging_bar: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct WebLayoutContent {
	#[serde(rename = "LayoutCall")]
	layout_call: Null,
	#[serde(rename = "LayoutType")]
	layout_type: String,
	#[serde(rename = "Widgets")]
	widgets: Vec<2, ["Forms$Placeholder"]>,
}

#[derive(Serialize, Deserialize)]
struct TabControl {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "DefaultPagePointer")]
	default_page_pointer: Binary,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "TabPages")]
	tab_pages: Vec<3, ["Forms$TabPage"]>,
}

#[derive(Serialize, Deserialize)]
struct Page {
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<1, ["String"]>,
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "FormCall")]
	form_call: Forms$LayoutCall,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<3, ["Forms$PageParameter"]>,
	#[serde(rename = "PopupCloseAction")]
	popup_close_action: String,
	#[serde(rename = "PopupHeight")]
	popup_height: i64,
	#[serde(rename = "PopupResizable")]
	popup_resizable: Bool,
	#[serde(rename = "PopupWidth")]
	popup_width: i64,
	#[serde(rename = "Title")]
	title: Texts$Text,
	#[serde(rename = "Url")]
	url: String,
}

#[derive(Serialize, Deserialize)]
struct GridSearchButton {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct SnippetCall {
	#[serde(rename = "Form")]
	form: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct GridNewButton {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "ButtonStyle")]
	button_style: String,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "EditLocation")]
	edit_location: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "FormSettings")]
	form_settings: Forms$FormSettings,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tooltip")]
	tooltip: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct RadioButtonGroup {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "AriaRequired")]
	aria_required: Bool,
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "ConditionalEditabilitySettings")]
	conditional_editability_settings: Null,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "Editable")]
	editable: String,
	#[serde(rename = "LabelTemplate")]
	label_template: Forms$ClientTemplate,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OnChangeAction")]
	on_change_action: Forms$NoAction,
	#[serde(rename = "OnEnterAction")]
	on_enter_action: Forms$NoAction,
	#[serde(rename = "OnLeaveAction")]
	on_leave_action: Forms$NoAction,
	#[serde(rename = "ReadOnlyStyle")]
	read_only_style: String,
	#[serde(rename = "RenderHorizontal")]
	render_horizontal: Bool,
	#[serde(rename = "ScreenReaderLabel")]
	screen_reader_label: Null,
	#[serde(rename = "SourceVariable")]
	source_variable: Forms$PageVariable,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
	#[serde(rename = "Validation")]
	validation: Forms$WidgetValidation,
}

#[derive(Serialize, Deserialize)]
struct DataGrid {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "CaptionTemplate")]
	caption_template: Forms$ClientTemplate,
	#[serde(rename = "Columns")]
	columns: Vec<2, ["Forms$DataGridColumn"]>,
	#[serde(rename = "ConditionalVisibilitySettings")]
	conditional_visibility_settings: Null,
	#[serde(rename = "ControlBar")]
	control_bar: Forms$GridControlBar,
	#[serde(rename = "DataSource")]
	data_source: Forms$GridXPathSource,
	#[serde(rename = "DefaultButtonTrigger")]
	default_button_trigger: String,
	#[serde(rename = "IsControlBarVisible")]
	is_control_bar_visible: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NumberOfRows")]
	number_of_rows: i64,
	#[serde(rename = "RefreshTime")]
	refresh_time: i64,
	#[serde(rename = "SelectFirst")]
	select_first: Bool,
	#[serde(rename = "SelectionMode")]
	selection_mode: String,
	#[serde(rename = "ShowEmptyRows")]
	show_empty_rows: Bool,
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
struct Layout {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "CanvasHeight")]
	canvas_height: i64,
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Content")]
	content: Forms$WebLayoutContent,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct MenuBar {
	#[serde(rename = "Appearance")]
	appearance: Forms$Appearance,
	#[serde(rename = "MenuSource")]
	menu_source: Forms$NavigationSource,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "TabIndex")]
	tab_index: i64,
}

#[derive(Serialize, Deserialize)]
struct CreateObjectClientAction {
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "NumberOfPagesToClose2")]
	number_of_pages_to_close_2: String,
	#[serde(rename = "PageSettings")]
	page_settings: Forms$FormSettings,
}

#[derive(Serialize, Deserialize)]
struct TableColumn {
	#[serde(rename = "Value")]
	value: i64,
}

#[derive(Serialize, Deserialize)]
struct SignOutClientAction {
	#[serde(rename = "DisabledDuringExecution")]
	disabled_during_execution: Bool,
}

