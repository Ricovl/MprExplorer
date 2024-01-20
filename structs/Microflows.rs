#[derive(Serialize, Deserialize)]
struct CreateListAction {
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct Annotation {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
struct DownloadFileAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FileDocumentVariableName")]
	file_document_variable_name: String,
	#[serde(rename = "ShowFileInBrowser")]
	show_file_in_browser: Bool,
}

#[derive(Serialize, Deserialize)]
struct Microflow {
	#[serde(rename = "AllowConcurrentExecution")]
	allow_concurrent_execution: Bool,
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<1, []>,
	#[serde(rename = "ApplyEntityAccess")]
	apply_entity_access: Bool,
	#[serde(rename = "ConcurrencyErrorMicroflow")]
	concurrency_error_microflow: String,
	#[serde(rename = "ConcurrenyErrorMessage")]
	concurreny_error_message: Texts$Text,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<3, ["Microflows$SequenceFlow"]>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: Bool,
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
struct ShowFormAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FormSettings")]
	form_settings: Forms$FormSettings,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
struct MicroflowCall {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, ["Microflows$MicroflowCallParameterMapping"]>,
	#[serde(rename = "QueueSettings")]
	queue_settings: Null,
}

#[derive(Serialize, Deserialize)]
struct Contains {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
struct NanoflowCall {
	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct ExpressionSplitCondition {
	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
struct Filter {
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
struct NanoflowCallParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
struct RuleCallParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
struct FindByExpression {
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
struct Union {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
struct SimpleRequestHandling {
	#[serde(rename = "NullValueOption")]
	null_value_option: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, ["Microflows$WebServiceOperationSimpleParameterMapping"]>,
}

#[derive(Serialize, Deserialize)]
struct HttpHeaderEntry {
	#[serde(rename = "Key")]
	key: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
struct RuleCall {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, ["Microflows$RuleCallParameterMapping"]>,
}

#[derive(Serialize, Deserialize)]
struct TextTemplate {
	#[serde(rename = "Parameters")]
	parameters: Vec<2, []>,
	#[serde(rename = "Text")]
	text: Texts$Text,
}

#[derive(Serialize, Deserialize)]
struct MicroflowCallParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "Parameter")]
	parameter: String,
}

#[derive(Serialize, Deserialize)]
struct MappingRequestHandling {
	#[serde(rename = "ContentType")]
	content_type: String,
	#[serde(rename = "MappingId")]
	mapping_id: String,
	#[serde(rename = "MappingVariableName")]
	mapping_variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct ActionActivity {
	#[serde(rename = "Action")]
	action: Microflows$RetrieveAction,
	#[serde(rename = "AutoGenerateCaption")]
	auto_generate_caption: Bool,
	#[serde(rename = "BackgroundColor")]
	background_color: String,
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Disabled")]
	disabled: Bool,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
struct MicroflowParameter {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "HasVariableNameBeenChanged")]
	has_variable_name_been_changed: Bool,
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
struct SequenceFlow {
	#[serde(rename = "DestinationBezierVector")]
	destination_bezier_vector: String,
	#[serde(rename = "DestinationConnectionIndex")]
	destination_connection_index: i64,
	#[serde(rename = "DestinationPointer")]
	destination_pointer: Binary,
	#[serde(rename = "IsErrorHandler")]
	is_error_handler: Bool,
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
struct RollbackAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: Bool,
	#[serde(rename = "RollbackVariableName")]
	rollback_variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct BinaryRequestHandling {
	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
struct JavaActionCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "JavaAction")]
	java_action: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, []>,
	#[serde(rename = "QueueSettings")]
	queue_settings: Null,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: Bool,
}

#[derive(Serialize, Deserialize)]
struct AnnotationFlow {
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
struct RuleSplitCondition {
	#[serde(rename = "RuleCall")]
	rule_call: Microflows$RuleCall,
}

#[derive(Serialize, Deserialize)]
struct ExclusiveSplit {
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
struct MicroflowParameterValue {
	#[serde(rename = "Microflow")]
	microflow: String,
}

#[derive(Serialize, Deserialize)]
struct ChangeActionItem {
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
struct IterableList {
	#[serde(rename = "ListVariableName")]
	list_variable_name: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct CallWebServiceAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "HttpConfiguration")]
	http_configuration: Microflows$HttpConfiguration,
	#[serde(rename = "ImportedService")]
	imported_service: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: Bool,
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
	use_request_time_out: Bool,
}

#[derive(Serialize, Deserialize)]
struct Find {
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
struct Tail {
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
struct MicroflowCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "MicroflowCall")]
	microflow_call: Microflows$MicroflowCall,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: Bool,
}

#[derive(Serialize, Deserialize)]
struct ContinueEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
struct CloseFormAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NumberOfPagesToClose")]
	number_of_pages_to_close: String,
}

#[derive(Serialize, Deserialize)]
struct CreateVariableAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "InitialValue")]
	initial_value: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
	#[serde(rename = "VariableType")]
	variable_type: DataTypes$StringType,
}

#[derive(Serialize, Deserialize)]
struct ConstantRange {
	#[serde(rename = "SingleObject")]
	single_object: Bool,
}

#[derive(Serialize, Deserialize)]
struct StartEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
struct EndEvent {
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
struct Subtract {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
struct DeleteAction {
	#[serde(rename = "DeleteVariableName")]
	delete_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: Bool,
}

#[derive(Serialize, Deserialize)]
struct CommitAction {
	#[serde(rename = "CommitVariableName")]
	commit_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: Bool,
	#[serde(rename = "WithEvents")]
	with_events: Bool,
}

#[derive(Serialize, Deserialize)]
struct ResultHandling {
	#[serde(rename = "Bind")]
	bind: Bool,
	#[serde(rename = "ImportMappingCall")]
	import_mapping_call: Microflows$ImportMappingCall,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "VariableType")]
	variable_type: DataTypes$ObjectType,
}

#[derive(Serialize, Deserialize)]
struct WebServiceOperationSimpleParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "IsChecked")]
	is_checked: Bool,
	#[serde(rename = "ParameterName")]
	parameter_name: String,
	#[serde(rename = "ParameterPath")]
	parameter_path: String,
}

