extern crate oauth2;

use oauth2::Config;
use std::env;
use std::collections::HashSet;
use uuid::Uuid;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Provider {
    pub name: &'static str,
    pub id: String,
    pub secret: String,
    pub base_auth_url: &'static str,
    pub token_url: &'static str,
}

thread_local!(pub static PROVIDERS: Vec<Provider> = vec![Provider {
    name: "Github",
    id: env::var("GITHUB_CLIENT_ID").unwrap_or(String::from("")),
    secret: env::var("GITHUB_CLIENT_SECRET").unwrap_or(String::from("")),
    base_auth_url: "https://github.com/login/oauth/authorize",
    token_url: "https://github.com/login/oauth/access_token"
}]);

lazy_static! {
    static ref STATES: Mutex<HashSet<String>> = { Mutex::new(HashSet::new()) };
}

impl Provider {
    pub fn auth_url(&self) -> String {
        let mut states = STATES.lock().unwrap();
        let state = Uuid::new_v4().to_string();
        states.insert(state.to_owned());
        let config = Config::new(
            self.id.to_owned(),
            self.secret.to_owned(),
            self.base_auth_url,
            self.token_url)
            .set_state(state);
        config.authorize_url().into_string()
    }
}
