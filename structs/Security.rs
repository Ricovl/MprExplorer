#[derive(Serialize, Deserialize)]
struct ModuleRole {
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct ModuleSecurity {
	#[serde(rename = "ModuleRoles")]
	module_roles: Vec<2, ["Security$ModuleRole"]>,
}

#[derive(Serialize, Deserialize)]
struct UserRole {
	#[serde(rename = "CheckSecurity")]
	check_security: Bool,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "GUID")]
	guid: Binary,
	#[serde(rename = "ManageableRoles")]
	manageable_roles: Vec<1, []>,
	#[serde(rename = "ManageAllRoles")]
	manage_all_roles: Bool,
	#[serde(rename = "ManageUsersWithoutRoles")]
	manage_users_without_roles: Bool,
	#[serde(rename = "ModuleRoles")]
	module_roles: Vec<1, ["String"]>,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectSecurity {
	#[serde(rename = "AdminPassword")]
	admin_password: String,
	#[serde(rename = "AdminUserName")]
	admin_user_name: String,
	#[serde(rename = "AdminUserRole")]
	admin_user_role: String,
	#[serde(rename = "CheckSecurity")]
	check_security: Bool,
	#[serde(rename = "DemoUsers")]
	demo_users: Vec<2, []>,
	#[serde(rename = "EnableDemoUsers")]
	enable_demo_users: Bool,
	#[serde(rename = "EnableGuestAccess")]
	enable_guest_access: Bool,
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
	strict_mode: Bool,
	#[serde(rename = "StrictPageUrlCheck")]
	strict_page_url_check: Bool,
	#[serde(rename = "UserRoles")]
	user_roles: Vec<2, ["Security$UserRole"]>,
}

#[derive(Serialize, Deserialize)]
struct PasswordPolicySettings {
	#[serde(rename = "MinimumLength")]
	minimum_length: i64,
	#[serde(rename = "RequireDigit")]
	require_digit: Bool,
	#[serde(rename = "RequireMixedCase")]
	require_mixed_case: Bool,
	#[serde(rename = "RequireSymbol")]
	require_symbol: Bool,
}

#[derive(Serialize, Deserialize)]
struct FileDocumentAccessRuleContainer {
	#[serde(rename = "AccessRules")]
	access_rules: Vec<3, []>,
}

#[derive(Serialize, Deserialize)]
struct ImageAccessRuleContainer {
	#[serde(rename = "AccessRules")]
	access_rules: Vec<3, []>,
}

