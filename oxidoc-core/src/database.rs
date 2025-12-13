use crate::collection::{Collection, Document};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    db: HashMap<String, Collection>,
}

impl Database {
    pub fn initialize() -> Database {
        Database { db: HashMap::new() }
    }

    pub fn create_collection(&mut self, name: String) {
        self.db.entry(name).or_insert_with(Collection::new);
    }

    pub fn delete_collection(&mut self, name: &str) {
        self.db.remove(name);
    }

    pub fn put(&mut self, collection: String, key: String, value: Document) {
        let coll = self.db.entry(collection).or_insert_with(Collection::new);
        coll.put(key, value);
    }

    pub fn get(&self, collection: &str, key: &str) -> Option<&Document> {
        self.db.get(collection).and_then(|coll| coll.get(key))
    }

    pub fn delete(&mut self, collection: String, key: String) {
        if let Some(coll) = self.db.get_mut(&collection) {
            coll.delete(key);
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    fn make_json_object(val: &str) -> Document {
        let mut obj = Map::new();
        obj.insert("data".to_string(), Value::String(val.to_string()));
        Value::Object(obj)
    }

    #[test]
    fn create_collection_and_put_and_get() {
        let mut db = Database::initialize();
        db.create_collection("test_coll".to_string());
        let value = make_json_object("value1");
        db.put("test_coll".to_string(), "key1".to_string(), value.clone());
        assert_eq!(db.get("test_coll", "key1"), Some(&value));
    }

    #[test]
    fn get_nonexistent_collection_returns_none() {
        let db = Database::initialize();
        assert_eq!(db.get("missing", "key1"), None);
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let mut db = Database::initialize();
        db.create_collection("test_coll".to_string());
        assert_eq!(db.get("test_coll", "missing"), None);
    }

    #[test]
    fn delete_key_removes_value() {
        let mut db = Database::initialize();
        db.create_collection("test_coll".to_string());
        let value = make_json_object("value1");
        db.put("test_coll".to_string(), "key1".to_string(), value);
        db.delete("test_coll".to_string(), "key1".to_string());
        assert_eq!(db.get("test_coll", "key1"), None);
    }

    #[test]
    fn delete_collection_always_removes() {
        let mut db = Database::initialize();
        db.create_collection("test_coll".to_string());
        let value = make_json_object("value1");
        db.put("test_coll".to_string(), "key1".to_string(), value);
        db.delete_collection("test_coll");
        assert_eq!(db.get("test_coll", "key1"), None);
    }
}
