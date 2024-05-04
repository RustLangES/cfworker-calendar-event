use reqwest::Client;
use serde::{Deserialize, Serialize};
use worker::console_warn;

pub async fn get(
    client: &Client,
    calendar_id: &str,
    api_key: &str,
    min_time: &str,
    max_time: &str,
) -> Calendar {
    // URL Format details from: https://developers.google.com/calendar/api/v3/reference/events/list
    client
        .get(format!("https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events?timeMin={min_time}&timeMax={max_time}&timeZone=UTC-0&key={api_key}"))
        .send()
        .await
        .inspect_err(|e| console_warn!("Reqwest Error: {e:?}"))
        .unwrap()
        .json()
        .await
        .inspect_err(|e| console_warn!("Json Error: {e:?}"))
        .unwrap()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub kind: String,
    pub summary: String,
    pub description: Option<String>,
    pub updated: String,
    pub time_zone: String,
    pub items: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub kind: String,
    pub html_link: String,
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub creator: Creator,
    pub start: Start,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub email: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Start {
    pub date_time: String,
    pub time_zone: String,
}
