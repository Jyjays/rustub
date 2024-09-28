use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use crate::primer::trie::Trie;

use super::trie::TrieFn;
/// 
#[derive(Debug,PartialEq,Eq)]
pub struct ValueGuard<T:Default> {
    root : Trie<T>,
    value : Rc<RefCell<T>>,
}

impl<T:Default> ValueGuard<T> {
    pub fn new() -> Self {
        ValueGuard {
            root : Trie::<T>::new(),
            value : Rc::new(RefCell::new(T::default())),
        }
    }
    pub fn get_value(&self) -> Rc<RefCell<T>> {
        self.value.clone()
    }
    pub fn get_root(&self) -> Trie<T> {
        self.root.clone()
    }
}
/// TrieStore is a struct based on Trie, which supports concurrent operations.
#[derive(Debug)]
pub struct TrieStore<T:Default> {
    root : Arc<RwLock<Trie<T>>>,
}


impl<T:Default> TrieStore<T> {
    pub fn new() -> Self {
        TrieStore {
            root : Arc::new(RwLock::new(Trie::<T>::new())),
        }
    }

    pub fn get(&self, key: String) -> Option<ValueGuard<T>> {
        // get mutex
        let read_guard = self.root.read().unwrap();
        if let Some(value) = read_guard.get(key.clone()) {
            Some(ValueGuard { root: read_guard.clone(), value: value })
        } else {
            None
        }
    }
    pub fn put(&self, key: String, value: T) {
        let mut write_guard = self.root.write().unwrap();
        let new_root = write_guard.put(key.clone(), value);
        *write_guard = new_root;
    }
    pub fn remove(&self, key: String) {
        let mut write_guard = self.root.write().unwrap();
        let new_root = write_guard.remove(key.clone());
        *write_guard = new_root;
    }
}
