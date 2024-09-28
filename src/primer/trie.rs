use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TrieNode<T> {
    pub children: HashMap<char, Arc<RwLock<TrieNode<T>>>>,
    pub is_value_node: bool,
    pub value: Arc<RwLock<T>>,
}

pub trait TrieNodeFn<T> {
    fn clone(&self) -> Box<TrieNode<T>>;
}

impl<T: Default> TrieNodeFn<T> for TrieNode<T> {
    fn clone(&self) -> Box<TrieNode<T>> {
        Box::new(TrieNode {
            children: self.children.clone(),
            is_value_node: self.is_value_node,
            value: self.value.clone(),
        })
    }
}

impl<T: Default> TrieNode<T> {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_value_node: false,
            value: Arc::new(RwLock::new(T::default())),
        }
    }
    pub fn new_with_children(
        children: HashMap<char, Arc<RwLock<TrieNode<T>>>>,
        is_value_node: bool,
    ) -> Self {
        TrieNode {
            children,
            is_value_node,
            value: Arc::new(RwLock::new(T::default())),
        }
    }
    pub fn get_children(&self) -> &HashMap<char, Arc<RwLock<TrieNode<T>>>> {
        &self.children
    }
    pub fn get_is_value_node(&self) -> bool {
        self.is_value_node
    }
    pub fn set_is_value_node(&mut self, is_value_node: bool) {
        self.is_value_node = is_value_node;
    }
    pub fn get_child(&self, c: char) -> Option<Arc<RwLock<TrieNode<T>>>> {
        self.children.get(&c).map(|x| x.clone())
    }
    pub fn add_child(&mut self, c: char, node: Arc<RwLock<TrieNode<T>>>) {
        self.children.insert(c, node);
    }
    pub fn remove_child(&mut self, c: char) {
        self.children.remove(&c);
    }
    pub fn get_value(&self) -> Option<Arc<RwLock<T>>> {
        Some(self.value.clone())
    }
    pub fn set_value(&mut self, value: T) {
        self.value = Arc::new(RwLock::new(value));
    }
}

#[derive(Debug)]
pub struct Trie<T: Default> {
    root: Arc<RwLock<TrieNode<T>>>,
}

impl<T: Default> Trie<T> {
    pub fn new() -> Self {
        Trie {
            root: Arc::new(RwLock::new(TrieNode::new())),
        }
    }
    pub fn new_with_root(root: Arc<RwLock<TrieNode<T>>>) -> Self {
        Trie { root }
    }
    pub fn get_root(&self) -> Arc<RwLock<TrieNode<T>>> {
        self.root.clone()
    }
    pub fn clone(&self) -> Trie<T> {
        Trie {
            root: self.root.clone(),
        }
    }
}

pub trait TrieFn<T: Default> {
    fn get(&self, key: String) -> Option<Arc<RwLock<T>>>;
    fn put(&self, key: String, value: T) -> Trie<T>;
    fn remove(&self, key: String) -> Trie<T>;
}

impl<T: Default> TrieFn<T> for Trie<T> {
    fn get(&self, key: String) -> Option<Arc<RwLock<T>>> {
        let root = self.get_root();
        let mut current_node = root.clone(); // Arc<RwLock<TrieNode<T>>>
        
        for c in key.chars() {
            let next_node_option = {
                let current_node_guard = current_node.read().unwrap(); // 拿到当前节点的读锁
                
                // 获取下一个节点，避免next_node被临时借用过早结束
                current_node_guard.get_child(c)
            };
        
            match next_node_option {
                Some(next_node) => {
                    current_node = next_node.clone(); // 更新current_node为next_node
                }
                None => {
                    return None;
                }
            }
        }
    
        let current_node_guard = current_node.read().unwrap(); // 最后一个节点的读锁
        if current_node_guard.get_is_value_node() {
            return Some(current_node_guard.get_value().unwrap().clone());
        }
    
        None
    }
        

    fn put(&self, key: String, value: T) -> Trie<T> {
        let root = self.get_root();
        if key.is_empty() {
            return Trie::<T>::new_with_root(root);
        }

        let mut current_node = root.clone();
        let mut current_node_guard = current_node.write().unwrap();

        for i in key[0..key.len() - 1].chars() {
            if let Some(child) = current_node_guard.get_child(i) {
                drop(current_node_guard); // Drop the guard before reassigning
                current_node = child.clone();
                current_node_guard = current_node.write().unwrap();
            } else {
                let new_node = Arc::new(RwLock::new(TrieNode::new()));
                current_node_guard.add_child(i, new_node.clone());
                drop(current_node_guard); // Drop the guard before reassigning
                current_node = new_node;
                current_node_guard = current_node.write().unwrap();
            }
        }

        let last_char = key.chars().last().unwrap();
        if let Some(child) = current_node_guard.get_child(last_char) {
            drop(current_node_guard); // Drop the guard before reassigning
            current_node = child.clone();
            let mut last_node_guard = current_node.write().unwrap();
            last_node_guard.set_value(value);
            last_node_guard.set_is_value_node(true);
        } else {
            let new_node = Arc::new(RwLock::new(TrieNode::new()));
            current_node_guard.add_child(last_char, new_node.clone());
            let mut last_node_guard = new_node.write().unwrap();
            last_node_guard.set_value(value);
            last_node_guard.set_is_value_node(true);
        }

        Trie::new_with_root(root)
    }

    fn remove(&self, key: String) -> Trie<T> {
        if key.is_empty() {
            return self.clone();
        }
        
        let root = self.get_root();
        let mut path = Vec::<Arc<RwLock<TrieNode<T>>>>::new();
        let mut current_node = root.clone();
        let mut current_node_guard = current_node.write().unwrap();

        for i in key[0..key.len() - 1].chars() {
            if let Some(child) = current_node_guard.get_child(i) {
                drop(current_node_guard);
                current_node = child.clone();
                current_node_guard = current_node.write().unwrap();
                path.push(current_node.clone());
            } else {
                return self.clone();
            }
        }

        let last_char = key.chars().last().unwrap();
        if let Some(child) = current_node_guard.get_child(last_char) {
            let mut child_guard = child.write().unwrap();
            if child_guard.get_children().is_empty() {
                current_node_guard.remove_child(last_char);
            } else {
                child_guard.set_is_value_node(false);
            }
        }
        drop(current_node_guard);
        let mut index = path.len();
        while index > 0 {
            let cur = path.pop().unwrap();
            let cur_guard = cur.read().unwrap();
            if {
                let cur_children = cur_guard.get_children();
                cur_children.is_empty() && !cur_guard.get_is_value_node()
            } {
                let parent = path.last().unwrap();
                let mut parent_guard = parent.write().unwrap();
                parent_guard.remove_child(key.chars().nth(index - 1).unwrap());  
                
            } else {
                break;
            }
            index -= 1;
        }
        Trie::new_with_root(root)
    }
}
