use bson::Bson;
use serde::{de::{self, SeqAccess, Visitor}, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, marker::PhantomData};
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


#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum ModuleDocuments {
	#[serde(rename = "Navigation$NavigationDocument")]
	NavigationDocument(navigation::NavigationDocument),
	#[serde(rename = "CustomIcons$CustomIconCollection")]
	CustomIconCollection(custom_icons::CustomIconCollection),
	#[serde(rename = "Menus$MenuDocument")]
	MenuDocument(menus::MenuDocument),
	#[serde(rename = "JavaActions$JavaAction")]
	JavaAction(java_actions::JavaAction),
	#[serde(rename = "Projects$Folder")]
	Folder(projects::Folder),
	#[serde(rename = "Settings$ProjectSettings")]
	ProjectSettings(settings::ProjectSettings),
	#[serde(rename = "ExportMappings$ExportMapping")]
	ExportMapping(export_mappings::ExportMapping),
	#[serde(rename = "DataSets$DataSet")]
	DataSet(data_sets::DataSet),
	#[serde(rename = "JsonStructures$JsonStructure")]
	JsonStructure(json_structures::JsonStructure),
	#[serde(rename = "Security$ModuleSecurity")]
	ModuleSecurity(security::ModuleSecurity),
	#[serde(rename = "JavaScriptActions$JavaScriptAction")]
	JavaScriptAction(java_script_actions::JavaScriptAction),
	#[serde(rename = "ScheduledEvents$ScheduledEvent")]
	ScheduledEvent(scheduled_events::ScheduledEvent),
	#[serde(rename = "Microflows$Rule")]
	Rule(microflows::Rule),
	#[serde(rename = "Projects$ModuleSettings")]
	ModuleSettings(projects::ModuleSettings),
	#[serde(rename = "Security$ProjectSecurity")]
	ProjectSecurity(security::ProjectSecurity),
	#[serde(rename = "WebServices$ImportedServiceImpl")]
	ImportedServiceImpl(web_services::ImportedServiceImpl),
	#[serde(rename = "Forms$Snippet")]
	Snippet(forms::Snippet),
	#[serde(rename = "Projects$ProjectConversion")]
	ProjectConversion(projects::ProjectConversion),
	#[serde(rename = "WebServices$PublishedService")]
	PublishedService(web_services::PublishedService),
	#[serde(rename = "Projects$Project")]
	Project(projects::Project),
	#[serde(rename = "Forms$Page")]
	Page(forms::Page),
	#[serde(rename = "DocumentTemplates$DocumentTemplate")]
	DocumentTemplate(document_templates::DocumentTemplate),
	#[serde(rename = "Forms$Layout")]
	Layout(forms::Layout),
	#[serde(rename = "ImportMappings$ImportMapping")]
	ImportMapping(import_mappings::ImportMapping),
	#[serde(rename = "Rest$PublishedRestService")]
	PublishedRestService(rest::PublishedRestService),
	#[serde(rename = "Images$ImageCollection")]
	ImageCollection(images::ImageCollection),
	#[serde(rename = "RegularExpressions$RegularExpression")]
	RegularExpression(regular_expressions::RegularExpression),
	#[serde(rename = "Projects$ModuleImpl")]
	ModuleImpl(projects::ModuleImpl),
	#[serde(rename = "Constants$Constant")]
	Constant(constants::Constant),
	#[serde(rename = "Enumerations$Enumeration")]
	Enumeration(enumerations::Enumeration),
	#[serde(rename = "XmlSchemas$XmlSchema")]
	XmlSchema(xml_schemas::XmlSchema),
	#[serde(rename = "MessageDefinitions$MessageDefinitionCollection")]
	MessageDefinitionCollection(message_definitions::MessageDefinitionCollection),
	#[serde(rename = "Queues$Queue")]
	Queue(queues::Queue),
	#[serde(rename = "Microflows$Nanoflow")]
	Nanoflow(microflows::Nanoflow),
	#[serde(rename = "DomainModels$DomainModel")]
	DomainModel(domain_models::DomainModel),
	#[serde(rename = "Forms$BuildingBlock")]
	BuildingBlock(forms::BuildingBlock),
	#[serde(rename = "Microflows$Microflow")]
	Microflow(microflows::Microflow),
	#[serde(rename = "Texts$SystemTextCollection")]
	SystemTextCollection(texts::SystemTextCollection),
}

pub fn deserialize_settings<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct VecVisitor<T> {
        marker: PhantomData<fn() -> T>,
    }

    impl<'de, T> Visitor<'de> for VecVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Vec<T>, A::Error>
        where
            A: SeqAccess<'de>,
        {
			// println!("<In of{:?}", self.marker);
            let mut values = Vec::new();

            // Skip the first element
            seq.next_element::<i32>()?.ok_or_else(|| de::Error::custom("Expected an integer at the beginning of the array"))?;

            // Deserialize the rest of the elements
            while let Some(value) = seq.next_element()? {
                values.push(value);
            }

			// println!(">Out of{:?}", self.marker);
            Ok(values)
        }
    }

    let visitor = VecVisitor { marker: PhantomData };
    deserializer.deserialize_seq(visitor)
}

fn serialize_settings<S, T>(settings: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut array = Vec::new();
    // Insert the first element as bson::Int32(1)
    array.push(Bson::Int32(1));

    // Serialize the rest of the elements and add them to the array
    for setting in settings {
        let bson_value = bson::to_bson(setting)
            .map_err(serde::ser::Error::custom)?;
        array.push(bson_value);
    }

    // Convert the array to a Bson::Array and serialize it
    Bson::Array(array).serialize(serializer)
}

// Usage example
// let serialized = bson::to_bson(&project_settings)?;