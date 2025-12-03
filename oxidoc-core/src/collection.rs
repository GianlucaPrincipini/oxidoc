use serde_json::Value;
use std::collections::HashMap;

pub type Document = Value;

#[derive(Debug)]
pub struct Collection {
    data: HashMap<String, Document>,
}

impl Collection {
    pub fn new() -> Collection {
        Collection { data: HashMap::new() }
    }
    
    pub fn put(&mut self, key: String, value: Document) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Document> {
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

    fn make_json_object(val: &str) -> Document {
        let mut obj = serde_json::Map::new();
        obj.insert("field1".to_string(), Value::String(val.to_string()));
        Value::Object(obj)
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