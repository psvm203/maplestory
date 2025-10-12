use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EventNoticeDetail {
    pub title: String,
    pub url: String,
    pub contents: String,
    pub date: String,
    pub date_event_start: String,
    pub date_event_end: String,
}
