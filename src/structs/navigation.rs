use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct RoleBasedHomePage {
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "page")]
	page: String,
	#[serde(rename = "user_role")]
	user_role: String,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationProfile {
	#[serde(rename = "app_icon")]
	app_icon: String,
	#[serde(rename = "app_title")]
	app_title: texts::Text,
	#[serde(rename = "home_items")]
	home_items: Vec<>,
	#[serde(rename = "home_page")]
	home_page: navigation::HomePage,
	#[serde(rename = "kind")]
	kind: String,
	#[serde(rename = "login_page_settings")]
	login_page_settings: forms::FormSettings,
	#[serde(rename = "menu")]
	menu: menus::MenuItemCollection,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "offline_entity_configs")]
	offline_entity_configs: Vec<>,
	#[serde(rename = "progressive_web_app_settings")]
	progressive_web_app_settings: Null,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationDocument {
	#[serde(rename = "profiles")]
	profiles: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct HomePage {
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "page")]
	page: String,
}