#[derive(Serialize, Deserialize)]
struct CustomRange {
	#[serde(rename = "LimitExpression")]
	limit_expression: String,
	#[serde(rename = "OffsetExpression")]
	offset_expression: String,
}

#[derive(Serialize, Deserialize)]
struct CreateChangeAction {
	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Items")]
	items: Vec<2, ["Microflows$ChangeActionItem"]>,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: Bool,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct CustomRequestHandling {
	#[serde(rename = "Template")]
	template: Microflows$StringTemplate,
}

#[derive(Serialize, Deserialize)]
struct NanoflowCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NanoflowCall")]
	nanoflow_call: Microflows$NanoflowCall,
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: Bool,
}

#[derive(Serialize, Deserialize)]
struct AggregateAction {
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
struct JavaScriptActionCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "JavaScriptAction")]
	java_script_action: String,
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, ["Microflows$JavaScriptActionParameterMapping"]>,
	#[serde(rename = "UseReturnVariable")]
	use_return_variable: Bool,
}

#[derive(Serialize, Deserialize)]
struct InheritanceSplit {
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
struct ShowHomePageAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
struct DocumentTemplateParameterMapping {
	#[serde(rename = "Argument")]
	argument: String,
	#[serde(rename = "WidgetName")]
	widget_name: String,
}

#[derive(Serialize, Deserialize)]
struct WhileLoopCondition {
	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "WhileExpression")]
	while_expression: String,
}

#[derive(Serialize, Deserialize)]
struct FilterByExpression {
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
struct GenerateDocumentAction {
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
	override_bottom_margin: Bool,
	#[serde(rename = "OverrideLeftMargin")]
	override_left_margin: Bool,
	#[serde(rename = "OverrideRightMargin")]
	override_right_margin: Bool,
	#[serde(rename = "OverrideTopMargin")]
	override_top_margin: Bool,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, ["Microflows$DocumentTemplateParameterMapping"]>,
}

#[derive(Serialize, Deserialize)]
struct HttpConfiguration {
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
	http_header_entries: Vec<3, ["Microflows$HttpHeaderEntry"]>,
	#[serde(rename = "HttpMethod")]
	http_method: String,
	#[serde(rename = "OverrideLocation")]
	override_location: Bool,
	#[serde(rename = "UseHttpAuthentication")]
	use_http_authentication: Bool,
}

#[derive(Serialize, Deserialize)]
struct ValidationFeedbackAction {
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
struct EnumerationCase {
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
struct ChangeAction {
	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Items")]
	items: Vec<2, ["Microflows$ChangeActionItem"]>,
	#[serde(rename = "RefreshInClient")]
	refresh_in_client: Bool,
}

#[derive(Serialize, Deserialize)]
struct ImportXmlAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: Bool,
	#[serde(rename = "ResultHandling")]
	result_handling: Microflows$ResultHandling,
	#[serde(rename = "XmlDocumentVariableName")]
	xml_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct ExportXmlAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: Bool,
	#[serde(rename = "OutputMethod")]
	output_method: ExportXmlAction$StringExport,
	#[serde(rename = "ResultHandling")]
	result_handling: Microflows$MappingRequestHandling,
}

#[derive(Serialize, Deserialize)]
struct RetrieveSorting {
	#[serde(rename = "AttributeRef")]
	attribute_ref: DomainModels$AttributeRef,
	#[serde(rename = "SortOrder")]
	sort_order: String,
}

#[derive(Serialize, Deserialize)]
struct StringTemplate {
	#[serde(rename = "Parameters")]
	parameters: Vec<2, ["Microflows$TemplateParameter"]>,
	#[serde(rename = "Text")]
	text: String,
}

#[derive(Serialize, Deserialize)]
struct NoCase {
}

#[derive(Serialize, Deserialize)]
struct Nanoflow {
	#[serde(rename = "AllowedModuleRoles")]
	allowed_module_roles: Vec<1, ["String"]>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<3, ["Microflows$SequenceFlow"]>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: Bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: DataTypes$BooleanType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: Microflows$MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
struct Sort {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "Sortings")]
	sortings: Microflows$SortingsList,
}

#[derive(Serialize, Deserialize)]
struct CastAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "VariableName")]
	variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct DatabaseRetrieveSource {
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "NewSortings")]
	new_sortings: Microflows$SortingsList,
	#[serde(rename = "Range")]
	range: Microflows$ConstantRange,
	#[serde(rename = "XpathConstraint")]
	xpath_constraint: String,
}

