use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterPropensity {
    pub date: Option<String>,
    pub charisma_level: i64,
    pub sensibility_level: i64,
    pub insight_level: i64,
    pub willingness_level: i64,
    pub handicraft_level: i64,
    pub charm_level: i64,
}
