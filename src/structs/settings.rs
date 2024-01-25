use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ActionActivityDefaultColor {
	#[serde(rename = "action_activity_type")]
	action_activity_type: String,
	#[serde(rename = "background_color")]
	background_color: String,
}

#[derive(Serialize, Deserialize)]
pub struct Certificate {
	#[serde(rename = "data")]
	data: Binary,
	#[serde(rename = "_type")]
	_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateSettings {
	#[serde(rename = "certificates")]
	certificates: Vec<settings::Certificate, >,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigurationSettings {
	#[serde(rename = "configurations")]
	configurations: Vec<settings::ServerConfiguration, >,
}

#[derive(Serialize, Deserialize)]
pub struct ConstantValue {
	#[serde(rename = "constant_id")]
	constant_id: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConventionSettings {
	#[serde(rename = "action_activity_default_colors")]
	action_activity_default_colors: Vec<settings::ActionActivityDefaultColor, >,
	#[serde(rename = "lower_case_microflow_variables")]
	lower_case_microflow_variables: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DistributionSettings {
	#[serde(rename = "based_on_version")]
	based_on_version: String,
	#[serde(rename = "is_distributable")]
	is_distributable: bool,
	#[serde(rename = "version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
pub struct IntegrationProjectSettingsPart {
}

#[derive(Serialize, Deserialize)]
pub struct JarDeploymentSettings {
	#[serde(rename = "exclusions")]
	exclusions: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct LanguageSettings {
	#[serde(rename = "default_language_code")]
	default_language_code: String,
	#[serde(rename = "languages")]
	languages: Vec<texts::Language, >,
}

#[derive(Serialize, Deserialize)]
pub struct ModelSettings {
	#[serde(rename = "after_startup_microflow")]
	after_startup_microflow: String,
	#[serde(rename = "allow_user_multiple_sessions")]
	allow_user_multiple_sessions: bool,
	#[serde(rename = "bcrypt_cost")]
	bcrypt_cost: i64,
	#[serde(rename = "before_shutdown_microflow")]
	before_shutdown_microflow: String,
	#[serde(rename = "default_time_zone_code")]
	default_time_zone_code: String,
	#[serde(rename = "enable_data_storage_optimistic_locking")]
	enable_data_storage_optimistic_locking: bool,
	#[serde(rename = "first_day_of_week")]
	first_day_of_week: String,
	#[serde(rename = "hash_algorithm")]
	hash_algorithm: String,
	#[serde(rename = "health_check_microflow")]
	health_check_microflow: String,
	#[serde(rename = "rounding_mode")]
	rounding_mode: String,
	#[serde(rename = "scheduled_event_time_zone_code")]
	scheduled_event_time_zone_code: String,
	#[serde(rename = "use_system_context_for_background_tasks")]
	use_system_context_for_background_tasks: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectSettings {
	#[serde(rename = "settings")]
	settings: Vec<settings::WorkflowsProjectSettingsPart, settings::ConventionSettings, forms::WebUIProjectSettingsPart, settings::IntegrationProjectSettingsPart, settings::CertificateSettings, settings::DistributionSettings, settings::ConfigurationSettings, settings::LanguageSettings, settings::ModelSettings, settings::JarDeploymentSettings, >,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfiguration {
	#[serde(rename = "application_root_url")]
	application_root_url: String,
	#[serde(rename = "constant_values")]
	constant_values: Vec<settings::ConstantValue, >,
	#[serde(rename = "custom_settings")]
	custom_settings: Vec<UnknownType>,
	#[serde(rename = "database_name")]
	database_name: String,
	#[serde(rename = "database_password")]
	database_password: String,
	#[serde(rename = "database_type")]
	database_type: String,
	#[serde(rename = "database_url")]
	database_url: String,
	#[serde(rename = "database_use_integrated_security")]
	database_use_integrated_security: bool,
	#[serde(rename = "database_user_name")]
	database_user_name: String,
	#[serde(rename = "extra_jvm_parameters")]
	extra_jvm_parameters: String,
	#[serde(rename = "http_port_number")]
	http_port_number: i64,
	#[serde(rename = "max_java_heap_size")]
	max_java_heap_size: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "open_admin_port")]
	open_admin_port: bool,
	#[serde(rename = "open_http_port")]
	open_http_port: bool,
	#[serde(rename = "server_port_number")]
	server_port_number: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeModuleEntry {
	#[serde(rename = "module_name")]
	module_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowsProjectSettingsPart {
	#[serde(rename = "default_task_parallelism")]
	default_task_parallelism: i64,
	#[serde(rename = "user_entity")]
	user_entity: String,
	#[serde(rename = "usertask_on_state_change_event")]
	usertask_on_state_change_event: Empty,
	#[serde(rename = "workflow_engine_parallelism")]
	workflow_engine_parallelism: i64,
	#[serde(rename = "workflow_on_state_change_event")]
	workflow_on_state_change_event: Empty,
}

