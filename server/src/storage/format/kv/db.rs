use std::collections::HashMap;
use json_value::JsonObject;
use crate::storage::format::kv::collection::Collection;

pub struct Db {
    db: HashMap<String, Collection>,
}

impl Db {
    pub fn new() -> Db {
        Db { db: HashMap::new() }
    }
    
    pub fn create_collection(&mut self, name: String) {
        self.db.entry(name).or_insert_with(Collection::new);
    }
    
    pub fn safe_delete_collection(&mut self, name: &str) -> bool {
        if self.db.contains_key(name) && self.db.get(name).unwrap().is_empty() {
            self.db.remove(name);
            true
        } else {
            false 
        }
    }
    
    pub fn delete_collection(&mut self, name: &str) -> bool {
        self.db.remove(name);
        true 
    }
    
    pub fn put(&mut self, collection: String, key: String, value: JsonObject) {
        let coll = self.db.entry(collection).or_insert_with(Collection::new);
        coll.put(key, value);
    }

    pub fn get(&self, collection: &str, key: &str) -> Option<&JsonObject> {
        self.db
            .get(collection)
            .and_then(|coll| coll.get(key))
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

    fn make_json_object(val: &str) -> JsonObject {
        let mut obj = Map::new();
        obj.insert("data".to_string(), Value::String(val.to_string()));
        obj
    }

    #[test]
    fn create_collection_and_put_and_get() {
        let mut db = Db::new();
        db.create_collection("test_coll".to_string());
        let value = make_json_object("value1");
        db.put("test_coll".to_string(), "key1".to_string(), value.clone());
        assert_eq!(db.get("test_coll", "key1"), Some(&value));
    }

    #[test]
    fn get_nonexistent_collection_returns_none() {
        let db = Db::new();
        assert_eq!(db.get("missing", "key1"), None);
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let mut db = Db::new();
        db.create_collection("test_coll".to_string());
        assert_eq!(db.get("test_coll", "missing"), None);
    }

    #[test]
    fn delete_key_removes_value() {
        let mut db = Db::new();
        db.create_collection("test_coll".to_string());
        let value = make_json_object("value1");
        db.put("test_coll".to_string(), "key1".to_string(), value);
        db.delete("test_coll".to_string(), "key1".to_string());
        assert_eq!(db.get("test_coll", "key1"), None);
    }

    #[test]
    fn safe_delete_collection_only_if_empty() {
        let mut db = Db::new();
        db.create_collection("test_coll".to_string());
        assert!(db.safe_delete_collection("test_coll"));
        db.create_collection("test_coll".to_string());
        let value = make_json_object("value1");
        db.put("test_coll".to_string(), "key1".to_string(), value);
        assert!(!db.safe_delete_collection("test_coll"));
    }

    #[test]
    fn delete_collection_always_removes() {
        let mut db = Db::new();
        db.create_collection("test_coll".to_string());
        let value = make_json_object("value1");
        db.put("test_coll".to_string(), "key1".to_string(), value);
        assert!(db.delete_collection("test_coll"));
        assert_eq!(db.get("test_coll", "key1"), None);
    }
}
