use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CustomRange {
	#[serde(rename = "LimitExpression")]
	limit_expression: String,
	#[serde(rename = "OffsetExpression")]
	offset_expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct EndEvent {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "ReturnValue")]
	return_value: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct RuleSplitCondition {
	#[serde(rename = "RuleCall")]
	rule_call: Microflows$RuleCall,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptActionParameterMapping {
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "ParameterValue")]
	parameter_value: Microflows$BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
pub struct ContinueEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportXmlAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: bool,
	#[serde(rename = "ResultHandling")]
	result_handling: Microflows$ResultHandling,
	#[serde(rename = "XmlDocumentVariableName")]
	xml_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IncrementCounterMeterAction {
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tags")]
	tags: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationFeedbackAction {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FeedbackTemplate")]
	feedback_template: Microflows$TextTemplate,
	#[serde(rename = "ValidationVariableName")]
	validation_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListOperationsAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NewOperation")]
	new_operation: Microflows$Head,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct BinaryRequestHandling {
	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct SortingsList {
	#[serde(rename = "Sortings")]
	sortings: Vec<Microflows$RetrieveSorting>,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveSorting {
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "SortOrder")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sort {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "Sortings")]
	sortings: Microflows$SortingsList,
}

#[derive(Serialize, Deserialize)]
pub struct CreateVariableAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "InitialValue")]
	initial_value: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
	#[serde(rename = "VariableType")]
	variable_type: DataTypes$DateTimeType,
}

