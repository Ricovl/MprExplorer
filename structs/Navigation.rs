#[derive(Serialize, Deserialize)]
struct NavigationDocument {
	#[serde(rename = "Profiles")]
	profiles: Vec<2, ["Navigation$NavigationProfile"]>,
}

#[derive(Serialize, Deserialize)]
struct RoleBasedHomePage {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Page")]
	page: String,
	#[serde(rename = "UserRole")]
	user_role: String,
}

#[derive(Serialize, Deserialize)]
struct NavigationProfile {
	#[serde(rename = "AppIcon")]
	app_icon: String,
	#[serde(rename = "AppTitle")]
	app_title: Texts$Text,
	#[serde(rename = "HomeItems")]
	home_items: Vec<2, ["Navigation$RoleBasedHomePage"]>,
	#[serde(rename = "HomePage")]
	home_page: Navigation$HomePage,
	#[serde(rename = "Kind")]
	kind: String,
	#[serde(rename = "LoginPageSettings")]
	login_page_settings: Forms$FormSettings,
	#[serde(rename = "Menu")]
	menu: Menus$MenuItemCollection,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OfflineEntityConfigs")]
	offline_entity_configs: Vec<3, []>,
	#[serde(rename = "ProgressiveWebAppSettings")]
	progressive_web_app_settings: Null,
}

#[derive(Serialize, Deserialize)]
struct HomePage {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Page")]
	page: String,
}

