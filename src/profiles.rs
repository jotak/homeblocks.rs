use super::oauth;
use super::users;
use super::files;
use std::path::Path;
use actix_web::error;

mod blocks;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    title: String,
    page: blocks::Page
}

fn list_user_profiles(user_id: i32) -> Vec<String> {
    let str_path = format!("users/{}", user_id);
    let path = Path::new(&str_path);
    if path.is_dir() {
        path.read_dir().map_err(|err| err.to_string())
            .map(|entries| entries.filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    entry.path().file_name().and_then(|file_name| {
                        let name = file_name.to_string_lossy().into_owned();
                        if name.ends_with(".json") {
                            Some(name.chars().take(file_name.len() - 5).collect::<String>())
                        } else {
                            None
                        }
                    })
                })
            }).collect::<Vec<_>>())
            .unwrap_or(vec![])
    } else {
        vec![]
    }
}

pub fn user_profiles(user: &users::UserInfo) -> Profile {
    let profiles = list_user_profiles(user.int_idx);
    Profile {
        title: format!("{}'s place", user.name),
        page: blocks::Page {
            blocks: vec![
                blocks::Block::Main { posx: 0, posy: 0, title: None },
                blocks::Block::Links {
                    posx: 1,
                    posy: 0,
                    title: Option::from(String::from("Profiles")),
                    links: profiles.iter().map(|p| {
                        blocks::Link {
                            url: format!("#/u/{}/{}", user.name, p),
                            title: p.clone(),
                            description: Option::None
                        }
                    }).collect::<Vec<_>>()
                },
            ]
        }
    }
}

pub fn load_page(user: &users::UserInfo, profile_name: &String) -> Result<blocks::Page, error::Error> {
    let str_path = format!("users/{}/{}.json", user.int_idx, profile_name);
    files::read_file(&str_path).map_err(|err| error::ErrorInternalServerError("Unable to read profile file"))
        .and_then(|content| serde_json::from_str(&content).map_err(|err| error::ErrorInternalServerError("Unable to deserialize profile")))
}

pub fn load_profile(user: &users::UserInfo, profile_name: &String) -> Result<Profile, error::Error> {
    load_page(user, profile_name).map(|page| {
        Profile {
            title: format!("{}'s {}", user.name, profile_name),
            page: page
        }
    })
}

pub fn login_profile() -> Profile {
    oauth::PROVIDERS.with(|providers| {
        let page = blocks::Page {
            blocks: vec![
                blocks::Block::Note {
                    posx: -1,
                    posy: -1,
                    title: Option::None,
                    note: String::from("<h3>Welcome to Homeblocks.net</h3><br/>Build your homepage, block after block!")
                },
                blocks::Block::Links {
                    posx: 0,
                    posy: 0,
                    title: Option::from(String::from("Login")),
                    links: providers.iter().map(|p| {
                        blocks::Link {
                            url: p.auth_url(),
                            title: String::from("Login with ") + p.name,
                            description: Option::None
                        }
                    }).collect::<Vec<_>>()
                },
                blocks::Block::Image {
                    posx: 0,
                    posy: -1,
                    title: Option::None,
                    links: vec![blocks::Link {
                        url: String::from("http://aws-cf.imdoc.fr/prod/photos/6/9/8/4936698/16029275/img-16029275b1f.jpg?v=11"),
                        title: String::from("A'Tuin"),
                        description: Option::None
                    }]
                }
            ]
        };
        Profile {
            title: String::from("login"),
            page: page
        }
    })
}
