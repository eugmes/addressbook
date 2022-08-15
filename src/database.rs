use std::collections::HashMap;

use patchable::Patchable;
use serde_json::Value;

use crate::json_with_id::JsonWithId;

pub struct Database<Entry> {
    next_id: i64,
    data: HashMap<i64, Entry>,
}

impl<Entry: JsonWithId<i64>> Database<Entry> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            data: HashMap::new(),
        }
    }

    pub fn as_value(&self) -> Value {
        let vec = self
            .data
            .iter()
            .map(|(id, entry)| entry.as_value(id))
            .collect();
        Value::Array(vec)
    }

    pub fn insert(&mut self, new_entry: Entry) -> i64 {
        let id = self.next_id;
        self.data.insert(id, new_entry);
        self.next_id += 1;
        id
    }

    pub fn remove(&mut self, id: &i64) -> Option<Entry> {
        self.data.remove(id)
    }

    pub fn get(&self, id: &i64) -> Option<&Entry> {
        self.data.get(id)
    }

    pub fn replace(&mut self, id: &i64, new_entry: Entry) -> Option<&Entry> {
        self.data.get_mut(id).map(|ent| -> &Entry {
            *ent = new_entry;
            ent
        })
    }

    pub fn update<Patch>(&mut self, id: &i64, patch: Patch) -> Option<&Entry>
    where
        Entry: Patchable<Patch>,
    {
        self.data.get_mut(id).map(|ent| -> &Entry {
            ent.apply_patch(patch);
            ent
        })
    }
}
