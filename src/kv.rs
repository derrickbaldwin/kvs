#![deny(missing_docs)]
use std::collections::HashMap;

/// Key-value stores string (key,value) pairs in momory
/// 
/// Example
/// 
/// ```rust
/// # use kvs::KvStore;
/// let mut map = KvStore::new();
/// map.set("yes".to_owned(), "oui".to_owned());
/// let french_word = map.get("yes".to_owned());
/// assert_eq!(french_word, Some("oui".to_owned()));
/// ```  
pub struct KvStore { 
    map: HashMap<String, String>
} 


impl KvStore {
    /// Initialze a keystore
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new()
        }        
    }
    /// Add a (key, value) pair to the memory store
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Fetch a value using corresponding key from the memory store
    pub fn get(&self, key: String) -> Option<String>{
        self.map.get(&key).cloned()
    }

    /// Delete a (key, value) pair from the memory store
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}


