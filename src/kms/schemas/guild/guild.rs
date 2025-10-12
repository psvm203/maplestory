use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Guild {
    pub oguild_id: String,
}
