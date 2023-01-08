#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use rocket_db_pools::{mongodb, Database};

#[derive(Database)]
#[database("mongo")]
struct Mongo(mongodb::Client);

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Mongo::init()).mount("/", routes![hello])
}