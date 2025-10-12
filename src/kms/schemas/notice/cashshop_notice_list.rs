use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CashshopNoticeList {
    pub cashshop_notice: Vec<CashshopNotice>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CashshopNotice {
    pub title: String,
    pub url: String,
    pub notice_id: i64,
    pub date: String,
    pub date_sale_start: Option<String>,
    pub date_sale_end: Option<String>,
    pub ongoing_flag: String,
}
