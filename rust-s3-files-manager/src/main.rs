#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "OK"
}

#[get("/file")]
fn get_file() -> &'static str {
    "shrunk of the downloaded file"
}

fn main() {

    rocket::ignite().mount(
        "/api/1",
        routes![
            ping,
            get_file
        ]
    ).launch();
}

#[cfg(test)]
mod test;
