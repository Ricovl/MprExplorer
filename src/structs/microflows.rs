use bson::Binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

pub enum Flow {
	AnnotationFlow(microflows::AnnotationFlow), 
	SequenceFlow(microflows::SequenceFlow),
}

pub enum MicroFlowObject {
	MicroflowParameter(microflows::MicroflowParameter),
	EndEvent(microflows::EndEvent),
	StartEvent(microflows::StartEvent),
	ContinueEvent(microflows::ContinueEvent),
	LoopedActivity(microflows::LoopedActivity),
	ExclusiveMerge(microflows::ExclusiveMerge),
	InheritanceSplit(microflows::InheritanceSplit),
	ErrorEvent(microflows::ErrorEvent),
	ExclusiveSplit(microflows::ExclusiveSplit),
	Annotation(microflows::Annotation),
	ActionActivity(microflows::ActionActivity),
	BreakEvent(microflows::BreakEvent),
}


#[derive(Serialize, Deserialize)]
pub struct ActionActivity {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	action: microflows::RetrieveAction,
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
pub struct AggregateAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct Annotation {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnnotationFlow {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct AssociationRetrieveSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AssociationId")]
	association_id: String,
	#[serde(rename = "StartVariableName")]
	start_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct BasicCodeActionParameterValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Argument")]
	argument: String,
}

#[derive(Serialize, Deserialize)]
pub struct BinaryRequestHandling {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct BreakEvent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct CallWebServiceAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "HttpConfiguration")]
	http_configuration: microflows::HttpConfiguration,
	#[serde(rename = "ImportedService")]
	imported_service: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: bool,
	#[serde(rename = "NewResultHandling")]
	new_result_handling: microflows::ResultHandling,
	#[serde(rename = "OperationName")]
	operation_name: String,
	#[serde(rename = "ProxyConfiguration")]
	proxy_configuration: Empty,
	#[serde(rename = "RequestBodyHandling")]
	request_body_handling: microflows::SimpleRequestHandling,
	#[serde(rename = "RequestHeaderHandling")]
	request_header_handling: microflows::SimpleRequestHandling,
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
pub struct CastAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Items")]
	items: Vec<microflows::ChangeActionItem, >,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeActionItem {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeListAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeVariableAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct CloseFormAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct ConstantRange {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "SingleObject")]
	single_object: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Contains {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContinueEvent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateChangeAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Items")]
	items: Vec<microflows::ChangeActionItem, >,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateListAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateVariableAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "InitialValue")]
	initial_value: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
	#[serde(rename = "VariableType")]
	variable_type: data_types::StringType,
}

