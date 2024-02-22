#[allow(unused_imports)] // yeah we have issues with skill
#[macro_use] extern crate rocket;
use rocket::serde::json::{Value, json};
use mongodb::{
    bson::{doc, Document},
    error::Error as MongoError,
    Client,
    Collection,
    Database,
};
use std::env;
use std::error::Error as ActualError;
use shared::{Colours};

struct MongoData { database: Database }

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
                "parent": "", // supposed to be null but because of rust's stance on being the most memory safe language of the yet,,,
                "content": "",
                "time": 0,
                "hasReplies": false,
            }
        ],
        "last": true
    })
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn ActualError>> {
    let mut url: String = "placeholder".to_string();
    dotenvy::dotenv()?;

    match env::var("WOT_MONGODB") {
        Ok(val) => url = format!("{val}"),
        Err(e) => println!("{e}"),
    }

    let client = Client::with_uri_str(url)
        .await
        .expect("Failed to connect to MongoDB, possibly skill issue on your end.");
    let mongo: Database = client.database("wasteof-time");

    rocket::build()
        .manage(mongo)
        .mount("/", routes![
            index,
            users,
            wall
        ])
        .launch()
        .await
        .expect("Rocket failed to launch, probably skill issue on Max's end.");

    Ok(())
}
