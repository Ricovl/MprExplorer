use bson::Binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum Flow {
	#[serde(rename = "Microflows$AnnotationFlow")]
	AnnotationFlow(microflows::AnnotationFlow), 
	#[serde(rename = "Microflows$SequenceFlow")]
	SequenceFlow(microflows::SequenceFlow),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum MicroFlowObject {
	#[serde(rename = "Microflows$MicroflowParameter")]
	MicroflowParameter(microflows::MicroflowParameter),
	#[serde(rename = "Microflows$EndEvent")]
	EndEvent(microflows::EndEvent),
	#[serde(rename = "Microflows$StartEvent")]
	StartEvent(microflows::StartEvent),
	#[serde(rename = "Microflows$ContinueEvent")]
	ContinueEvent(microflows::ContinueEvent),
	#[serde(rename = "Microflows$LoopedActivity")]
	LoopedActivity(microflows::LoopedActivity),
	#[serde(rename = "Microflows$ExclusiveMerge")]
	ExclusiveMerge(microflows::ExclusiveMerge),
	#[serde(rename = "Microflows$InheritanceSplit")]
	InheritanceSplit(microflows::InheritanceSplit),
	#[serde(rename = "Microflows$ErrorEvent")]
	ErrorEvent(microflows::ErrorEvent),
	#[serde(rename = "Microflows$ExclusiveSplit")]
	ExclusiveSplit(microflows::ExclusiveSplit),
	#[serde(rename = "Microflows$Annotation")]
	Annotation(microflows::Annotation),
	#[serde(rename = "Microflows$ActionActivity")]
	ActionActivity(microflows::ActionActivity),
	#[serde(rename = "Microflows$BreakEvent")]
	BreakEvent(microflows::BreakEvent),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum MicroflowActionType {
	#[serde(rename = "Microflows$ShowFormAction")]
	ShowFormAction(microflows::ShowFormAction),
	#[serde(rename = "Microflows$SynchronizeAction")]
	SynchronizeAction(microflows::SynchronizeAction),
	#[serde(rename = "Microflows$NanoflowCallAction")]
	NanoflowCallAction(microflows::NanoflowCallAction),
	#[serde(rename = "Microflows$LogMessageAction")]
	LogMessageAction(microflows::LogMessageAction),
	#[serde(rename = "Microflows$JavaScriptActionCallAction")]
	JavaScriptActionCallAction(microflows::JavaScriptActionCallAction),
	#[serde(rename = "Microflows$RollbackAction")]
	RollbackAction(microflows::RollbackAction),
	#[serde(rename = "Microflows$CallWebServiceAction")]
	CallWebServiceAction(microflows::CallWebServiceAction),
	#[serde(rename = "Microflows$RestCallAction")]
	RestCallAction(microflows::RestCallAction),
	#[serde(rename = "Microflows$CreateChangeAction")]
	CreateChangeAction(microflows::CreateChangeAction),
	#[serde(rename = "Microflows$RetrieveAction")]
	RetrieveAction(microflows::RetrieveAction),
	#[serde(rename = "Microflows$ExportXmlAction")]
	ExportXmlAction(microflows::ExportXmlAction),
	#[serde(rename = "Microflows$CreateVariableAction")]
	CreateVariableAction(microflows::CreateVariableAction),
	#[serde(rename = "Microflows$DeleteAction")]
	DeleteAction(microflows::DeleteAction),
	#[serde(rename = "Microflows$ChangeListAction")]
	ChangeListAction(microflows::ChangeListAction),
	#[serde(rename = "Microflows$CreateListAction")]
	CreateListAction(microflows::CreateListAction),
	#[serde(rename = "Microflows$ChangeVariableAction")]
	ChangeVariableAction(microflows::ChangeVariableAction),
	#[serde(rename = "Microflows$AggregateAction")]
	AggregateAction(microflows::AggregateAction),
	#[serde(rename = "Microflows$ValidationFeedbackAction")]
	ValidationFeedbackAction(microflows::ValidationFeedbackAction),
	#[serde(rename = "Microflows$ListOperationsAction")]
	ListOperationsAction(microflows::ListOperationsAction),
	#[serde(rename = "Microflows$ImportXmlAction")]
	ImportXmlAction(microflows::ImportXmlAction),
	#[serde(rename = "Microflows$ShowMessageAction")]
	ShowMessageAction(microflows::ShowMessageAction),
	#[serde(rename = "Microflows$MicroflowCallAction")]
	MicroflowCallAction(microflows::MicroflowCallAction),
	#[serde(rename = "Microflows$CloseFormAction")]
	CloseFormAction(microflows::CloseFormAction),
	#[serde(rename = "Microflows$CommitAction")]
	CommitAction(microflows::CommitAction),
	#[serde(rename = "Microflows$ShowHomePageAction")]
	ShowHomePageAction(microflows::ShowHomePageAction),
	#[serde(rename = "Microflows$CastAction")]
	CastAction(microflows::CastAction),
	#[serde(rename = "Microflows$DownloadFileAction")]
	DownloadFileAction(microflows::DownloadFileAction),
	#[serde(rename = "Microflows$IncrementCounterMeterAction")]
	IncrementCounterMeterAction(microflows::IncrementCounterMeterAction),
	#[serde(rename = "Microflows$GenerateDocumentAction")]
	GenerateDocumentAction(microflows::GenerateDocumentAction),
	#[serde(rename = "Microflows$ChangeAction")]
	ChangeAction(microflows::ChangeAction),
	#[serde(rename = "Microflows$JavaActionCallAction")]
	JavaActionCallAction(microflows::JavaActionCallAction),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum ActionParameterValue {
	#[serde(rename = "Microflows$MicroflowParameterValue")]
	MicroflowParameterValue(microflows::MicroflowParameterValue),
	#[serde(rename = "Microflows$EntityTypeCodeActionParameterValue")]
	EntityTypeCodeActionParameterValue(microflows::EntityTypeCodeActionParameterValue),
	#[serde(rename = "Microflows$BasicCodeActionParameterValue")]
	BasicCodeActionParameterValue(microflows::BasicCodeActionParameterValue)
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum RequestHandler {
	#[serde(rename = "Microflows$MappingRequestHandling")]
	MappingRequestHandling(microflows::MappingRequestHandling),
	#[serde(rename = "Microflows$SimpleRequestHandling")]
	SimpleRequestHandling(microflows::SimpleRequestHandling),
	#[serde(rename = "Microflows$CustomRequestHandling")]
	CustomRequestHandling(microflows::CustomRequestHandling),
	#[serde(rename = "Microflows$BinaryRequestHandling")]
	BinaryRequestHandling(microflows::BinaryRequestHandling),
}


#[derive(Serialize, Deserialize)]
pub struct ActionActivity {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	pub action: Option<MicroflowActionType>,
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
	destination_pointer: Uuid,
	#[serde(rename = "OriginBezierVector")]
	origin_bezier_vector: String,
	#[serde(rename = "OriginConnectionIndex")]
	origin_connection_index: i64,
	#[serde(rename = "OriginPointer")]
	origin_pointer: Uuid,
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
	http_configuration: Option<microflows::HttpConfiguration>,
	#[serde(rename = "ImportedService")]
	imported_service: String,
	#[serde(rename = "IsValidationRequired")]
	is_validation_required: bool,
	#[serde(rename = "NewResultHandling")]
	new_result_handling: Option<microflows::ResultHandling>,
	#[serde(rename = "OperationName")]
	operation_name: String,
	#[serde(rename = "ProxyConfiguration")]
	proxy_configuration: Option<Empty>,
	#[serde(rename = "RequestBodyHandling")]
	request_body_handling: RequestHandler,
	#[serde(rename = "RequestHeaderHandling")]
	request_header_handling: Option<microflows::SimpleRequestHandling>,
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
	#[serde(rename = "Items", deserialize_with = "deserialize_settings")]
	items: Vec<microflows::ChangeActionItem>,
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
	#[serde(rename = "Items", deserialize_with = "deserialize_settings")]
	items: Vec<microflows::ChangeActionItem>,
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
	variable_type: Option<data_types::DataType>,
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
	template: Option<microflows::StringTemplate>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum SourceRange {
	#[serde(rename = "Microflows$ConstantRange")]
	ConstantRange(microflows::ConstantRange),
	#[serde(rename = "Microflows$CustomRange")]
	CustomRange(microflows::CustomRange),
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseRetrieveSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "NewSortings")]
	new_sortings: Option<microflows::SortingsList>,
	#[serde(rename = "Range")]
	range: SourceRange,
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
#[serde(tag = "$Type")]
pub enum SplitCondition {
	#[serde(rename = "Microflows$ExpressionSplitCondition")]
	ExpressionSplitCondition(microflows::ExpressionSplitCondition),
	#[serde(rename = "Microflows$RuleSplitCondition")]
	RuleSplitCondition(microflows::RuleSplitCondition),
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
	split_condition: SplitCondition,
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
	output_method: Option<export_xml_action::XmlExportAction>,
	#[serde(rename = "ResultHandling")]
	result_handling: Option<microflows::MappingRequestHandling>,
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
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<microflows::DocumentTemplateParameterMapping>,
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
	custom_location_template: Option<microflows::StringTemplate>,
	#[serde(rename = "HttpAuthenticationPassword")]
	http_authentication_password: String,
	#[serde(rename = "HttpAuthenticationUserName")]
	http_authentication_user_name: String,
	#[serde(rename = "HttpHeaderEntries", deserialize_with = "deserialize_settings")]
	http_header_entries: Vec<microflows::HttpHeaderEntry>,
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
	range: Option<microflows::ConstantRange>,
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
	result_handling: Option<microflows::ResultHandling>,
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
	#[serde(rename = "Tags", deserialize_with = "deserialize_settings")]
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
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<microflows::JavaActionParameterMapping>,
	#[serde(rename = "QueueSettings")]
	queue_settings: Option<Empty>,
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
	value: ActionParameterValue,
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
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<microflows::JavaScriptActionParameterMapping>,
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
	parameter_value: ActionParameterValue,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum ListOperation {
	#[serde(rename = "Microflows$Head")]
	Head(microflows::Head),
	#[serde(rename = "Microflows$Subtract")]
	Subtract(microflows::Subtract),
	#[serde(rename = "Microflows$Filter")]
	Filter(microflows::Filter),
	#[serde(rename = "Microflows$FindByExpression")]
	FindByExpression(microflows::FindByExpression),
	#[serde(rename = "Microflows$Contains")]
	Contains(microflows::Contains),
	#[serde(rename = "Microflows$Find")]
	Find(microflows::Find),
	#[serde(rename = "Microflows$Sort")]
	Sort(microflows::Sort),
	#[serde(rename = "Microflows$Union")]
	Union(microflows::Union),
	#[serde(rename = "Microflows$Tail")]
	Tail(microflows::Tail),
	#[serde(rename = "Microflows$Intersect")]
	Intersect(microflows::Intersect),
	#[serde(rename = "Microflows$FilterByExpression")]
	FilterByExpression(microflows::FilterByExpression),
}

#[derive(Serialize, Deserialize)]
pub struct ListOperationsAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NewOperation")]
	new_operation: ListOperation,
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
	message_template: Option<microflows::StringTemplate>,
	#[serde(rename = "Node")]
	node: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum LoopSource {
	#[serde(rename = "Microflows$IterableList")]
	IterableList(microflows::IterableList),
	#[serde(rename = "Microflows$WhileLoopCondition")]
	WhileLoopCondition(microflows::WhileLoopCondition),
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
	loop_source: LoopSource,
	#[serde(rename = "ObjectCollection")]
	object_collection: Option<microflows::MicroflowObjectCollection>,
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
	#[serde(rename = "AllowedModuleRoles", deserialize_with = "deserialize_settings")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "ApplyEntityAccess")]
	apply_entity_access: bool,
	#[serde(rename = "ConcurrencyErrorMicroflow")]
	concurrency_error_microflow: String,
	#[serde(rename = "ConcurrenyErrorMessage")]
	concurreny_error_message: Option<texts::Text>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows", deserialize_with = "deserialize_settings")]
	flows: Vec<Flow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowActionInfo")]
	microflow_action_info: Option<Empty>,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: Option<data_types::DataType>,
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "ObjectCollection")]
	pub object_collection: Option<microflows::MicroflowObjectCollection>,
	#[serde(rename = "WorkflowActionInfo")]
	workflow_action_info: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	pub microflow: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	pub parameter_mappings: Vec<microflows::MicroflowCallParameterMapping>,
	#[serde(rename = "QueueSettings")]
	queue_settings: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "MicroflowCall")]
	pub microflow_call: Option<microflows::MicroflowCall>,
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
	pub argument: String,
	#[serde(rename = "Parameter")]
	pub parameter: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowObjectCollection {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Objects", deserialize_with = "deserialize_settings")]
	pub objects: Vec<MicroFlowObject>,
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
	pub name: String,
	#[serde(rename = "RelativeMiddlePoint")]
	relative_middle_point: String,
	#[serde(rename = "Size")]
	size: String,
	#[serde(rename = "VariableType")]
	pub variable_type: Option<data_types::DataType>,
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

	#[serde(rename = "AllowedModuleRoles", deserialize_with = "deserialize_settings")]
	allowed_module_roles: Vec<String>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Flows", deserialize_with = "deserialize_settings")]
	flows: Vec<Flow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: Option<data_types::DataType>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: Option<microflows::MicroflowObjectCollection>,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Nanoflow")]
	nanoflow: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<microflows::NanoflowCallParameterMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct NanoflowCallAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "NanoflowCall")]
	nanoflow_call: Option<microflows::NanoflowCall>,
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
	http_configuration: Option<microflows::HttpConfiguration>,
	#[serde(rename = "ProxyConfiguration")]
	proxy_configuration: Option<Empty>,
	#[serde(rename = "RequestHandling")]
	request_handling: RequestHandler,
	#[serde(rename = "RequestHandlingType")]
	request_handling_type: String,
	#[serde(rename = "RequestProxyType")]
	request_proxy_type: String,
	#[serde(rename = "ResultHandling")]
	result_handling: Option<microflows::ResultHandling>,
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
	import_mapping_call: Option<microflows::ImportMappingCall>,
	#[serde(rename = "ResultVariableName")]
	result_variable_name: String,
	#[serde(rename = "VariableType")]
	variable_type: Option<data_types::DataType>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum RetrieveSource {
	#[serde(rename = "Microflows$AssociationRetrieveSource")]
	AssociationRetrieveSource(microflows::AssociationRetrieveSource),
	#[serde(rename = "Microflows$DatabaseRetrieveSource")]
	DatabaseRetrieveSource(microflows::DatabaseRetrieveSource),
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
	retrieve_source: RetrieveSource,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveSorting {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeRef")]
	attribute_ref: Option<domain_models::AttributeRef>,
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
	#[serde(rename = "Flows", deserialize_with = "deserialize_settings")]
	flows: Vec<microflows::SequenceFlow>,
	#[serde(rename = "MarkAsUsed")]
	mark_as_used: bool,
	#[serde(rename = "MicroflowReturnType")]
	microflow_return_type: Option<data_types::DataType>,
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "ObjectCollection")]
	object_collection: Option<microflows::MicroflowObjectCollection>,
}

