use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ProjectSecurity {
	#[serde(rename = "AdminPassword")]
	admin_password: String,
	#[serde(rename = "AdminUserName")]
	admin_user_name: String,
	#[serde(rename = "AdminUserRole")]
	admin_user_role: String,
	#[serde(rename = "CheckSecurity")]
	check_security: bool,
	#[serde(rename = "DemoUsers")]
	demo_users: Vec<2, []>,
	#[serde(rename = "EnableDemoUsers")]
	enable_demo_users: bool,
	#[serde(rename = "EnableGuestAccess")]
	enable_guest_access: bool,
	#[serde(rename = "FileDocumentAccess")]
	file_document_access: Security$FileDocumentAccessRuleContainer,
	#[serde(rename = "GuestUserRole")]
	guest_user_role: String,
	#[serde(rename = "ImageAccess")]
	image_access: Security$ImageAccessRuleContainer,
	#[serde(rename = "PasswordPolicySettings")]
	password_policy_settings: Security$PasswordPolicySettings,
	#[serde(rename = "SecurityLevel")]
	security_level: String,
	#[serde(rename = "StrictMode")]
	strict_mode: bool,
	#[serde(rename = "StrictPageUrlCheck")]
	strict_page_url_check: bool,
	#[serde(rename = "UserRoles")]
	user_roles: Vec<Security$UserRole>,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleRole {
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageAccessRuleContainer {
	#[serde(rename = "AccessRules")]
	access_rules: Vec<3, []>,
}

#[derive(Serialize, Deserialize)]
pub struct FileDocumentAccessRuleContainer {
	#[serde(rename = "AccessRules")]
	access_rules: Vec<3, []>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRole {
	#[serde(rename = "CheckSecurity")]
	check_security: bool,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "GUID")]
	guid: Binary,
	#[serde(rename = "ManageableRoles")]
	manageable_roles: Vec<1, []>,
	#[serde(rename = "ManageAllRoles")]
	manage_all_roles: bool,
	#[serde(rename = "ManageUsersWithoutRoles")]
	manage_users_without_roles: bool,
	#[serde(rename = "ModuleRoles")]
	module_roles: Vec<String>,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleSecurity {
	#[serde(rename = "ModuleRoles")]
	module_roles: Vec<Security$ModuleRole>,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordPolicySettings {
	#[serde(rename = "MinimumLength")]
	minimum_length: i64,
	#[serde(rename = "RequireDigit")]
	require_digit: bool,
	#[serde(rename = "RequireMixedCase")]
	require_mixed_case: bool,
	#[serde(rename = "RequireSymbol")]
	require_symbol: bool,
}

