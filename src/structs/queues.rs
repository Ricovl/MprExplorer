use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct BasicQueueConfig {
	#[serde(rename = "cluster_wide")]
	cluster_wide: bool,
	#[serde(rename = "parallelism_expression")]
	parallelism_expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct Queue {
	#[serde(rename = "config")]
	config: queues::BasicQueueConfig,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueueSettings {
	#[serde(rename = "queue")]
	queue: String,
	#[serde(rename = "retry")]
	retry: Empty,
}

