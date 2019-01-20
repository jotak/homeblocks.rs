#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate actix_web;
#[macro_use]
extern crate lazy_static;

use actix_web::{server, App, error, HttpRequest, Path, Responder, fs, Json, Result, http};
mod profiles;
mod oauth;
mod files;
mod users;

lazy_static! {
    static ref USERS: users::UsersService = {
        let list = users::load_users();
        users::UsersService::new(list)
    };
}

fn logged(_: &HttpRequest) -> impl Responder {
    ""
}

fn login(_: HttpRequest) -> Result<Json<profiles::Profile>> {
    Ok(Json(profiles::login_profile()))
}

fn get_user(args: Path<String>) -> Result<Json<profiles::Profile>> {
    let user_name: String = args.into_inner();
    // TODO: get logged user
    USERS.find_by_alias(user_name)
        .ok_or(error::ErrorNotFound("User not found"))
        .map(|user| Json(profiles::user_profiles(user)))
}

fn get_profile(args: Path<(String, String)>) -> Result<Json<profiles::Profile>> {
    let user_name: String = args.0.clone();
    let profile_name: String = args.1.clone();
    // TODO: get logged user
    USERS.find_by_alias(user_name)
        .ok_or(error::ErrorNotFound("User not found"))
        .and_then(|user| profiles::load_profile(user, &profile_name).map(|p| Json(p)))
}

fn main() {
    println!("Starting server on http://127.0.0.1:8000");
    server::new(|| {
        App::new()
            .resource("api/logged", |r| r.f(logged))
            .resource("api/login", |r| r.method(http::Method::GET).with(login))
            .resource("/api/user/{user}", |r| r.method(http::Method::GET).with(get_user))
            .resource("/api/user/{user}/profile/{name}", |r| r.method(http::Method::GET).with(get_profile))
            .handler(
                "/",
                fs::StaticFiles::new("./public")
                    .unwrap()
                    .index_file("index.html"))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
