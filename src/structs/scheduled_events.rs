use bson::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct DaySchedule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "HourOfDay")]
	hour_of_day: i64,
	#[serde(rename = "MinuteOfHour")]
	minute_of_hour: i64,
}

#[derive(Serialize, Deserialize)]
pub struct HourSchedule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "MinuteOffset")]
	minute_offset: i64,
	#[serde(rename = "Multiplier")]
	multiplier: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MinuteSchedule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Multiplier")]
	multiplier: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ScheduledEvent {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Enabled")]
	enabled: bool,
	#[serde(rename = "Excluded")]
	excluded: bool,
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
	schedule: scheduled_events::MinuteSchedule,
	#[serde(rename = "StartDateTime")]
	start_date_time: DateTime,
	#[serde(rename = "TimeZone")]
	time_zone: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeekSchedule {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Friday")]
	friday: bool,
	#[serde(rename = "HourOfDay")]
	hour_of_day: i64,
	#[serde(rename = "MinuteOfHour")]
	minute_of_hour: i64,
	#[serde(rename = "Monday")]
	monday: bool,
	#[serde(rename = "Saturday")]
	saturday: bool,
	#[serde(rename = "Sunday")]
	sunday: bool,
	#[serde(rename = "Thursday")]
	thursday: bool,
	#[serde(rename = "Tuesday")]
	tuesday: bool,
	#[serde(rename = "Wednesday")]
	wednesday: bool,
}

