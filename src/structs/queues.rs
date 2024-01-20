use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Queue {
	#[serde(rename = "Config")]
	config: Queues$BasicQueueConfig,
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
	#[serde(rename = "Queue")]
	queue: String,
	#[serde(rename = "Retry")]
	retry: Null,
}

#[derive(Serialize, Deserialize)]
pub struct BasicQueueConfig {
	#[serde(rename = "ClusterWide")]
	cluster_wide: bool,
	#[serde(rename = "ParallelismExpression")]
	parallelism_expression: String,
}

