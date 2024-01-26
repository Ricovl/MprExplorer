use bson::Binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct FileDocumentAccessRuleContainer {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AccessRules", deserialize_with = "deserialize_settings")]
	access_rules: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageAccessRuleContainer {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AccessRules", deserialize_with = "deserialize_settings")]
	access_rules: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleRole {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleSecurity {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ModuleRoles", deserialize_with = "deserialize_settings")]
	module_roles: Vec<security::ModuleRole>,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordPolicySettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "MinimumLength")]
	minimum_length: i64,
	#[serde(rename = "RequireDigit")]
	require_digit: bool,
	#[serde(rename = "RequireMixedCase")]
	require_mixed_case: bool,
	#[serde(rename = "RequireSymbol")]
	require_symbol: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectSecurity {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AdminPassword")]
	admin_password: String,
	#[serde(rename = "AdminUserName")]
	admin_user_name: String,
	#[serde(rename = "AdminUserRole")]
	admin_user_role: String,
	#[serde(rename = "CheckSecurity")]
	check_security: bool,
	#[serde(rename = "DemoUsers", deserialize_with = "deserialize_settings")]
	demo_users: Vec<UnknownType>,
	#[serde(rename = "EnableDemoUsers")]
	enable_demo_users: bool,
	#[serde(rename = "EnableGuestAccess")]
	enable_guest_access: bool,
	#[serde(rename = "FileDocumentAccess")]
	file_document_access: Option<security::FileDocumentAccessRuleContainer>,
	#[serde(rename = "GuestUserRole")]
	guest_user_role: String,
	#[serde(rename = "ImageAccess")]
	image_access: Option<security::ImageAccessRuleContainer>,
	#[serde(rename = "PasswordPolicySettings")]
	password_policy_settings: Option<security::PasswordPolicySettings>,
	#[serde(rename = "SecurityLevel")]
	security_level: String,
	#[serde(rename = "StrictMode")]
	strict_mode: bool,
	#[serde(rename = "StrictPageUrlCheck")]
	strict_page_url_check: bool,
	#[serde(rename = "UserRoles", deserialize_with = "deserialize_settings")]
	user_roles: Vec<security::UserRole>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRole {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "CheckSecurity")]
	check_security: bool,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "GUID")]
	guid: Uuid,
	#[serde(rename = "ManageableRoles", deserialize_with = "deserialize_settings")]
	manageable_roles: Vec<String>,
	#[serde(rename = "ManageAllRoles")]
	manage_all_roles: bool,
	#[serde(rename = "ManageUsersWithoutRoles")]
	manage_users_without_roles: bool,
	#[serde(rename = "ModuleRoles", deserialize_with = "deserialize_settings")]
	module_roles: Vec<String>,
	#[serde(rename = "Name")]
	name: String,
}

