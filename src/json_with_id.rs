use serde_json::Value;

/// Trait for types that can be serialized to JSON with an ID.
pub trait JsonWithId<Id> {
    /// Convert `self` to JSON, include `id` into the generated JSON object.
    fn as_value(&self, id: &Id) -> Value;
}
