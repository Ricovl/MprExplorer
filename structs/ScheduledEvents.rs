#[derive(Serialize, Deserialize)]
struct MinuteSchedule {
	#[serde(rename = "Multiplier")]
	multiplier: i64,
}

#[derive(Serialize, Deserialize)]
struct HourSchedule {
	#[serde(rename = "MinuteOffset")]
	minute_offset: i64,
	#[serde(rename = "Multiplier")]
	multiplier: i64,
}

#[derive(Serialize, Deserialize)]
struct DaySchedule {
	#[serde(rename = "HourOfDay")]
	hour_of_day: i64,
	#[serde(rename = "MinuteOfHour")]
	minute_of_hour: i64,
}

#[derive(Serialize, Deserialize)]
struct WeekSchedule {
	#[serde(rename = "Friday")]
	friday: Bool,
	#[serde(rename = "HourOfDay")]
	hour_of_day: i64,
	#[serde(rename = "MinuteOfHour")]
	minute_of_hour: i64,
	#[serde(rename = "Monday")]
	monday: Bool,
	#[serde(rename = "Saturday")]
	saturday: Bool,
	#[serde(rename = "Sunday")]
	sunday: Bool,
	#[serde(rename = "Thursday")]
	thursday: Bool,
	#[serde(rename = "Tuesday")]
	tuesday: Bool,
	#[serde(rename = "Wednesday")]
	wednesday: Bool,
}

#[derive(Serialize, Deserialize)]
struct ScheduledEvent {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Enabled")]
	enabled: Bool,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Interval")]
	interval: i64,
	#[serde(rename = "IntervalType")]
	interval_type: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OnOverlap")]
	on_overlap: String,
	#[serde(rename = "Schedule")]
	schedule: ScheduledEvents$MinuteSchedule,
	#[serde(rename = "StartDateTime")]
	start_date_time: DateTime,
	#[serde(rename = "TimeZone")]
	time_zone: String,
}

