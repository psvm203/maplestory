use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EventNoticeList {
    pub event_notice: Vec<EventNotice>,
}

#[derive(Debug, Deserialize)]
pub struct EventNotice {
    pub title: String,
    pub url: String,
    pub notice_id: i64,
    pub date: String,
    pub date_event_start: String,
    pub date_event_end: String,
}
