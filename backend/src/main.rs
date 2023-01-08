#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use rocket_db_pools::{mongodb, Database};

mod cors;

#[derive(Database)]
#[database("mongo")]
struct Mongo(mongodb::Client);

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/value")]
fn value() -> Result<Json<i32>, Status> {
    Ok(Json(45))
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Mongo::init()).attach(cors::Cors).mount("/", routes![value])
}
