#![feature(proc_macro_hygiene)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

pub mod apis;
pub mod models;


#[rocket::catch(404)]
fn not_found() -> rocket_contrib::json::JsonValue {
    rocket_contrib::json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn server() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", rocket::routes![crate::apis::table::table_update])
        .register(rocket::catchers![not_found])
}
