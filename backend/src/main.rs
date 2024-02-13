#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]
fn index() -> Value {
    json!({
        "ok": "ok",
        "frontend": "",
        "uptime": ""
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
