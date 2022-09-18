use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigurationFile {
    pub credentials: Credentials,
    pub emotes: Emotes,
}

#[derive(Deserialize)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub instance_uri: String,
    pub auth_code: Option<String>,
}

#[derive(Deserialize)]
pub struct Emotes {
    pub turf_war: String,
    pub ranked: String,
    pub salmon_run: String,

    pub rainmaker: String,
    pub clam_blitz: String,
    pub splat_zones: String,
    pub tower_control: String,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: ConfigurationFile = {
        let file = std::fs::read_to_string("./config.toml").unwrap();
        let deser = toml::from_str(&file).unwrap();
        deser
    };
}