#[derive(Serialize, Deserialize)]
struct AssociationRetrieveSource {
	#[serde(rename = "AssociationId")]
	association_id: String,
	#[serde(rename = "StartVariableName")]
	start_variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct Intersect {
	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "SecondListOrObjectName")]
	second_list_or_object_name: String,
}

#[derive(Serialize, Deserialize)]
struct InheritanceCase {
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
struct TemplateParameter {
	#[serde(rename = "Expression")]
	expression: String,
}

#[derive(Serialize, Deserialize)]
struct SortingsList {
	#[serde(rename = "Sortings")]
	sortings: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct JavaActionParameterMapping {
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "Value")]
	value: Microflows$BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
struct JavaScriptActionParameterMapping {
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "ParameterValue")]
	parameter_value: Microflows$BasicCodeActionParameterValue,
}

#[derive(Serialize, Deserialize)]
struct BasicCodeActionParameterValue {
	#[serde(rename = "Argument")]
	argument: String,
}

#[derive(Serialize, Deserialize)]
struct ExclusiveMerge {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
struct ShowMessageAction {
	#[serde(rename = "Blocking")]
	blocking: Bool,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Template")]
	template: Microflows$TextTemplate,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct Rule {
	#[serde(rename = "ApplyEntityAccess")]
	apply_entity_access: Bool,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows")]
	flows: Vec<3, ["Microflows$SequenceFlow"]>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: Bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: DataTypes$BooleanType,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: Microflows$MicroflowObjectCollection,
}

#[derive(Serialize, Deserialize)]
struct LogMessageAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "IncludeLatestStackTrace")]
	include_latest_stack_trace: Bool,
	#[serde(rename = "Level")]
	level: String,
	#[serde(rename = "MessageTemplate")]
	message_template: Microflows$StringTemplate,
	#[serde(rename = "Node")]
	node: String,
}

#[derive(Serialize, Deserialize)]
struct ChangeListAction {
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
struct RestCallAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "ErrorResultHandlingType")]
	error_result_handling_type: String,
	#[serde(rename = "HttpConfiguration")]
	http_configuration: Microflows$HttpConfiguration,
	#[serde(rename = "ProxyConfiguration")]
	proxy_configuration: Null,
	#[serde(rename = "RequestHandling")]
	request_handling: Microflows$BinaryRequestHandling,
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
	use_request_time_out: Bool,
}

#[derive(Serialize, Deserialize)]
struct SynchronizeAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Type")]
	type: String,
	#[serde(rename = "VariableNames")]
	variable_names: Vec<1, ["String"]>,
}

#[derive(Serialize, Deserialize)]
struct BreakEvent {
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
}

#[derive(Serialize, Deserialize)]
struct IncrementCounterMeterAction {
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
struct ListOperationsAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NewOperation")]
	new_operation: Microflows$Head,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct EntityTypeCodeActionParameterValue {
	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
struct ChangeVariableAction {
	#[serde(rename = "ChangeVariableName")]
	change_variable_name: String,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
struct Head {
	#[serde(rename = "ListName")]
	list_name: String,
}

#[derive(Serialize, Deserialize)]
struct MicroflowObjectCollection {
	#[serde(rename = "Objects")]
	objects: Vec<3, ["Microflows$StartEvent", "Microflows$EndEvent", "Microflows$MicroflowParameter", "Microflows$ActionActivity"]>,
}

#[derive(Serialize, Deserialize)]
struct LoopedActivity {
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
struct RetrieveAction {
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "RetrieveSource")]
	retrieve_source: Microflows$AssociationRetrieveSource,
}

#[derive(Serialize, Deserialize)]
struct ImportMappingCall {
	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "ContentType")]
	content_type: String,
	#[serde(rename = "ForceSingleOccurrence")]
	force_single_occurrence: Bool,
	#[serde(rename = "ObjectHandlingBackup")]
	object_handling_backup: String,
	#[serde(rename = "ParameterVariableName")]
	parameter_variable_name: String,
	#[serde(rename = "Range")]
	range: Microflows$ConstantRange,
	#[serde(rename = "ReturnValueMapping")]
	return_value_mapping: String,
}

