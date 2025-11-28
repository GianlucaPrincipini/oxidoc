use json_value::JsonObject;
use std::collections::HashMap;

type Document = JsonObject;

#[derive(Debug)]
pub struct Collection {
    data: HashMap<String, Document>,
}

impl Collection {
    pub fn new() -> Collection {
        Collection { data: HashMap::new() }
    }
    
    pub fn put(&mut self, key: String, value: JsonObject) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&JsonObject> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: String) {
        self.data.remove(&key);
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_value::JsonValue;

    fn make_json_object(val: &str) -> JsonObject {
        let mut obj = JsonObject::new();
        obj.insert("field1".to_string(), JsonValue::String(val.to_string()));
        obj
    }

    fn json_str_to_object(json: &str) -> JsonObject {
        let value: json_value::JsonValue = serde_json::from_str(json).expect("Invalid JSON string");
        match value {
            json_value::JsonValue::Object(obj) => obj,
            _ => panic!("Provided string is not a JSON object"),
        }
    }

    #[test]
    fn put_and_get_value() {
        let mut collection = Collection::new();
        let value = make_json_object("value1");
        collection.put("key1".to_string(), value.clone());
        println!("{:?}", collection);
        assert_eq!(collection.get("key1"), Some(&value));
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let collection = Collection::new();
        assert_eq!(collection.get("missing"), None);
    }

    #[test]
    fn delete_removes_value() {
        let mut collection = Collection::new();
        let value = make_json_object("value1");
        collection.put("key1".to_string(), value);
        collection.delete("key1".to_string());
        assert_eq!(collection.get("key1"), None);
    }

    #[test]
    fn is_empty_true_on_new_collection() {
        let collection = Collection::new();
        assert!(collection.is_empty());
    }

    #[test]
    fn is_empty_false_when_has_values() {
        let mut collection = Collection::new();
        let value = make_json_object("value1");
        collection.put("key1".to_string(), value);
        assert!(!collection.is_empty());
    }

    #[test]
    fn overwrite_existing_key() {
        let mut collection = Collection::new();
        let value1 = make_json_object("value1");
        let value2 = make_json_object("value2");
        collection.put("key1".to_string(), value1.clone());
        collection.put("key1".to_string(), value2.clone());
        assert_eq!(collection.get("key1"), Some(&value2));
        assert_ne!(collection.get("key1"), Some(&value1));
    }

    #[test]
    fn delete_nonexistent_key_does_nothing() {
        let mut collection = Collection::new();
        let value = make_json_object("value1");
        collection.put("key1".to_string(), value.clone());
        collection.delete("missing".to_string());
        assert_eq!(collection.get("key1"), Some(&value));
    }
}