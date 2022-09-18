use std::collections::HashMap;

use reqwest::{Client, Method};
use serde::Deserialize;

use crate::config;

#[derive(Deserialize)]
struct TokenResponse {
    pub access_token: String,
}

pub struct Mastodon {
    pub client: Client,
    pub authn_token: Option<String>,
    pub auth_code: String,
    pub authed: bool,
}

impl Mastodon {
    pub fn new(auth_code: String) -> Self {
        Mastodon {
            client: Client::new(),
            authn_token: None,
            auth_code,
            authed: false,
        }
    }

    pub fn new_with_auth(token: String) -> Self {
        Mastodon {
            client: Client::new(),
            authn_token: Some(token),
            auth_code: "".into(),
            authed: true,
        }
    }

    pub async fn login(&mut self) {
        let builder = self.client.request(Method::POST,
            format!("{}/oauth/token", config::CONFIG.credentials.instance_uri));
        let mut map = HashMap::new();
        map.insert("client_id", config::CONFIG.credentials.client_id.as_str());
        map.insert("client_secret", &config::CONFIG.credentials.client_secret);
        map.insert("redirect_uri", "urn:ietf:wg:oauth:2.0:oob");
        map.insert("grant_type", "authorization_code");
        map.insert("code", &self.auth_code);
        map.insert("scope", "read write follow push");

        let builder = builder.form(&map);
        let a = builder.send().await.unwrap().text().await.unwrap();
        let tk = serde_json::from_str::<TokenResponse>(&a).unwrap();
        println!("Logged in ok, bearer token is {}", tk.access_token);
        self.authn_token = Some(tk.access_token);
        self.authed = true;
    }

    pub async fn make_post(&self, post_text: &str) {
        let builder = self.client.request(Method::POST,
            format!("{}/api/v1/statuses", config::CONFIG.credentials.instance_uri));
        let builder = builder.header("Authorization",
            format!("Bearer {}", self.authn_token.as_ref().unwrap()));

        let mut map = HashMap::new();
        map.insert("status", post_text);
        map.insert("visibility", "public");

        let builder = builder.form(&map);
        builder.send().await.unwrap().text().await.unwrap();
    }
}