#[derive(Serialize, Deserialize)]
pub struct RuleCall {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<microflows::RuleCallParameterMapping>,
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
	rule_call: Option<microflows::RuleCall>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum SequenceFlowCase {
	#[serde(rename = "Microflows$EnumerationCase")]
	EnumerationCase(microflows::EnumerationCase),
	#[serde(rename = "Microflows$InheritanceCase")]
	InheritanceCase(microflows::InheritanceCase),
	#[serde(rename = "Microflows$NoCase")]
	NoCase(microflows::NoCase),
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
	destination_pointer: Uuid,
	#[serde(rename = "IsErrorHandler")]
	is_error_handler: bool,
	#[serde(rename = "NewCaseValue")]
	new_case_value: SequenceFlowCase,
	#[serde(rename = "OriginBezierVector")]
	origin_bezier_vector: String,
	#[serde(rename = "OriginConnectionIndex")]
	origin_connection_index: i64,
	#[serde(rename = "OriginPointer")]
	origin_pointer: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ShowFormAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FormSettings")]
	form_settings: Option<forms::FormSettings>,
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
	template: Option<microflows::TextTemplate>,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleRequestHandling {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "NullValueOption")]
	null_value_option: String,
	#[serde(rename = "ParameterMappings", deserialize_with = "deserialize_settings")]
	parameter_mappings: Vec<microflows::WebServiceOperationSimpleParameterMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct Sort {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ListName")]
	list_name: String,
	#[serde(rename = "Sortings")]
	sortings: Option<microflows::SortingsList>,
}

#[derive(Serialize, Deserialize)]
pub struct SortingsList {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Sortings", deserialize_with = "deserialize_settings")]
	sortings: Vec<microflows::RetrieveSorting>,
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

	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<microflows::TemplateParameter>,
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
	#[serde(rename = "VariableNames", deserialize_with = "deserialize_settings")]
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

	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<microflows::TemplateParameter>,
	#[serde(rename = "Text")]
	text: Option<texts::Text>,
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
	association: Option<String>,
	#[serde(rename = "Attribute")]
	attribute: Option<String>,
	#[serde(rename = "ErrorHandlingType")]
	error_handling_type: String,
	#[serde(rename = "FeedbackTemplate")]
	feedback_template: Option<microflows::TextTemplate>,
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