#[derive(Serialize, Deserialize)]
pub struct JavaActionCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "JavaAction")]
	java_action: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<Microflows$JavaActionParameterMapping>,
	#[serde(rename = "QueueSettings")]
	queue_settings: Null,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct IterableList {
	#[serde(rename = "ListVariableName")]
	list_variable_name: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpHeaderEntry {
	#[serde(rename = "Key")]
	key: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomRequestHandling {
	#[serde(rename = "Template")]
	template: Microflows$StringTemplate,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptActionCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "JavaScriptAction")]
	java_script_action: String,
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<Microflows$JavaScriptActionParameterMapping>,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WebServiceOperationSimpleParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "IsChecked")]
	is_checked: bool,
	#[serde(rename = "ParameterName")]
	parameter_name: String,
	#[serde(rename = "ParameterPath")]
	parameter_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExclusiveMerge {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
	#[serde(rename = "ApplyEntityAccess")]
	apply_entity_access: bool,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<Microflows$SequenceFlow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: DataTypes$BooleanType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: Microflows$MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
pub struct BreakEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct RestCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "ErrorResultHandlingType")]
	error_result_handling_type: String,
	#[serde(rename = "HttpConfiguration")]
	http_configuration: Microflows$HttpConfiguration,
	#[serde(rename = "ProxyConfiguration")]
	proxy_configuration: Null,
	#[serde(rename = "RequestHandling")]
	request_handling: Microflows$CustomRequestHandling,
	#[serde(rename = "RequestHandlingType")]
	request_handling_type: String,
	#[serde(rename = "RequestProxyType")]
	request_proxy_type: String,
	#[serde(rename = "ResultHandling")]
	result_handling: Microflows$ResultHandling,
	#[serde(rename = "ResultHandlingType")]
	result_handling_type: String,
	#[serde(rename = "TimeOutExpression")]
	time_out_expression: String,
	#[serde(rename = "UseRequestTimeOut")]
	use_request_time_out: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Union {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tail {
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SequenceFlow {
	#[serde(rename = "DestinationBezierVector")]
	destination_bezier_vector: String,
	#[serde(rename = "DestinationConnectionIndex")]
	destination_connection_index: i64,
	#[serde(rename = "DestinationPointer")]
	destination_pointer: Binary,
	#[serde(rename = "IsErrorHandler")]
	is_error_handler: bool,
	#[serde(rename = "NewCaseValue")]
	new_case_value: Microflows$NoCase,
	#[serde(rename = "OriginBezierVector")]
	origin_bezier_vector: String,
	#[serde(rename = "OriginConnectionIndex")]
	origin_connection_index: i64,
	#[serde(rename = "OriginPointer")]
	origin_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct AggregateAction {
	#[serde(rename = "AggregateFunction")]
	aggregate_function: String,
	#[serde(rename = "AggregateVariableName")]
	aggregate_variable_name: String,
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAction {
	#[serde(rename = "DeleteVariableName")]
	delete_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeActionItem {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExclusiveSplit {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
	#[serde(rename = "SplitCondition")]
	split_condition: Microflows$ExpressionSplitCondition,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentTemplateParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "WidgetName")]
	widget_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Nanoflow {
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<Microflows$SequenceFlow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: DataTypes$VoidType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: Microflows$MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
pub struct HttpConfiguration {
	#[serde(rename = "ClientCertificate")]
	client_certificate: String,
	#[serde(rename = "CustomLocation")]
	custom_location: String,
	#[serde(rename = "CustomLocationTemplate")]
	custom_location_template: Microflows$StringTemplate,
	#[serde(rename = "HttpAuthenticationPassword")]
	http_authentication_password: String,
	#[serde(rename = "HttpAuthenticationUserName")]
	http_authentication_user_name: String,
	#[serde(rename = "HttpHeaderEntries")]
	http_header_entries: Vec<Microflows$HttpHeaderEntry>,
	#[serde(rename = "HttpMethod")]
	http_method: String,
	#[serde(rename = "OverrideLocation")]
	override_location: bool,
	#[serde(rename = "UseHttpAuthentication")]
	use_http_authentication: bool,
}

#[derive(Serialize, Deserialize)]
pub struct InheritanceCase {
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameter {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "HasVariableNameBeenChanged")]
	has_variable_name_been_changed: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
	#[serde(rename = "VariableType")]
	variable_type: DataTypes$ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct AnnotationFlow {
	#[serde(rename = "DestinationBezierVector")]
	destination_bezier_vector: String,
	#[serde(rename = "DestinationConnectionIndex")]
	destination_connection_index: i64,
	#[serde(rename = "DestinationPointer")]
	destination_pointer: Binary,
	#[serde(rename = "OriginBezierVector")]
	origin_bezier_vector: String,
	#[serde(rename = "OriginConnectionIndex")]
	origin_connection_index: i64,
	#[serde(rename = "OriginPointer")]
	origin_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct Microflow {
	#[serde(rename = "AllowConcurrentExecution")]
	allow_concurrent_execution: bool,
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "ApplyEntityAccess")]
	apply_entity_access: bool,
	#[serde(rename = "ConcurrencyErrorMicroflow")]
	concurrency_error_microflow: String,
	#[serde(rename = "ConcurrenyErrorMessage")]
	concurreny_error_message: Texts$Text,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<Microflows$SequenceFlow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowActionInfo")]
	microflow_action_info: Null,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: DataTypes$VoidType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: Microflows$MicroflowObjectCollection,
	#[serde(rename = "WorkflowActionInfo")]
	workflow_action_info: Null,
}

#[derive(Serialize, Deserialize)]
pub struct RuleCallParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subtract {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitAction {
	#[serde(rename = "CommitVariableName")]
	commit_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
	#[serde(rename = "WithEvents")]
	with_events: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Contains {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SynchronizeAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "VariableNames")]
	variable_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CloseFormAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExportXmlAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: bool,
	#[serde(rename = "OutputMethod")]
	output_method: ExportXmlAction$FileDocumentExport,
	#[serde(rename = "ResultHandling")]
	result_handling: Microflows$MappingRequestHandling,
}

#[derive(Serialize, Deserialize)]
pub struct TextTemplate {
	#[serde(rename = "Parameters")]
	parameters: Vec<2, []>,
	#[serde(rename = "Text")]
	text: Texts$Text,
}

#[derive(Serialize, Deserialize)]
pub struct FilterByExpression {
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Find {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FindByExpression {
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConstantRange {
	#[serde(rename = "SingleObject")]
	single_object: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowObjectCollection {
	#[serde(rename = "Objects")]
	objects: Vec<3, ["Microflows$StartEvent", "Microflows$MicroflowParameter", "Microflows$ActionActivity", "Microflows$ExclusiveSplit", "Microflows$EndEvent"]>,
}

#[derive(Serialize, Deserialize)]
pub struct MappingRequestHandling {
	#[serde(rename = "ContentType")]
	content_type: String,
	#[serde(rename = "MappingId")]
	mapping_id: String,
	#[serde(rename = "MappingVariableName")]
	mapping_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WhileLoopCondition {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "WhileExpression")]
	while_expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct Intersect {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResultHandling {
	#[serde(rename = "Bind")]
	bind: bool,
	#[serde(rename = "ImportMappingCall")]
	import_mapping_call: Microflows$ImportMappingCall,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "VariableType")]
	variable_type: DataTypes$ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct ShowMessageAction {
	#[serde(rename = "Blocking")]
	blocking: bool,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Template")]
	template: Microflows$TextTemplate,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportMappingCall {
	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "ContentType")]
	content_type: String,
	#[serde(rename = "ForceSingleOccurrence")]
	force_single_occurrence: bool,
	#[serde(rename = "ObjectHandlingBackup")]
	object_handling_backup: String,
	#[serde(rename = "ParameterVariableName")]
	parameter_variable_name: String,
	#[serde(rename = "Range")]
	range: Microflows$ConstantRange,
	#[serde(rename = "ReturnValueMapping")]
	return_value_mapping: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameterValue {
	#[serde(rename = "Microflow")]
	microflow: String,
}

#[derive(Serialize, Deserialize)]
pub struct ActionActivity {
	#[serde(rename = "Action")]
	action: Microflows$MicroflowCallAction,
	#[serde(rename = "AutoGenerateCaption")]
	auto_generate_caption: bool,
	#[serde(rename = "BackgroundColor")]
	background_color: String,
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Disabled")]
	disabled: bool,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateChangeAction {
	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Items")]
	items: Vec<2, []>,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RuleCall {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<Microflows$RuleCallParameterMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct CastAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "MicroflowCall")]
	microflow_call: Microflows$MicroflowCall,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JavaActionParameterMapping {
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Value")]
	value: Microflows$BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleRequestHandling {
	#[serde(rename = "NullValueOption")]
	null_value_option: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<Microflows$WebServiceOperationSimpleParameterMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateDocumentAction {
	#[serde(rename = "DocumentTemplate")]
	document_template: String,
	#[serde(rename = "DocumentType")]
	document_type: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FileVariableName")]
	file_variable_name: String,
	#[serde(rename = "LanguageSetting")]
	language_setting: String,
	#[serde(rename = "LanguageVariableName")]
	language_variable_name: String,
	#[serde(rename = "MarginBottomInInch")]
	margin_bottom_in_inch: String,
	#[serde(rename = "MarginLeftInInch")]
	margin_left_in_inch: String,
	#[serde(rename = "MarginRightInInch")]
	margin_right_in_inch: String,
	#[serde(rename = "MarginTopInInch")]
	margin_top_in_inch: String,
	#[serde(rename = "OverrideBottomMargin")]
	override_bottom_margin: bool,
	#[serde(rename = "OverrideLeftMargin")]
	override_left_margin: bool,
	#[serde(rename = "OverrideRightMargin")]
	override_right_margin: bool,
	#[serde(rename = "OverrideTopMargin")]
	override_top_margin: bool,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseRetrieveSource {
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "NewSortings")]
	new_sortings: Microflows$SortingsList,
	#[serde(rename = "Range")]
	range: Microflows$CustomRange,
	#[serde(rename = "XpathConstraint")]
	xpath_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct InheritanceSplit {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
	#[serde(rename = "SplitVariableName")]
	split_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NanoflowCall")]
	nanoflow_call: Microflows$NanoflowCall,
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "RetrieveSource")]
	retrieve_source: Microflows$DatabaseRetrieveSource,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateParameter {
	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCallParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeListAction {
	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringTemplate {
	#[serde(rename = "Parameters")]
	parameters: Vec<Microflows$TemplateParameter>,
	#[serde(rename = "Text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeVariableAction {
	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct RollbackAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
	#[serde(rename = "RollbackVariableName")]
	rollback_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoopedActivity {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "LoopSource")]
	loop_source: Microflows$IterableList,
	#[serde(rename = "ObjectCollection")]
	object_collection: Microflows$MicroflowObjectCollection,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct Head {
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExpressionSplitCondition {
	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShowHomePageAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCall {
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<Microflows$NanoflowCallParameterMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct StartEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCall {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<Microflows$MicroflowCallParameterMapping>,
	#[serde(rename = "QueueSettings")]
	queue_settings: Null,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeAction {
	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Items")]
	items: Vec<Microflows$ChangeActionItem>,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogMessageAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IncludeLatestStackTrace")]
	include_latest_stack_trace: bool,
	#[serde(rename = "Level")]
	level: String,
	#[serde(rename = "MessageTemplate")]
	message_template: Microflows$StringTemplate,
	#[serde(rename = "Node")]
	node: String,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadFileAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FileDocumentVariableName")]
	file_document_variable_name: String,
	#[serde(rename = "ShowFileInBrowser")]
	show_file_in_browser: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EntityTypeCodeActionParameterValue {
	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssociationRetrieveSource {
	#[serde(rename = "AssociationId")]
	association_id: String,
	#[serde(rename = "StartVariableName")]
	start_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateListAction {
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Annotation {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoCase {
}

#[derive(Serialize, Deserialize)]
pub struct BasicCodeActionParameterValue {
	#[serde(rename = "Argument")]
	argument: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShowFormAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FormSettings")]
	form_settings: Forms$FormSettings,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct CallWebServiceAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "HttpConfiguration")]
	http_configuration: Microflows$HttpConfiguration,
	#[serde(rename = "ImportedService")]
	imported_service: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: bool,
	#[serde(rename = "NewResultHandling")]
	new_result_handling: Microflows$ResultHandling,
	#[serde(rename = "OperationName")]
	operation_name: String,
	#[serde(rename = "ProxyConfiguration")]
	proxy_configuration: Null,
	#[serde(rename = "RequestBodyHandling")]
	request_body_handling: Microflows$SimpleRequestHandling,
	#[serde(rename = "RequestHeaderHandling")]
	request_header_handling: Microflows$SimpleRequestHandling,
	#[serde(rename = "RequestProxyType")]
	request_proxy_type: String,
	#[serde(rename = "ServiceName")]
	service_name: String,
	#[serde(rename = "TimeOutExpression")]
	time_out_expression: String,
	#[serde(rename = "UseRequestTimeOut")]
	use_request_time_out: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationCase {
	#[serde(rename = "Value")]
	value: String,
}

