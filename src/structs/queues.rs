use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct BasicQueueConfig {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ClusterWide")]
	cluster_wide: bool,
	#[serde(rename = "ParallelismExpression")]
	parallelism_expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct Queue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Config")]
	config: Option<queues::BasicQueueConfig>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueueSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Queue")]
	queue: String,
	#[serde(rename = "Retry")]
	retry: Option<Empty>,
}