#[derive(Serialize, Deserialize)]
pub struct CustomRange {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "LimitExpression")]
	limit_expression: String,
	#[serde(rename = "OffsetExpression")]
	offset_expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomRequestHandling {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Template")]
	template: microflows::StringTemplate,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseRetrieveSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "NewSortings")]
	new_sortings: microflows::SortingsList,
	#[serde(rename = "Range")]
	range: microflows::ConstantRange,
	#[serde(rename = "XpathConstraint")]
	xpath_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DeleteVariableName")]
	delete_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentTemplateParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "WidgetName")]
	widget_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadFileAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FileDocumentVariableName")]
	file_document_variable_name: String,
	#[serde(rename = "ShowFileInBrowser")]
	show_file_in_browser: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EndEvent {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct EntityTypeCodeActionParameterValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationCase {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorEvent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExclusiveMerge {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExclusiveSplit {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
	split_condition: microflows::ExpressionSplitCondition,
}

#[derive(Serialize, Deserialize)]
pub struct ExportXmlAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: bool,
	#[serde(rename = "OutputMethod")]
	output_method: export_xml_action::StringExport,
	#[serde(rename = "ResultHandling")]
	result_handling: microflows::MappingRequestHandling,
}

#[derive(Serialize, Deserialize)]
pub struct ExpressionSplitCondition {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct FilterByExpression {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Find {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateDocumentAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
	parameter_mappings: Vec<microflows::DocumentTemplateParameterMapping, >,
}

#[derive(Serialize, Deserialize)]
pub struct Head {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpConfiguration {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ClientCertificate")]
	client_certificate: String,
	#[serde(rename = "CustomLocation")]
	custom_location: String,
	#[serde(rename = "CustomLocationTemplate")]
	custom_location_template: microflows::StringTemplate,
	#[serde(rename = "HttpAuthenticationPassword")]
	http_authentication_password: String,
	#[serde(rename = "HttpAuthenticationUserName")]
	http_authentication_user_name: String,
	#[serde(rename = "HttpHeaderEntries")]
	http_header_entries: Vec<microflows::HttpHeaderEntry, >,
	#[serde(rename = "HttpMethod")]
	http_method: String,
	#[serde(rename = "OverrideLocation")]
	override_location: bool,
	#[serde(rename = "UseHttpAuthentication")]
	use_http_authentication: bool,
}

#[derive(Serialize, Deserialize)]
pub struct HttpHeaderEntry {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Key")]
	key: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportMappingCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
	range: microflows::ConstantRange,
	#[serde(rename = "ReturnValueMapping")]
	return_value_mapping: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportXmlAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: bool,
	#[serde(rename = "ResultHandling")]
	result_handling: microflows::ResultHandling,
	#[serde(rename = "XmlDocumentVariableName")]
	xml_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IncrementCounterMeterAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tags")]
	tags: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct InheritanceCase {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct InheritanceSplit {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct Intersect {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IterableList {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListVariableName")]
	list_variable_name: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct JavaActionCallAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "JavaAction")]
	java_action: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<microflows::JavaActionParameterMapping, >,
	#[serde(rename = "QueueSettings")]
	queue_settings: Empty,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JavaActionParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Value")]
	value: microflows::BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptActionCallAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "JavaScriptAction")]
	java_script_action: String,
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<microflows::JavaScriptActionParameterMapping, >,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptActionParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "ParameterValue")]
	parameter_value: microflows::BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
pub struct ListOperationsAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NewOperation")]
	new_operation: microflows::Head,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogMessageAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IncludeLatestStackTrace")]
	include_latest_stack_trace: bool,
	#[serde(rename = "Level")]
	level: String,
	#[serde(rename = "MessageTemplate")]
	message_template: microflows::StringTemplate,
	#[serde(rename = "Node")]
	node: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoopedActivity {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "LoopSource")]
	loop_source: microflows::IterableList,
	#[serde(rename = "ObjectCollection")]
	object_collection: microflows::MicroflowObjectCollection,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct MappingRequestHandling {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ContentType")]
	content_type: String,
	#[serde(rename = "MappingId")]
	mapping_id: String,
	#[serde(rename = "MappingVariableName")]
	mapping_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Microflow {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowConcurrentExecution")]
	allow_concurrent_execution: bool,
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "ApplyEntityAccess")]
	apply_entity_access: bool,
	#[serde(rename = "ConcurrencyErrorMicroflow")]
	concurrency_error_microflow: String,
	#[serde(rename = "ConcurrenyErrorMessage")]
	concurreny_error_message: texts::Text,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<Flow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowActionInfo")]
	microflow_action_info: Empty,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: data_types::VoidType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: microflows::MicroflowObjectCollection,
	#[serde(rename = "WorkflowActionInfo")]
	workflow_action_info: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<microflows::MicroflowCallParameterMapping>,
	#[serde(rename = "QueueSettings")]
	queue_settings: Empty,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "MicroflowCall")]
	microflow_call: microflows::MicroflowCall,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowObjectCollection {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Objects")]
	objects: Vec<MicroFlowObject>,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
	variable_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameterValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
}

#[derive(Serialize, Deserialize)]
pub struct Nanoflow {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<Flow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: data_types::BooleanType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: microflows::MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<microflows::NanoflowCallParameterMapping, >,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCallAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NanoflowCall")]
	nanoflow_call: microflows::NanoflowCall,
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCallParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoCase {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct RestCallAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "ErrorResultHandlingType")]
	error_result_handling_type: String,
	#[serde(rename = "HttpConfiguration")]
	http_configuration: microflows::HttpConfiguration,
	#[serde(rename = "ProxyConfiguration")]
	proxy_configuration: Empty,
	#[serde(rename = "RequestHandling")]
	request_handling: microflows::BinaryRequestHandling,
	#[serde(rename = "RequestHandlingType")]
	request_handling_type: String,
	#[serde(rename = "RequestProxyType")]
	request_proxy_type: String,
	#[serde(rename = "ResultHandling")]
	result_handling: microflows::ResultHandling,
	#[serde(rename = "ResultHandlingType")]
	result_handling_type: String,
	#[serde(rename = "TimeOutExpression")]
	time_out_expression: String,
	#[serde(rename = "UseRequestTimeOut")]
	use_request_time_out: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ResultHandling {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Bind")]
	bind: bool,
	#[serde(rename = "ImportMappingCall")]
	import_mapping_call: microflows::ImportMappingCall,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "VariableType")]
	variable_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "RetrieveSource")]
	retrieve_source: microflows::AssociationRetrieveSource,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveSorting {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "SortOrder")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
