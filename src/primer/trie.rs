use crate::utils::logutil;
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    env::consts,
    hash::Hash,
    rc::Rc,
};

/// Trie is a tree data structure that is used to store a dynamic set of strings
/// It is used to store strings that can be represented as a sequence of characters
/// The root of the trie is an empty string

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TrieNode<T> {
    pub children: HashMap<char, Rc<RefCell<TrieNode<T>>>>,
    pub is_value_node: bool,
    pub value: Rc<RefCell<T>>,
}

pub trait TrieNodeFn<T> {
    fn Clone(&self) -> Box<TrieNode<T>>;
}

impl<T: Default> TrieNodeFn<T> for TrieNode<T> {
    fn Clone(&self) -> Box<TrieNode<T>> {
        Box::new(TrieNode {
            children: self.children.clone(),
            is_value_node: self.is_value_node,
            value: self.value.clone(),
        })
    }
}

impl<T:Default> TrieNode<T> {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_value_node: false,
            value: Rc::new(RefCell::new(T::default())),
        }
    }
    pub fn new_with_children(
        children: HashMap<char, Rc<RefCell<TrieNode<T>>>>,
        is_value_node: bool,
    ) -> Self {
        TrieNode {
            children,
            is_value_node,
            value: Rc::new(RefCell::new(T::default())),
        }
    }
    pub fn get_children(&self) -> &HashMap<char, Rc<RefCell<TrieNode<T>>>> {
        &self.children
    }
    pub fn get_is_value_node(&self) -> bool {
        self.is_value_node
    }
    pub fn set_is_value_node(&mut self, is_value_node: bool) {
        self.is_value_node = is_value_node;
    }
    pub fn get_child(&self, c: char) -> Option<Rc<RefCell<TrieNode<T>>>> {
        self.children.get(&c).map(|x| x.clone())
    }
    pub fn add_child(&mut self, c: char, node: Rc<RefCell<TrieNode<T>>>) {
        self.children.insert(c, node);
    }
    pub fn remove_child(&mut self, c: char) {
        self.children.remove(&c);
    }
    pub fn get_value(&self) -> Option<Rc<RefCell<T>>> {
        Some(self.value.clone())
    }
    pub fn set_value(&mut self, value: T) {
        self.value = Rc::new(RefCell::new(value));
    }
}
pub struct Trie<T: Default> {
    root: Rc<RefCell<TrieNode<T>>>,
}

impl<T: Default> Trie<T> {
    pub fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TrieNode::new())),
        }
    }
    pub fn new_with_root(root: Rc<RefCell<TrieNode<T>>>) -> Self {
        Trie { root }
    }
    pub fn get_root(&self) -> Rc<RefCell<TrieNode<T>>> {
        self.root.clone()
    }
}

pub trait TrieFn<T: Default> {
    // TODO : implement the string_view struct
    fn Get(&self, key: String) -> Option<Rc<RefCell<T>>>;
    fn Put(&self, key: String, value: T) -> Trie<T>;
    fn Remove(&self, key: String) -> Trie<T>;
}

impl<T: Default> TrieFn<T> for Trie<T> {
    fn Get(&self, key: String) -> Option<Rc<RefCell<T>>> {
        if self.get_root().borrow().children.is_empty() {
            return None;
        }
        let mut current_node = self.get_root().clone();
        for c in key.chars() {
            let child = current_node.borrow().get_child(c);
            match child {
                Some(x) => {
                    current_node = x;
                }
                None => {
                    return None;
                }
            }
        }
        if current_node.borrow().get_is_value_node() {
            return Some(current_node.borrow().get_value().unwrap());
        }
        None
    }
    fn Put(&self, key: String, value: T) -> Trie<T> {
        let root = self.get_root();
        if key.is_empty() {
            return Trie::<T>::new_with_root(root);
        }

        let mut current_node = root.clone();
        for i in key[0..key.len() - 1].chars() {
            if let Some(ch) = {
                let mut node_borrow = current_node.borrow_mut();
                node_borrow.get_child(i)
            } {
                current_node = ch.clone();
                continue;
            } else {
                let new_node = Rc::new(RefCell::new(TrieNode::new()));
                current_node.borrow_mut().add_child(i, new_node.clone());
                current_node = new_node;
            }
        }
        let last_char = key.chars().last().unwrap();
        
        if let Some(ch) = {
            let mut last_node = current_node.borrow_mut();
            last_node.get_child(last_char)
        } {
            current_node = ch.clone();
            current_node.borrow_mut().set_value(value);
            current_node.borrow_mut().set_is_value_node(true);

        } else {
            let new_node = Rc::new(RefCell::new(TrieNode::<T>::new()));
            current_node.borrow_mut().add_child(last_char, new_node.clone());
            let mut last_node = new_node.borrow_mut();
            last_node.set_value(value);
            last_node.set_is_value_node(true);
        }
        Trie::new_with_root(root)
    }

    fn Remove(&self, key: String) -> Trie<T> {
        unimplemented!()
    }
}
