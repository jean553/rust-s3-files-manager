#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "OK"
}

fn main() {
    rocket::ignite().mount("/", routes![ping]).launch();
}