pub struct RollbackAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: bool,
	#[serde(rename = "RollbackVariableName")]
	rollback_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ApplyEntityAccess")]
	apply_entity_access: bool,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<microflows::SequenceFlow, >,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: data_types::BooleanType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: microflows::MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
pub struct RuleCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<microflows::RuleCallParameterMapping, >,
}

#[derive(Serialize, Deserialize)]
pub struct RuleCallParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct RuleSplitCondition {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "RuleCall")]
	rule_call: microflows::RuleCall,
}

#[derive(Serialize, Deserialize)]
pub struct SequenceFlow {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DestinationBezierVector")]
	destination_bezier_vector: String,
	#[serde(rename = "DestinationConnectionIndex")]
	destination_connection_index: i64,
	#[serde(rename = "DestinationPointer")]
	destination_pointer: Binary,
	#[serde(rename = "IsErrorHandler")]
	is_error_handler: bool,
	#[serde(rename = "NewCaseValue")]
	new_case_value: microflows::NoCase,
	#[serde(rename = "OriginBezierVector")]
	origin_bezier_vector: String,
	#[serde(rename = "OriginConnectionIndex")]
	origin_connection_index: i64,
	#[serde(rename = "OriginPointer")]
	origin_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct ShowFormAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FormSettings")]
	form_settings: forms::FormSettings,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShowHomePageAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShowMessageAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Blocking")]
	blocking: bool,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Template")]
	template: microflows::TextTemplate,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleRequestHandling {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "NullValueOption")]
	null_value_option: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<microflows::WebServiceOperationSimpleParameterMapping, >,
}

#[derive(Serialize, Deserialize)]
pub struct Sort {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "Sortings")]
	sortings: microflows::SortingsList,
}

#[derive(Serialize, Deserialize)]
pub struct SortingsList {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Sortings")]
	sortings: Vec<microflows::RetrieveSorting, >,
}

#[derive(Serialize, Deserialize)]
pub struct StartEvent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringTemplate {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameters")]
	parameters: Vec<microflows::TemplateParameter, >,
	#[serde(rename = "Text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subtract {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SynchronizeAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Type")]
	var_type: String,
	#[serde(rename = "VariableNames")]
	variable_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Tail {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct TextTemplate {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameters")]
	parameters: Vec<microflows::TemplateParameter, >,
	#[serde(rename = "Text")]
	text: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct Union {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationFeedbackAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FeedbackTemplate")]
	feedback_template: microflows::TextTemplate,
	#[serde(rename = "ValidationVariableName")]
	validation_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebServiceOperationSimpleParameterMapping {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
pub struct WhileLoopCondition {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "WhileExpression")]
	while_expression: String,
}

