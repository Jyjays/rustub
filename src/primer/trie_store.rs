use std::sync::{Arc, RwLock};

use crate::primer::trie::Trie;

use super::trie::TrieFn;

/// ValueGuard wraps the Trie and a reference to the value, allowing access to the value.
#[derive(Debug)]
pub struct ValueGuard<T: Default> {
    root: Trie<T>,
    value: Arc<RwLock<T>>,
}

impl<T: Default> ValueGuard<T> {
    pub fn new() -> Self {
        ValueGuard {
            root: Trie::<T>::new(),
            value: Arc::new(RwLock::new(T::default())),
        }
    }

    pub fn get_value(&self) -> Arc<RwLock<T>> {
        self.value.clone()
    }

    pub fn get_root(&self) -> Trie<T> {
        self.root.clone()
    }
}

/// TrieStore is a struct based on Trie, which supports concurrent operations using RwLock.
#[derive(Debug)]
pub struct TrieStore<T: Default> {
    root: Arc<RwLock<Trie<T>>>,
}

impl<T: Default> TrieStore<T> {
    /// Creates a new TrieStore with an empty Trie.
    pub fn new() -> Self {
        TrieStore {
            root: Arc::new(RwLock::new(Trie::<T>::new())),
        }
    }

    /// Clones the TrieStore, sharing the same underlying data.
    pub fn clone(&self) -> TrieStore<T> {
        TrieStore {
            root: self.root.clone(),
        }
    }

    /// Gets the value associated with the given key, if it exists, wrapped in a ValueGuard.
    pub fn get(&self, key: String) -> Option<ValueGuard<T>> {
        let read_guard = self.root.read().unwrap(); // Acquire a read lock
        if let Some(value) = read_guard.get(key.clone()) {
            Some(ValueGuard {
                root: read_guard.clone(),
                value: value,
            })
        } else {
            None
        }
    }

    /// Inserts the given value into the Trie associated with the specified key.
    pub fn put(&self, key: String, value: T) {
        let mut write_guard = self.root.write().unwrap(); // Acquire a write lock
        let new_root = write_guard.put(key.clone(), value);
        *write_guard = new_root;
    }

    /// Removes the value associated with the given key from the Trie.
    pub fn remove(&self, key: String) {
        let mut write_guard = self.root.write().unwrap(); // Acquire a write lock
        let new_root = write_guard.remove(key.clone());
        *write_guard = new_root;
    }
}
