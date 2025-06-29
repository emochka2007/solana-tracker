use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApiResponse {
    pub data: Vec<DataEntry>,
    pub start: String,
    pub end: String,
    pub cumulative_total: CumulativeTotal,
    pub daily_average: DailyAverage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataEntry {
    pub languages: Vec<Metric>,
    pub grand_total: GrandTotal,
    pub editors: Vec<Metric>,
    pub operating_systems: Vec<Metric>,
    pub categories: Vec<Metric>,
    pub dependencies: Vec<Metric>,
    pub machines: Vec<MachineMetric>,
    pub projects: Vec<ProjectMetric>,
    pub range: DateRange,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metric {
    pub name: String,
    pub total_seconds: f64,
    pub digital: String,
    pub decimal: String,
    pub text: String,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GrandTotal {
    pub hours: u64,
    pub minutes: u64,
    pub total_seconds: f64,
    pub digital: String,
    pub decimal: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MachineMetric {
    pub name: String,
    pub total_seconds: f64,
    pub machine_name_id: String,
    pub digital: String,
    pub decimal: String,
    pub text: String,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectMetric {
    pub name: String,
    pub total_seconds: f64,
    pub color: Option<String>,
    pub digital: String,
    pub decimal: String,
    pub text: String,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DateRange {
    pub start: String,
    pub end: String,
    pub date: String,
    pub text: String,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CumulativeTotal {
    pub seconds: f64,
    pub text: String,
    pub digital: String,
    pub decimal: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DailyAverage {
    pub holidays: u32,
    pub days_minus_holidays: u32,
    pub days_including_holidays: u32,
    pub seconds: u64,
    pub seconds_including_other_language: u64,
    pub text: String,
    pub text_including_other_language: String,
}
