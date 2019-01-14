#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate uuid;
extern crate actix_web;
#[macro_use]
extern crate lazy_static;

use actix_web::{server, App, HttpRequest, Responder, fs, Json, Result, http};
mod profiles;

fn logged(_: &HttpRequest) -> impl Responder {
    ""
}

fn login(_: HttpRequest) -> Result<Json<profiles::Profile>> {
    Ok(Json(profiles::login_profile()))
}

fn main() {
    println!("Starting server on http://127.0.0.1:8000");
    server::new(|| {
        App::new()
            .resource("api/logged", |r| r.f(logged))
            .resource("api/login", |r| r.method(http::Method::GET).with(login))
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
