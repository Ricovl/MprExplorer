use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ActionActivityDefaultColor {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ActionActivityType")]
	action_activity_type: String,
	#[serde(rename = "BackgroundColor")]
	background_color: String,
}

#[derive(Serialize, Deserialize)]
pub struct Certificate {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Data")]
	data: Binary,
	#[serde(rename = "Type")]
	var_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Certificates")]
	certificates: Vec<settings::Certificate, >,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigurationSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Configurations")]
	configurations: Vec<settings::ServerConfiguration, >,
}

#[derive(Serialize, Deserialize)]
pub struct ConstantValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ConstantId")]
	constant_id: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConventionSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ActionActivityDefaultColors")]
	action_activity_default_colors: Vec<settings::ActionActivityDefaultColor, >,
	#[serde(rename = "LowerCaseMicroflowVariables")]
	lower_case_microflow_variables: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DistributionSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "BasedOnVersion")]
	based_on_version: String,
	#[serde(rename = "IsDistributable")]
	is_distributable: bool,
	#[serde(rename = "Version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
pub struct IntegrationProjectSettingsPart {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct JarDeploymentSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Exclusions")]
	exclusions: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct LanguageSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DefaultLanguageCode")]
	default_language_code: String,
	#[serde(rename = "Languages")]
	languages: Vec<texts::Language, >,
}

#[derive(Serialize, Deserialize)]
pub struct ModelSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AfterStartupMicroflow")]
	after_startup_microflow: String,
	#[serde(rename = "AllowUserMultipleSessions")]
	allow_user_multiple_sessions: bool,
	#[serde(rename = "BcryptCost")]
	bcrypt_cost: i64,
	#[serde(rename = "BeforeShutdownMicroflow")]
	before_shutdown_microflow: String,
	#[serde(rename = "DefaultTimeZoneCode")]
	default_time_zone_code: String,
	#[serde(rename = "EnableDataStorageOptimisticLocking")]
	enable_data_storage_optimistic_locking: bool,
	#[serde(rename = "FirstDayOfWeek")]
	first_day_of_week: String,
	#[serde(rename = "HashAlgorithm")]
	hash_algorithm: String,
	#[serde(rename = "HealthCheckMicroflow")]
	health_check_microflow: String,
	#[serde(rename = "RoundingMode")]
	rounding_mode: String,
	#[serde(rename = "ScheduledEventTimeZoneCode")]
	scheduled_event_time_zone_code: String,
	#[serde(rename = "UseSystemContextForBackgroundTasks")]
	use_system_context_for_background_tasks: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Settings")]
	settings: Vec<settings::WorkflowsProjectSettingsPart, settings::DistributionSettings, settings::JarDeploymentSettings, settings::LanguageSettings, settings::ModelSettings, settings::CertificateSettings, forms::WebUIProjectSettingsPart, settings::IntegrationProjectSettingsPart, settings::ConfigurationSettings, settings::ConventionSettings, >,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfiguration {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ApplicationRootUrl")]
	application_root_url: String,
	#[serde(rename = "ConstantValues")]
	constant_values: Vec<settings::ConstantValue, >,
	#[serde(rename = "CustomSettings")]
	custom_settings: Vec<UnknownType>,
	#[serde(rename = "DatabaseName")]
	database_name: String,
	#[serde(rename = "DatabasePassword")]
	database_password: String,
	#[serde(rename = "DatabaseType")]
	database_type: String,
	#[serde(rename = "DatabaseUrl")]
	database_url: String,
	#[serde(rename = "DatabaseUseIntegratedSecurity")]
	database_use_integrated_security: bool,
	#[serde(rename = "DatabaseUserName")]
	database_user_name: String,
	#[serde(rename = "ExtraJvmParameters")]
	extra_jvm_parameters: String,
	#[serde(rename = "HttpPortNumber")]
	http_port_number: i64,
	#[serde(rename = "MaxJavaHeapSize")]
	max_java_heap_size: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OpenAdminPort")]
	open_admin_port: bool,
	#[serde(rename = "OpenHttpPort")]
	open_http_port: bool,
	#[serde(rename = "ServerPortNumber")]
	server_port_number: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeModuleEntry {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ModuleName")]
	module_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowsProjectSettingsPart {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DefaultTaskParallelism")]
	default_task_parallelism: i64,
	#[serde(rename = "UserEntity")]
	user_entity: String,
	#[serde(rename = "UsertaskOnStateChangeEvent")]
	usertask_on_state_change_event: Empty,
	#[serde(rename = "WorkflowEngineParallelism")]
	workflow_engine_parallelism: i64,
	#[serde(rename = "WorkflowOnStateChangeEvent")]
	workflow_on_state_change_event: Empty,
}

