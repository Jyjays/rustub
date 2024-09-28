#[cfg(test)]
mod trie_test {
    use bustub_rust::primer::trie::*;

    #[test]
    fn test_trie() {
        let trie = Trie::<i32>::new();
        trie.put("hello".to_string(), 1);
        trie.put("world".to_string(), 2);
        trie.put("hello world".to_string(), 3);
        assert_eq!(*trie.get("hello".to_string()).unwrap().read().unwrap(), 1);
    }

    #[test]
    fn test_remove() {
        let trie = Trie::<i32>::new();
        trie.put("hello".to_string(), 1);
        trie.put("hel".to_string(), 2);
        trie.put("h".to_string(), 3);
        trie.remove("hello".to_string());
        assert!(trie.get("hello".to_string()).is_none());
        assert!(trie.get("hell" .to_string()).is_none());
        assert!(trie.get("hel" .to_string()).is_some());
    }
}

mod trie_store_test {

    use bustub_rust::primer::trie_store::*;
    #[test]
    fn test_trie_store() {
        // let trie_store = TrieStore::<i32>::new();
        // trie_store.put("hello".to_string(), 1);
        // trie_store.put("world".to_string(), 2);
        // trie_store.put("hello world".to_string(), 3);
        // assert_eq!(
        //     *trie_store
        //         .get("hello".to_string())
        //         .unwrap()
        //         .get_value()
        //         .borrow::<i32>(),
        //     1
        // );
    }

    #[test]
    fn test_trie_store_concurrent() {
        let trie_store = TrieStore::<i32>::new();
        let mut handles = vec![];
        for i in 0..10 {
            let trie_store = trie_store.clone();
            let handle = std::thread::spawn(move || {
                trie_store.put(format!("hello{}", i), i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        for i in 0..10 {
            assert_eq!(
                *trie_store
                    .get(format!("hello{}", i))
                    .unwrap()
                    .get_value()
                    .read().unwrap(),
                i
            );
        }
        
    }
}
