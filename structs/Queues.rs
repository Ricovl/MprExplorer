#[derive(Serialize, Deserialize)]
struct QueueSettings {
	#[serde(rename = "Queue")]
	queue: String,
	#[serde(rename = "Retry")]
	retry: Null,
}

#[derive(Serialize, Deserialize)]
struct BasicQueueConfig {
	#[serde(rename = "ClusterWide")]
	cluster_wide: Bool,
	#[serde(rename = "ParallelismExpression")]
	parallelism_expression: String,
}

#[derive(Serialize, Deserialize)]
struct Queue {
	#[serde(rename = "Config")]
	config: Queues$BasicQueueConfig,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
}

