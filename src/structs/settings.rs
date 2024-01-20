use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct JarDeploymentSettings {
	#[serde(rename = "Exclusions")]
	exclusions: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectSettings {
	#[serde(rename = "Settings")]
	settings: Vec<2, ["Forms$WebUIProjectSettingsPart", "Settings$IntegrationProjectSettingsPart", "Settings$ConfigurationSettings", "Settings$ModelSettings", "Settings$ConventionSettings", "Settings$LanguageSettings", "Settings$CertificateSettings", "Settings$WorkflowsProjectSettingsPart", "Settings$JarDeploymentSettings", "Settings$DistributionSettings"]>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionActivityDefaultColor {
	#[serde(rename = "ActionActivityType")]
	action_activity_type: String,
	#[serde(rename = "BackgroundColor")]
	background_color: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConventionSettings {
	#[serde(rename = "ActionActivityDefaultColors")]
	action_activity_default_colors: Vec<Settings$ActionActivityDefaultColor>,
	#[serde(rename = "LowerCaseMicroflowVariables")]
	lower_case_microflow_variables: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConstantValue {
	#[serde(rename = "ConstantId")]
	constant_id: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfiguration {
	#[serde(rename = "ApplicationRootUrl")]
	application_root_url: String,
	#[serde(rename = "ConstantValues")]
	constant_values: Vec<Settings$ConstantValue>,
	#[serde(rename = "CustomSettings")]
	custom_settings: Vec<3, []>,
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
pub struct ConfigurationSettings {
	#[serde(rename = "Configurations")]
	configurations: Vec<Settings$ServerConfiguration>,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeModuleEntry {
	#[serde(rename = "ModuleName")]
	module_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Certificate {
	#[serde(rename = "Data")]
	data: Binary,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DistributionSettings {
	#[serde(rename = "BasedOnVersion")]
	based_on_version: String,
	#[serde(rename = "IsDistributable")]
	is_distributable: bool,
	#[serde(rename = "Version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModelSettings {
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
pub struct IntegrationProjectSettingsPart {
}

#[derive(Serialize, Deserialize)]
pub struct LanguageSettings {
	#[serde(rename = "DefaultLanguageCode")]
	default_language_code: String,
	#[serde(rename = "Languages")]
	languages: Vec<Texts$Language>,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateSettings {
	#[serde(rename = "Certificates")]
	certificates: Vec<Settings$Certificate>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowsProjectSettingsPart {
	#[serde(rename = "DefaultTaskParallelism")]
	default_task_parallelism: i64,
	#[serde(rename = "UserEntity")]
	user_entity: String,
	#[serde(rename = "UsertaskOnStateChangeEvent")]
	usertask_on_state_change_event: Null,
	#[serde(rename = "WorkflowEngineParallelism")]
	workflow_engine_parallelism: i64,
	#[serde(rename = "WorkflowOnStateChangeEvent")]
	workflow_on_state_change_event: Null,
}

