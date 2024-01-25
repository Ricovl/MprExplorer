use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod code_actions;
pub mod constants;
pub mod custom_icons;
pub mod custom_widgets;
pub mod data_sets;
pub mod data_types;
pub mod document_templates;
pub mod domain_models;
pub mod enumerations;
pub mod export_mappings;
pub mod export_xml_action;
pub mod forms;
pub mod images;
pub mod import_mappings;
pub mod java_actions;
pub mod java_script_actions;
pub mod json_structures;
pub mod mappings;
pub mod menus;
pub mod message_definitions;
pub mod microflows;
pub mod navigation;
pub mod projects;
pub mod queues;
pub mod regular_expressions;
pub mod rest;
pub mod scheduled_events;
pub mod security;
pub mod settings;
pub mod texts;
pub mod web_services;
pub mod xml_schemas;



#[derive(Serialize, Deserialize)]
pub struct Empty {
	#[serde(rename = "$ID")]
	_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UnknownType {
	#[serde(rename = "$ID")]
	_id: Uuid,
}