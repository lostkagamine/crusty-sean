use serde::Deserialize;

// internal API stuff
#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiStage {
    pub id: String,
    pub vs_stage_id: Option<i32>,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiVsRule {
    pub name: String,
    pub rule: String, //appears to be an enum of some kind
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiMatchSetting {
    pub vs_stages: Vec<ApiStage>,
    pub vs_rule: ApiVsRule,
    //appears to be set to "CHALLENGE" for series and "OPEN" for normal
    pub mode: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiRegularNode {
    pub start_time: String,
    pub end_time: String,
    pub regular_match_setting: ApiMatchSetting,
    pub fest_match_setting: Option<ApiMatchSetting>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiRegularNodeContainer {
    pub nodes: Vec<ApiRegularNode>,
}

// named "Bankara" internally idc
#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiAnarchyNode {
    pub start_time: String,
    pub end_time: String,
    // 2 of them - one is CHALLENGE the other is OPEN
    pub bankara_match_settings: Vec<ApiMatchSetting>,
    pub fest_match_setting: Option<ApiMatchSetting>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiAnarchyNodeContainer {
    pub nodes: Vec<ApiAnarchyNode>,
}

// it's one fucking field
#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiWeapon {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
// salmon run
pub struct ApiCoopSetting {
    pub coop_stage: ApiStage,
    // 4 of them
    pub weapons: Vec<ApiWeapon>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiCoopNode {
    pub start_time: String,
    pub end_time: String,
    pub setting: ApiCoopSetting,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiCoopNodeContainer {
    pub nodes: Vec<ApiCoopNode>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiCoopGroupingSchedule {
    pub regular_schedules: ApiCoopNodeContainer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiDataResponse {
    pub regular_schedules: ApiRegularNodeContainer,
    pub bankara_schedules: ApiAnarchyNodeContainer,
    pub coop_grouping_schedule: ApiCoopGroupingSchedule,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ApiResponse {
    pub data: ApiDataResponse,
}