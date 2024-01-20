#[derive(Serialize, Deserialize)]
struct Certificate {
	#[serde(rename = "Data")]
	data: Binary,
	#[serde(rename = "Type")]
	type: String,
}

#[derive(Serialize, Deserialize)]
struct ConfigurationSettings {
	#[serde(rename = "Configurations")]
	configurations: Vec<3, ["Settings$ServerConfiguration"]>,
}

#[derive(Serialize, Deserialize)]
struct ConventionSettings {
	#[serde(rename = "ActionActivityDefaultColors")]
	action_activity_default_colors: Vec<3, ["Settings$ActionActivityDefaultColor"]>,
	#[serde(rename = "LowerCaseMicroflowVariables")]
	lower_case_microflow_variables: Bool,
}

#[derive(Serialize, Deserialize)]
struct ServerConfiguration {
	#[serde(rename = "ApplicationRootUrl")]
	application_root_url: String,
	#[serde(rename = "ConstantValues")]
	constant_values: Vec<3, ["Settings$ConstantValue"]>,
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
	database_use_integrated_security: Bool,
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
	open_admin_port: Bool,
	#[serde(rename = "OpenHttpPort")]
	open_http_port: Bool,
	#[serde(rename = "ServerPortNumber")]
	server_port_number: i64,
}

#[derive(Serialize, Deserialize)]
struct LanguageSettings {
	#[serde(rename = "DefaultLanguageCode")]
	default_language_code: String,
	#[serde(rename = "Languages")]
	languages: Vec<3, ["Texts$Language"]>,
}

#[derive(Serialize, Deserialize)]
struct JarDeploymentSettings {
	#[serde(rename = "Exclusions")]
	exclusions: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct ProjectSettings {
	#[serde(rename = "Settings")]
	settings: Vec<2, ["Forms$WebUIProjectSettingsPart", "Settings$IntegrationProjectSettingsPart", "Settings$ConfigurationSettings", "Settings$ModelSettings", "Settings$ConventionSettings", "Settings$LanguageSettings", "Settings$CertificateSettings", "Settings$WorkflowsProjectSettingsPart", "Settings$JarDeploymentSettings", "Settings$DistributionSettings"]>,
}

#[derive(Serialize, Deserialize)]
struct ActionActivityDefaultColor {
	#[serde(rename = "ActionActivityType")]
	action_activity_type: String,
	#[serde(rename = "BackgroundColor")]
	background_color: String,
}

#[derive(Serialize, Deserialize)]
struct WorkflowsProjectSettingsPart {
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

#[derive(Serialize, Deserialize)]
struct DistributionSettings {
	#[serde(rename = "BasedOnVersion")]
	based_on_version: String,
	#[serde(rename = "IsDistributable")]
	is_distributable: Bool,
	#[serde(rename = "Version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
struct ThemeModuleEntry {
	#[serde(rename = "ModuleName")]
	module_name: String,
}

#[derive(Serialize, Deserialize)]
struct IntegrationProjectSettingsPart {
}

#[derive(Serialize, Deserialize)]
struct ConstantValue {
	#[serde(rename = "ConstantId")]
	constant_id: String,
	#[serde(rename = "Value")]
	value: String,
}

#[derive(Serialize, Deserialize)]
struct CertificateSettings {
	#[serde(rename = "Certificates")]
	certificates: Vec<3, ["Settings$Certificate"]>,
}

#[derive(Serialize, Deserialize)]
struct ModelSettings {
	#[serde(rename = "AfterStartupMicroflow")]
	after_startup_microflow: String,
	#[serde(rename = "AllowUserMultipleSessions")]
	allow_user_multiple_sessions: Bool,
	#[serde(rename = "BcryptCost")]
	bcrypt_cost: i64,
	#[serde(rename = "BeforeShutdownMicroflow")]
	before_shutdown_microflow: String,
	#[serde(rename = "DefaultTimeZoneCode")]
	default_time_zone_code: String,
	#[serde(rename = "EnableDataStorageOptimisticLocking")]
	enable_data_storage_optimistic_locking: Bool,
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
	use_system_context_for_background_tasks: Bool,
}

