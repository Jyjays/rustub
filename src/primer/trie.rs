use std::{cell::RefCell, collections::HashMap, env::consts, rc::Rc};

/// Trie is a tree data structure that is used to store a dynamic set of strings
/// It is used to store strings that can be represented as a sequence of characters
/// The root of the trie is an empty string
struct TrieNode{
    children : HashMap<char, Rc<RefCell<TrieNode>>>,
    is_value_node : bool,
}

pub trait TrieNode_fn {
    fn new() -> Self;
    fn Clone(&self) -> Box<TrieNode>;
}

impl TrieNode_fn for TrieNode {
    fn new() -> Self {
        TrieNode {
            children : HashMap::new(),
            is_value_node : false,
        }
    }
    fn Clone(&self) -> Box<TrieNode> {
        Box::new(
            TrieNode { 
                children : self.children.clone(),
                is_value_node : self.is_value_node,
             }
        )
    }
}

impl TrieNode {
    pub fn new_with_children(children : HashMap<char, Rc<RefCell<TrieNode>>>, is_value_node : bool) -> Self {
        TrieNode {
            children,
            is_value_node,
        }
    }
}
/// TrieNodeWithValue is a TrieNode with a value
/// The value is stored in a Rc<RefCell<T>> to allow for shared ownership and interior mutability
/// This is useful when the value is a struct that needs to be mutated
struct TrieNodeWithValue<T>{
    node : TrieNode,
    value : Rc<RefCell<T>>,
}

impl<T: Default> TrieNode_fn for TrieNodeWithValue<T> {
    fn new() -> Self {
        TrieNodeWithValue {
            node : TrieNode::new(),
            value : Rc::new(RefCell::new(T::default())),
        }
    }
    fn Clone(&self) -> Box<TrieNode> {
        Box::new(
            TrieNode { 
                children : self.node.children.clone(),
                is_value_node : self.node.is_value_node,
             }
        )
    }
}

impl<T> TrieNodeWithValue<T> {
    pub fn new_with_cv(children : HashMap<char, Rc<RefCell<TrieNode>>>, is_value_node : bool, value : Rc<RefCell<T>>) -> Self {
        TrieNodeWithValue {
            node : TrieNode::new_with_children(children, is_value_node),
            value,
        }
    }
    pub fn new_with_value(value : Rc<RefCell<T>>) -> Self {
        TrieNodeWithValue {
            node : TrieNode::new(),
            value,
        }
    }   
}

pub struct Trie {
    root : Rc<RefCell<TrieNode>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root : Rc::new(RefCell::new(TrieNode::new())),
        }
    }
    pub fn new_with_root(root : Rc<RefCell<TrieNode>>) -> Self {
        Trie {
            root,
        }
    }
}

pub trait Trie_fn<T> {
    // TODO : implement the string_view struct
    fn Get(key : String) -> Option<Rc<RefCell<T>>>;
    fn Put(key : String, value : T) -> Trie;
    fn Remove(key : String) -> Trie;
}