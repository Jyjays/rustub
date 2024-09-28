#[cfg(test)]
mod trie_test {
    use bustub_rust::primer::trie::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie = trie.put("hello".to_string(), 1);
        trie = trie.put("world".to_string(), 2);
        trie = trie.put("hello world".to_string(), 3);
        assert_eq!(*trie.get("hello".to_string()).unwrap().borrow(), 1);
    }
}

mod trie_store_test {
    use bustub_rust::primer::trie_store::*;
    #[test]
    fn test_trie_store() {
        let trie_store = TrieStore::<i32>::new();
        trie_store.put("hello".to_string(), 1);
        trie_store.put("world".to_string(), 2);
        trie_store.put("hello world".to_string(), 3);
        assert_eq!(*trie_store.get("hello".to_string()).unwrap().get_value().borrow(), 1);
    }
}