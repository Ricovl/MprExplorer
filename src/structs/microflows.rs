use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct RuleSplitCondition {
	#[serde(rename = "rule_call")]
	rule_call: microflows::RuleCall,
}

#[derive(Serialize, Deserialize)]
pub struct CreateChangeAction {
	#[serde(rename = "commit")]
	commit: String,
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "items")]
	items: Vec<>,
	#[serde(rename = "refresh_in_client")]
	refresh_in_client: bool,
	#[serde(rename = "variable_name")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct JavaActionCallAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "java_action")]
	java_action: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<>,
	#[serde(rename = "queue_settings")]
	queue_settings: Null,
	#[serde(rename = "result_variable_name")]
	result_variable_name: String,
	#[serde(rename = "use_return_variable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BreakEvent {
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct InheritanceCase {
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationCase {
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContinueEvent {
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct RollbackAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "refresh_in_client")]
	refresh_in_client: bool,
	#[serde(rename = "rollback_variable_name")]
	rollback_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowObjectCollection {
	#[serde(rename = "objects")]
	objects: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptActionParameterMapping {
	#[serde(rename = "parameter")]
	parameter: String,
	#[serde(rename = "parameter_value")]
	parameter_value: microflows::BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
pub struct CommitAction {
	#[serde(rename = "commit_variable_name")]
	commit_variable_name: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "refresh_in_client")]
	refresh_in_client: bool,
	#[serde(rename = "with_events")]
	with_events: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameterValue {
	#[serde(rename = "microflow")]
	microflow: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tail {
	#[serde(rename = "list_name")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportMappingCall {
	#[serde(rename = "commit")]
	commit: String,
	#[serde(rename = "content_type")]
	content_type: String,
	#[serde(rename = "force_single_occurrence")]
	force_single_occurrence: bool,
	#[serde(rename = "object_handling_backup")]
	object_handling_backup: String,
	#[serde(rename = "parameter_variable_name")]
	parameter_variable_name: String,
	#[serde(rename = "range")]
	range: microflows::ConstantRange,
	#[serde(rename = "return_value_mapping")]
	return_value_mapping: String,
}

#[derive(Serialize, Deserialize)]
pub struct CloseFormAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "number_of_pages_to_close")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateVariableAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "initial_value")]
	initial_value: String,
	#[serde(rename = "variable_name")]
	variable_name: String,
	#[serde(rename = "variable_type")]
	variable_type: data_types::DateTimeType,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCallParameterMapping {
	#[serde(rename = "argument")]
	argument: String,
	#[serde(rename = "parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeListAction {
	#[serde(rename = "change_variable_name")]
	change_variable_name: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "type")]
	type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct InheritanceSplit {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
	#[serde(rename = "split_variable_name")]
	split_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoopedActivity {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "loop_source")]
	loop_source: microflows::IterableList,
	#[serde(rename = "object_collection")]
	object_collection: microflows::MicroflowObjectCollection,
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeVariableAction {
	#[serde(rename = "change_variable_name")]
	change_variable_name: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentTemplateParameterMapping {
	#[serde(rename = "argument")]
	argument: String,
	#[serde(rename = "widget_name")]
	widget_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FindByExpression {
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "list_name")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Union {
	#[serde(rename = "list_name")]
	list_name: String,
	#[serde(rename = "second_list_or_object_name")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateDocumentAction {
	#[serde(rename = "document_template")]
	document_template: String,
	#[serde(rename = "document_type")]
	document_type: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "file_variable_name")]
	file_variable_name: String,
	#[serde(rename = "language_setting")]
	language_setting: String,
	#[serde(rename = "language_variable_name")]
	language_variable_name: String,
	#[serde(rename = "margin_bottom_in_inch")]
	margin_bottom_in_inch: String,
	#[serde(rename = "margin_left_in_inch")]
	margin_left_in_inch: String,
	#[serde(rename = "margin_right_in_inch")]
	margin_right_in_inch: String,
	#[serde(rename = "margin_top_in_inch")]
	margin_top_in_inch: String,
	#[serde(rename = "override_bottom_margin")]
	override_bottom_margin: bool,
	#[serde(rename = "override_left_margin")]
	override_left_margin: bool,
	#[serde(rename = "override_right_margin")]
	override_right_margin: bool,
	#[serde(rename = "override_top_margin")]
	override_top_margin: bool,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorEvent {
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExclusiveMerge {
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCallAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "nanoflow_call")]
	nanoflow_call: microflows::NanoflowCall,
	#[serde(rename = "output_variable_name")]
	output_variable_name: String,
	#[serde(rename = "use_return_variable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Microflow {
	#[serde(rename = "allow_concurrent_execution")]
	allow_concurrent_execution: bool,
	#[serde(rename = "allowed_module_roles")]
	allowed_module_roles: Vec<>,
	#[serde(rename = "apply_entity_access")]
	apply_entity_access: bool,
	#[serde(rename = "concurrency_error_microflow")]
	concurrency_error_microflow: String,
	#[serde(rename = "concurreny_error_message")]
	concurreny_error_message: texts::Text,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "flows")]
	flows: Vec<>,
	#[serde(rename = "mark_as_used")]
	mark_as_used: bool,
	#[serde(rename = "microflow_action_info")]
	microflow_action_info: Null,
	#[serde(rename = "microflow_return_type")]
	microflow_return_type: data_types::VoidType,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "object_collection")]
	object_collection: microflows::MicroflowObjectCollection,
	#[serde(rename = "workflow_action_info")]
	workflow_action_info: Null,
}

