use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Achievement {
    pub account_list: Vec<AccountList>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AccountList {
    pub account_id: String,
    pub achievement_achieve: Vec<AchievementAchieve>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AchievementAchieve {
    pub achievement_name: String,
    pub achievement_description: String,
}
