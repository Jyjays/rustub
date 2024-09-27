#[cfg(test)]
mod trie_test {
    use bustub_rust::primer::trie::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie = trie.Put("hello".to_string(), 1);
        trie = trie.Put("world".to_string(), 2);
        assert!(trie.Get("hello".to_string()).is_some());
    }
}