use crate::primer::trie;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trie() {
        

        let value = Rc::new(RefCell::new(5));

        *value.borrow_mut() = 10;
        print!("{:?}", value);
    }
}
