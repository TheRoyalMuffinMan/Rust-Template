#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use rocket_db_pools::{mongodb, Database};

#[derive(Database)]
#[database("mongo")]
struct Mongo(mongodb::Client);

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Mongo::init()).mount("/", routes![hello])
}