#[derive(Serialize, Deserialize)]
pub struct NoCase {
}

#[derive(Serialize, Deserialize)]
pub struct AssociationRetrieveSource {
	#[serde(rename = "association_id")]
	association_id: String,
	#[serde(rename = "start_variable_name")]
	start_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallParameterMapping {
	#[serde(rename = "argument")]
	argument: String,
	#[serde(rename = "parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "microflow_call")]
	microflow_call: microflows::MicroflowCall,
	#[serde(rename = "result_variable_name")]
	result_variable_name: String,
	#[serde(rename = "use_return_variable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationFeedbackAction {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "feedback_template")]
	feedback_template: microflows::TextTemplate,
	#[serde(rename = "validation_variable_name")]
	validation_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShowFormAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "form_settings")]
	form_settings: forms::FormSettings,
	#[serde(rename = "number_of_pages_to_close")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpHeaderEntry {
	#[serde(rename = "key")]
	key: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptActionCallAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "java_script_action")]
	java_script_action: String,
	#[serde(rename = "output_variable_name")]
	output_variable_name: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<>,
	#[serde(rename = "use_return_variable")]
	use_return_variable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Contains {
	#[serde(rename = "list_name")]
	list_name: String,
	#[serde(rename = "second_list_or_object_name")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SynchronizeAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "type")]
	type: String,
	#[serde(rename = "variable_names")]
	variable_names: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomRange {
	#[serde(rename = "limit_expression")]
	limit_expression: String,
	#[serde(rename = "offset_expression")]
	offset_expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpConfiguration {
	#[serde(rename = "client_certificate")]
	client_certificate: String,
	#[serde(rename = "custom_location")]
	custom_location: String,
	#[serde(rename = "custom_location_template")]
	custom_location_template: microflows::StringTemplate,
	#[serde(rename = "http_authentication_password")]
	http_authentication_password: String,
	#[serde(rename = "http_authentication_user_name")]
	http_authentication_user_name: String,
	#[serde(rename = "http_header_entries")]
	http_header_entries: Vec<>,
	#[serde(rename = "http_method")]
	http_method: String,
	#[serde(rename = "override_location")]
	override_location: bool,
	#[serde(rename = "use_http_authentication")]
	use_http_authentication: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConstantRange {
	#[serde(rename = "single_object")]
	single_object: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BinaryRequestHandling {
	#[serde(rename = "expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct CastAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "variable_name")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExportXmlAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "is_validation_required")]
	is_validation_required: bool,
	#[serde(rename = "output_method")]
	output_method: export_xml_action::FileDocumentExport,
	#[serde(rename = "result_handling")]
	result_handling: microflows::MappingRequestHandling,
}

#[derive(Serialize, Deserialize)]
pub struct Annotation {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExpressionSplitCondition {
	#[serde(rename = "expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct MappingRequestHandling {
	#[serde(rename = "content_type")]
	content_type: String,
	#[serde(rename = "mapping_id")]
	mapping_id: String,
	#[serde(rename = "mapping_variable_name")]
	mapping_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowParameter {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "has_variable_name_been_changed")]
	has_variable_name_been_changed: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
	#[serde(rename = "variable_type")]
	variable_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeAction {
	#[serde(rename = "change_variable_name")]
	change_variable_name: String,
	#[serde(rename = "commit")]
	commit: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "items")]
	items: Vec<>,
	#[serde(rename = "refresh_in_client")]
	refresh_in_client: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCall {
	#[serde(rename = "nanoflow")]
	nanoflow: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct Subtract {
	#[serde(rename = "list_name")]
	list_name: String,
	#[serde(rename = "second_list_or_object_name")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AggregateAction {
	#[serde(rename = "aggregate_function")]
	aggregate_function: String,
	#[serde(rename = "aggregate_variable_name")]
	aggregate_variable_name: String,
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "variable_name")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RuleCall {
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseRetrieveSource {
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "new_sortings")]
	new_sortings: microflows::SortingsList,
	#[serde(rename = "range")]
	range: microflows::CustomRange,
	#[serde(rename = "xpath_constraint")]
	xpath_constraint: String,
}

#[derive(Serialize, Deserialize)]
pub struct Nanoflow {
	#[serde(rename = "allowed_module_roles")]
	allowed_module_roles: Vec<>,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "flows")]
	flows: Vec<>,
	#[serde(rename = "mark_as_used")]
	mark_as_used: bool,
	#[serde(rename = "microflow_return_type")]
	microflow_return_type: data_types::VoidType,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "object_collection")]
	object_collection: microflows::MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAction {
	#[serde(rename = "delete_variable_name")]
	delete_variable_name: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "refresh_in_client")]
	refresh_in_client: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleRequestHandling {
	#[serde(rename = "null_value_option")]
	null_value_option: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct ShowMessageAction {
	#[serde(rename = "blocking")]
	blocking: bool,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "template")]
	template: microflows::TextTemplate,
	#[serde(rename = "type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
	#[serde(rename = "apply_entity_access")]
	apply_entity_access: bool,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "flows")]
	flows: Vec<>,
	#[serde(rename = "mark_as_used")]
	mark_as_used: bool,
	#[serde(rename = "microflow_return_type")]
	microflow_return_type: data_types::BooleanType,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "object_collection")]
	object_collection: microflows::MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
