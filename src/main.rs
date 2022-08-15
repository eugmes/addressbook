#[macro_use]
extern crate rocket;

use std::sync::RwLock;

use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use serde_json::Value;

pub mod address_entry;
pub mod database;
pub mod json_with_id;

use address_entry::{AddressEntry, AddressEntryPatch};
use database::Database;
use json_with_id::JsonWithId;

type SharedAddressData = RwLock<Database<AddressEntry>>;

#[get("/")]
fn contacts(db: &State<SharedAddressData>) -> Value {
    db.read().unwrap().as_value()
}

#[get("/<id>")]
fn contact(id: i64, db: &State<SharedAddressData>) -> Option<Value> {
    db.read().unwrap().get(&id).map(|ent| ent.as_value(&id))
}

#[post("/", data = "<new_entry>")]
fn add_contact(
    new_entry: Json<AddressEntry>,
    db: &State<SharedAddressData>,
    route: &Route,
) -> status::Created<Value> {
    let id = db.write().unwrap().insert(new_entry.0.clone());
    let location = format!("{}/{}", route.uri.path(), id);
    status::Created::new(location).body(new_entry.as_value(&id))
}

#[delete("/<id>")]
fn remove_contact(id: i64, db: &State<SharedAddressData>) -> Option<Value> {
    db.write().unwrap().remove(&id).map(|ent| ent.as_value(&id))
}

#[put("/<id>", data = "<new_entry>")]
fn replace_contact(
    id: i64,
    new_entry: Json<AddressEntry>,
    db: &State<SharedAddressData>,
) -> Option<Value> {
    db.write()
        .unwrap()
        .replace(&id, new_entry.0.clone())
        .map(|ent| ent.as_value(&id))
}

#[patch("/<id>", data = "<patch>")]
fn update_contact(
    id: i64,
    patch: Json<AddressEntryPatch>,
    db: &State<SharedAddressData>,
) -> Option<Value> {
    db.write()
        .unwrap()
        .update(&id, patch.0)
        .map(|ent| ent.as_value(&id))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(SharedAddressData::new(Database::new()))
        .mount(
            "/v2/contact",
            routes![
                contacts,
                contact,
                add_contact,
                remove_contact,
                replace_contact,
                update_contact
            ],
        )
}
