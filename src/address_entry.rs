use patchable::Patchable;
use rocket::serde::json::json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json_with_id::JsonWithId;

#[derive(Deserialize, Clone)]
pub struct AddressEntry {
    name: String,
    address: String,
}

#[derive(Deserialize)]
pub struct AddressEntryPatch {
    name: Option<String>,
    address: Option<String>,
}

impl<Id: Serialize> JsonWithId<Id> for AddressEntry {
    fn as_value(&self, id: &Id) -> Value {
        json!({
            "id": id,
            "name": self.name,
            "address": self.address
        })
    }
}

impl Patchable<AddressEntryPatch> for AddressEntry {
    fn apply_patch(&mut self, patch: AddressEntryPatch) {
        self.name.apply_patch(patch.name);
        self.address.apply_patch(patch.address);
    }
}
