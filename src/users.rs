use super::files;

const INDEX: &str = "users/_index.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub int_idx: i32,
    pub name: String,
    prov: String,
    prov_u_id: String,
}

pub fn load_users() -> Vec<UserInfo> {
    let content = files::read_file(INDEX).expect("Could not open users index file");
    serde_json::from_str(&content).expect("Could not deserialize index file")
}

fn provider_key(pname: &String, id: &String) -> String { format!("{}-{}", pname, id) }

pub struct UsersService {
    users: Vec<UserInfo>,
    max_id: i32,
}

impl UsersService {
    pub fn new(users: Vec<UserInfo>) -> Self {
        let max_id = users.iter().fold(0, |max, user| if user.int_idx > max { user.int_idx } else { max });
        UsersService {
            users: users,
            max_id: max_id,
        }
    }

    pub fn is_alias_available(&self, user_alias: String) -> bool {
        if user_alias.starts_with("@user") {
            // Reserved for internal alias generation
            false
        } else {
            !self.users.iter().any(|u| u.name == user_alias)
        }
    }

    fn update_users(&mut self, user_info: UserInfo) -> Result<(), String> {
        self.users.retain(|u| u.int_idx != user_info.int_idx);
        self.users.push(user_info);
        serde_json::to_string_pretty(&self.users)
            .map_err(|err| err.to_string())
            .and_then(|content| {
                files::write_file(INDEX, content).map_err(|err| err.to_string())
            })
    }

    pub fn find_or_create(&mut self, prov: String, prov_u_id: String) -> Result<UserInfo, String> {
        match self.users.iter().find(|user| { user.prov == prov && user.prov_u_id == prov_u_id }) {
            Some(user) => Ok(user.clone()),
            None => {
                self.max_id = self.max_id + 1;
                let new_user = UserInfo {
                    prov: prov,
                    prov_u_id: prov_u_id,
                    name: format!("@user{}", self.max_id),
                    int_idx: self.max_id,
                };
                self.update_users(new_user.clone()).map(|_| new_user)
            }
        }
    }

    pub fn save_alias(&mut self, id: i32, user_alias: String) -> Result<UserInfo, String> {
        if !self.is_alias_available(user_alias.to_owned()) {
            return Err(String::from("Alias not available"));
        }
        match self.users.iter().find(|user| { user.int_idx == id }) {
            Some(user) => {
                let new_user = UserInfo {
                    prov: user.prov.to_owned(),
                    prov_u_id: user.prov_u_id.to_owned(),
                    name: user_alias,
                    int_idx: id,
                };
                self.update_users(new_user.clone()).map(|_| new_user)
            },
            None => Err(String::from("Could not find existing user"))
        }
    }

    pub fn find_by_id(&self, id: i32) -> Option<&UserInfo> {
        self.users.iter().find(|user| { user.int_idx == id })
    }

    pub fn find_by_alias(&self, name: String) -> Option<&UserInfo> {
        self.users.iter().find(|user| { user.name == name })
    }
}
