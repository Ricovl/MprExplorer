use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ProjectSecurity {
	#[serde(rename = "admin_password")]
	admin_password: String,
	#[serde(rename = "admin_user_name")]
	admin_user_name: String,
	#[serde(rename = "admin_user_role")]
	admin_user_role: String,
	#[serde(rename = "check_security")]
	check_security: bool,
	#[serde(rename = "demo_users")]
	demo_users: Vec<>,
	#[serde(rename = "enable_demo_users")]
	enable_demo_users: bool,
	#[serde(rename = "enable_guest_access")]
	enable_guest_access: bool,
	#[serde(rename = "file_document_access")]
	file_document_access: security::FileDocumentAccessRuleContainer,
	#[serde(rename = "guest_user_role")]
	guest_user_role: String,
	#[serde(rename = "image_access")]
	image_access: security::ImageAccessRuleContainer,
	#[serde(rename = "password_policy_settings")]
	password_policy_settings: security::PasswordPolicySettings,
	#[serde(rename = "security_level")]
	security_level: String,
	#[serde(rename = "strict_mode")]
	strict_mode: bool,
	#[serde(rename = "strict_page_url_check")]
	strict_page_url_check: bool,
	#[serde(rename = "user_roles")]
	user_roles: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleSecurity {
	#[serde(rename = "module_roles")]
	module_roles: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageAccessRuleContainer {
	#[serde(rename = "access_rules")]
	access_rules: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRole {
	#[serde(rename = "check_security")]
	check_security: bool,
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "guid")]
	guid: Binary,
	#[serde(rename = "manageable_roles")]
	manageable_roles: Vec<>,
	#[serde(rename = "manage_all_roles")]
	manage_all_roles: bool,
	#[serde(rename = "manage_users_without_roles")]
	manage_users_without_roles: bool,
	#[serde(rename = "module_roles")]
	module_roles: Vec<>,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleRole {
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordPolicySettings {
	#[serde(rename = "minimum_length")]
	minimum_length: i64,
	#[serde(rename = "require_digit")]
	require_digit: bool,
	#[serde(rename = "require_mixed_case")]
	require_mixed_case: bool,
	#[serde(rename = "require_symbol")]
	require_symbol: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FileDocumentAccessRuleContainer {
	#[serde(rename = "access_rules")]
	access_rules: Vec<>,
}

