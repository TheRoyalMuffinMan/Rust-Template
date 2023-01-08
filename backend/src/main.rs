#[macro_use] extern crate rocket;

use rocket::{get, http::Status, serde::json::Json}

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

// Try visiting:
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

// #[get("/value")]
// fn value() -> Value {
//     json!({
//         "id": 83,
//         "values": [1, 2, 3, 4]
//     })
// }

#[get("/value")]
fn value() -> Result<Json<i32>, Status> {
    Ok(Json(32));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![world, wave])
        .mount("/", routes![value])
}