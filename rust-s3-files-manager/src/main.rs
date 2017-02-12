#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "OK"
}

fn main() {

    rocket::ignite().mount(
        "/api/1",
        routes![ping]
    ).launch();
}

#[cfg(test)]
mod test;
