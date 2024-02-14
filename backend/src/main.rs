// use std::env;
// use std::error::Error;
#[macro_use] extern crate rocket;
use mongodb::{
    bson::doc,
    sync::{Client, Collection}
};
// unused... for now, we will need this in the future
// use serde::{ Deserialize, Serialize };
use rocket::serde::json::{Value, json};
use shared::{Colours};

// dotenvy::dotenv();
// println!("{:#?}", env::var("WOT_MONGODB"));

// todo: use this for once
// const client = Client::with_uri_str(env::var("WOT_MONGODB"))?;

#[get("/")]
fn index() -> Value {
    json!({
        "ok": "ok",
        "frontend": "",
        "uptime": ""
    })
}

#[get("/users/<username>")]
fn users(username: &str) -> Value {
    json!({
        "name": username,
        "id": "",
        "bio": "",
        "verified": false,
        "permissions": {
            "admin": false,
            "banned": false
        },
        "beta": false,
        "color": "",
        "links": [],
        "history": {
            "joined": 0
        },
        "stats": {
            "followers": 0,
            "following": 0,
            "posts": 0
        },
        "online": false
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        users
    ])
}
