use std::collections::HashMap;

pub struct Collection {
    data: HashMap<String, String>,
}

impl Collection {
    pub fn new() -> Collection {
        Collection { data: HashMap::new() }
    }
    
    pub fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
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
    use std::collections::HashMap;


    #[test]
    fn put_and_get_value() {
        let mut coll = Collection::new();
        coll.put("key1".to_string(), "value1".to_string());
        assert_eq!(coll.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let coll = Collection::new();
        assert_eq!(coll.get("missing"), None);
    }

    #[test]
    fn delete_removes_key() {
        let mut coll = Collection::new();
        coll.put("key2".to_string(), "value2".to_string());
        coll.delete("key2".to_string());
        assert_eq!(coll.get("key2"), None);
    }

    #[test]
    fn overwrite_existing_key() {
        let mut coll = Collection::new();
        coll.put("key3".to_string(), "value3".to_string());
        coll.put("key3".to_string(), "value4".to_string());
        assert_eq!(coll.get("key3"), Some(&"value4".to_string()));
    }
}