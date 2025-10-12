use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Character {
    pub ocid: String,
}