pub struct Intersect {
	#[serde(rename = "list_name")]
	list_name: String,
	#[serde(rename = "second_list_or_object_name")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RuleCallParameterMapping {
	#[serde(rename = "argument")]
	argument: String,
	#[serde(rename = "parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExclusiveSplit {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
	#[serde(rename = "split_condition")]
	split_condition: microflows::ExpressionSplitCondition,
}

#[derive(Serialize, Deserialize)]
pub struct EndEvent {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "return_value")]
	return_value: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct BasicCodeActionParameterValue {
	#[serde(rename = "argument")]
	argument: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResultHandling {
	#[serde(rename = "bind")]
	bind: bool,
	#[serde(rename = "import_mapping_call")]
	import_mapping_call: microflows::ImportMappingCall,
	#[serde(rename = "result_variable_name")]
	result_variable_name: String,
	#[serde(rename = "variable_type")]
	variable_type: data_types::ObjectType,
}

#[derive(Serialize, Deserialize)]
pub struct WebServiceOperationSimpleParameterMapping {
	#[serde(rename = "argument")]
	argument: String,
	#[serde(rename = "is_checked")]
	is_checked: bool,
	#[serde(rename = "parameter_name")]
	parameter_name: String,
	#[serde(rename = "parameter_path")]
	parameter_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomRequestHandling {
	#[serde(rename = "template")]
	template: microflows::StringTemplate,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadFileAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "file_document_variable_name")]
	file_document_variable_name: String,
	#[serde(rename = "show_file_in_browser")]
	show_file_in_browser: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CallWebServiceAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "http_configuration")]
	http_configuration: microflows::HttpConfiguration,
	#[serde(rename = "imported_service")]
	imported_service: String,
	#[serde(rename = "is_validation_required")]
	is_validation_required: bool,
	#[serde(rename = "new_result_handling")]
	new_result_handling: microflows::ResultHandling,
	#[serde(rename = "operation_name")]
	operation_name: String,
	#[serde(rename = "proxy_configuration")]
	proxy_configuration: Null,
	#[serde(rename = "request_body_handling")]
	request_body_handling: microflows::SimpleRequestHandling,
	#[serde(rename = "request_header_handling")]
	request_header_handling: microflows::SimpleRequestHandling,
	#[serde(rename = "request_proxy_type")]
	request_proxy_type: String,
	#[serde(rename = "service_name")]
	service_name: String,
	#[serde(rename = "time_out_expression")]
	time_out_expression: String,
	#[serde(rename = "use_request_time_out")]
	use_request_time_out: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SequenceFlow {
	#[serde(rename = "destination_bezier_vector")]
	destination_bezier_vector: String,
	#[serde(rename = "destination_connection_index")]
	destination_connection_index: i64,
	#[serde(rename = "destination_pointer")]
	destination_pointer: Binary,
	#[serde(rename = "is_error_handler")]
	is_error_handler: bool,
	#[serde(rename = "new_case_value")]
	new_case_value: microflows::NoCase,
	#[serde(rename = "origin_bezier_vector")]
	origin_bezier_vector: String,
	#[serde(rename = "origin_connection_index")]
	origin_connection_index: i64,
	#[serde(rename = "origin_pointer")]
	origin_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "list_name")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateParameter {
	#[serde(rename = "expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnnotationFlow {
	#[serde(rename = "destination_bezier_vector")]
	destination_bezier_vector: String,
	#[serde(rename = "destination_connection_index")]
	destination_connection_index: i64,
	#[serde(rename = "destination_pointer")]
	destination_pointer: Binary,
	#[serde(rename = "origin_bezier_vector")]
	origin_bezier_vector: String,
	#[serde(rename = "origin_connection_index")]
	origin_connection_index: i64,
	#[serde(rename = "origin_pointer")]
	origin_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct TextTemplate {
	#[serde(rename = "parameters")]
	parameters: Vec<>,
	#[serde(rename = "text")]
	text: texts::Text,
}

#[derive(Serialize, Deserialize)]
pub struct Find {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "list_name")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveSorting {
	#[serde(rename = "attribute_ref")]
	attribute_ref: domain_models::AttributeRef,
	#[serde(rename = "sort_order")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImportXmlAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "is_validation_required")]
	is_validation_required: bool,
	#[serde(rename = "result_handling")]
	result_handling: microflows::ResultHandling,
	#[serde(rename = "xml_document_variable_name")]
	xml_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sort {
	#[serde(rename = "list_name")]
	list_name: String,
	#[serde(rename = "sortings")]
	sortings: microflows::SortingsList,
}

#[derive(Serialize, Deserialize)]
pub struct ShowHomePageAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCall {
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<>,
	#[serde(rename = "queue_settings")]
	queue_settings: Null,
}

#[derive(Serialize, Deserialize)]
pub struct Head {
	#[serde(rename = "list_name")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WhileLoopCondition {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "while_expression")]
	while_expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilterByExpression {
	#[serde(rename = "expression")]
	expression: String,
	#[serde(rename = "list_name")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogMessageAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "include_latest_stack_trace")]
	include_latest_stack_trace: bool,
	#[serde(rename = "level")]
	level: String,
	#[serde(rename = "message_template")]
	message_template: microflows::StringTemplate,
	#[serde(rename = "node")]
	node: String,
}

#[derive(Serialize, Deserialize)]
pub struct SortingsList {
	#[serde(rename = "sortings")]
	sortings: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionActivity {
	#[serde(rename = "action")]
	action: microflows::MicroflowCallAction,
	#[serde(rename = "auto_generate_caption")]
	auto_generate_caption: bool,
	#[serde(rename = "background_color")]
	background_color: String,
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "disabled")]
	disabled: bool,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeActionItem {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "type")]
	type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct EntityTypeCodeActionParameterValue {
	#[serde(rename = "entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "result_variable_name")]
	result_variable_name: String,
	#[serde(rename = "retrieve_source")]
	retrieve_source: microflows::DatabaseRetrieveSource,
}

#[derive(Serialize, Deserialize)]
pub struct JavaActionParameterMapping {
	#[serde(rename = "parameter")]
	parameter: String,
	#[serde(rename = "value")]
	value: microflows::BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
pub struct IncrementCounterMeterAction {
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tags")]
	tags: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct RestCallAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "error_result_handling_type")]
	error_result_handling_type: String,
	#[serde(rename = "http_configuration")]
	http_configuration: microflows::HttpConfiguration,
	#[serde(rename = "proxy_configuration")]
	proxy_configuration: Null,
	#[serde(rename = "request_handling")]
	request_handling: microflows::CustomRequestHandling,
	#[serde(rename = "request_handling_type")]
	request_handling_type: String,
	#[serde(rename = "request_proxy_type")]
	request_proxy_type: String,
	#[serde(rename = "result_handling")]
	result_handling: microflows::ResultHandling,
	#[serde(rename = "result_handling_type")]
	result_handling_type: String,
	#[serde(rename = "time_out_expression")]
	time_out_expression: String,
	#[serde(rename = "use_request_time_out")]
	use_request_time_out: bool,
}

#[derive(Serialize, Deserialize)]
pub struct StringTemplate {
	#[serde(rename = "parameters")]
	parameters: Vec<>,
	#[serde(rename = "text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateListAction {
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "variable_name")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IterableList {
	#[serde(rename = "list_variable_name")]
	list_variable_name: String,
	#[serde(rename = "variable_name")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListOperationsAction {
	#[serde(rename = "error_handling_type")]
	error_handling_type: String,
	#[serde(rename = "new_operation")]
	new_operation: microflows::Head,
	#[serde(rename = "result_variable_name")]
	result_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StartEvent {
	#[serde(rename = "relative_middle_point")]
	relative_middle_point: String,
	#[serde(rename = "size")]
	size: String,
}

