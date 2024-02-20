#[macro_use] extern crate rocket;
use rocket::serde::json::{Value, json};
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Client,
    Collection,
    Database,
};
use shared::{Colours};

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

#[get("/users/<username>/wall")]
fn wall(username: &str) -> Value {
    json!({
        "comments": [
            {
                "_id": "",
                "wall": {
                    "name": username,
                    "id": ""
                },
                "poster": {
                    "name": "",
                    "id": "",
                    "color": "",
                },
                "parent": None,
                "content": "",
                "time": 0,
                "hasReplies": false,
            }
        ],
        "last": true
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        users,
        wall
    ])
}
