#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate actix_web;

use actix_web::{server, App, HttpRequest, Responder, fs, Json, Result, http};
mod profiles;

// fn greet(req: &HttpRequest) -> impl Responder {
//     let to = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", to)
// }

fn logged(_: &HttpRequest) -> impl Responder {
    ""
}

fn login(_: HttpRequest) -> Result<Json<profiles::Profile>> {
    Ok(Json(profiles::login_profile()))
}

fn main() {
    server::new(|| {
        App::new()
            .resource("api/logged", |r| r.f(logged))
            .resource("api/login", |r| r.method(http::Method::GET).with(login))
            .handler(
                "/",
                fs::StaticFiles::new("./public")
                    .unwrap()
                    .show_files_listing())
//            .resource("/", |r| r.f(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
