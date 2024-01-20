use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ScheduledEvent {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "enabled")]
	enabled: bool,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "interval")]
	interval: i64,
	#[serde(rename = "interval_type")]
	interval_type: String,
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "on_overlap")]
	on_overlap: String,
	#[serde(rename = "schedule")]
	schedule: scheduled_events::HourSchedule,
	#[serde(rename = "start_date_time")]
	start_date_time: DateTime,
	#[serde(rename = "time_zone")]
	time_zone: String,
}

#[derive(Serialize, Deserialize)]
pub struct DaySchedule {
	#[serde(rename = "hour_of_day")]
	hour_of_day: i64,
	#[serde(rename = "minute_of_hour")]
	minute_of_hour: i64,
}

#[derive(Serialize, Deserialize)]
pub struct WeekSchedule {
	#[serde(rename = "friday")]
	friday: bool,
	#[serde(rename = "hour_of_day")]
	hour_of_day: i64,
	#[serde(rename = "minute_of_hour")]
	minute_of_hour: i64,
	#[serde(rename = "monday")]
	monday: bool,
	#[serde(rename = "saturday")]
	saturday: bool,
	#[serde(rename = "sunday")]
	sunday: bool,
	#[serde(rename = "thursday")]
	thursday: bool,
	#[serde(rename = "tuesday")]
	tuesday: bool,
	#[serde(rename = "wednesday")]
	wednesday: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MinuteSchedule {
	#[serde(rename = "multiplier")]
	multiplier: i64,
}

#[derive(Serialize, Deserialize)]
pub struct HourSchedule {
	#[serde(rename = "minute_offset")]
	minute_offset: i64,
	#[serde(rename = "multiplier")]
	multiplier: i64,
}

