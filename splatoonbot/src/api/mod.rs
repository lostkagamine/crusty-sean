mod internal;

pub struct Stage(pub String);
pub struct Weapon(pub String);

pub enum RankedGameMode {
    ClamBlitz,
    TowerControl,
    SplatZones,
    Rainmaker,
}

impl RankedGameMode {
    pub fn from_rule(rule: &str) -> Self {
        match rule {
            "AREA" => RankedGameMode::SplatZones,
            "CLAM" => RankedGameMode::ClamBlitz,
            "GOAL" => RankedGameMode::Rainmaker,
            "LOFT" => RankedGameMode::TowerControl,
            a @ _ => panic!("wtf, found unknown rule {a}")
        }
    }

    pub fn get_en_name(&self) -> &str {
        match self {
            RankedGameMode::ClamBlitz => "Clam Blitz",
            RankedGameMode::TowerControl => "Tower Control",
            RankedGameMode::SplatZones => "Splat Zones",
            RankedGameMode::Rainmaker => "Rainmaker",
        }
    }
}

pub enum AnarchyType {
    Series,
    Open,
}

pub struct TurfWarSchedule {
    pub ends_at: chrono::DateTime<chrono::FixedOffset>,
    pub stages: Vec<Stage>,
}

pub struct AnarchySchedule {
    pub ends_at: chrono::DateTime<chrono::FixedOffset>,
    pub stages: Vec<Stage>,
    pub mode: RankedGameMode,
    pub mode_type: AnarchyType,
}

pub struct SalmonRunSchedule {
    pub ends_at: chrono::DateTime<chrono::FixedOffset>,
    pub stage: Stage,
    pub weapons: Vec<Weapon>,
}

pub struct Schedules {
    pub turf_war: TurfWarSchedule,
    pub anarchy_series: AnarchySchedule,
    pub anarchy_open: AnarchySchedule,
    pub salmon_run: SalmonRunSchedule,
}

pub async fn get_latest_schedules() -> Schedules {
    let raw_data = reqwest::get("https://splatoon3.ink/data/schedules.json")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let deser_data = serde_json::from_str::<internal::ApiResponse>(&raw_data).unwrap();

    let turf_war_schedule = &deser_data.data.regular_schedules.nodes[0];
    let turf_sched_new = TurfWarSchedule {
        ends_at: chrono::DateTime::parse_from_rfc3339(&turf_war_schedule.end_time).unwrap(),
        stages: turf_war_schedule.regular_match_setting
            .vs_stages.iter().map(|a| Stage(a.name.clone())).collect()
    };

    let anarchy_schedule_api =
        &deser_data.data.bankara_schedules.nodes[0];
    
    let anarchy_schedule_api_series = anarchy_schedule_api.bankara_match_settings.iter()
        .find(|a| a.mode == Some("CHALLENGE".into())).unwrap();
    let anarchy_schedule_api_open = anarchy_schedule_api.bankara_match_settings.iter()
        .find(|a| a.mode == Some("OPEN".into())).unwrap();
    
    let anarchy_sched_series = AnarchySchedule {
        ends_at: chrono::DateTime::parse_from_rfc3339(&anarchy_schedule_api.end_time).unwrap(),
        mode_type: AnarchyType::Series,
        stages: anarchy_schedule_api_series.vs_stages.iter()
            .map(|a| Stage(a.name.clone())).collect(),
        mode: RankedGameMode::from_rule(&anarchy_schedule_api_series.vs_rule.rule)
    };

    let anarchy_sched_open = AnarchySchedule {
        ends_at: chrono::DateTime::parse_from_rfc3339(&anarchy_schedule_api.end_time).unwrap(),
        mode_type: AnarchyType::Open,
        stages: anarchy_schedule_api_open.vs_stages.iter()
            .map(|a| Stage(a.name.clone())).collect(),
        mode: RankedGameMode::from_rule(&anarchy_schedule_api_open.vs_rule.rule)
    };

    let salmon_schedule_api =
        &deser_data.data.coop_grouping_schedule.regular_schedules.nodes[0];
    
    let salmon_sched = SalmonRunSchedule {
        ends_at: chrono::DateTime::parse_from_rfc3339(&salmon_schedule_api.end_time).unwrap(),
        stage: Stage(salmon_schedule_api.setting.coop_stage.name.clone()),
        weapons: salmon_schedule_api.setting.weapons.iter()
            .map(|a| Weapon(a.name.clone())).collect()
    };

    Schedules {
        turf_war: turf_sched_new,
        anarchy_series: anarchy_sched_series,
        anarchy_open: anarchy_sched_open,
        salmon_run: salmon_sched
    }
}