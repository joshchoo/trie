use std::collections::HashMap;

pub struct Trie {
    root: Node,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node::new(' '),
        }
    }
    pub fn insert(&mut self, word: &str) {
        // &mut borrow is required
        let mut current = &mut self.root;
        for c in word.chars() {
            current.is_leaf = false;
            if current.get_child(c).is_none() {
                current.add_child(Node::new(c));
            }
            current = current
                .get_child_mut(c)
                .unwrap_or_else(|| panic!("Could not find child for key: {}", c));
        }
        current.is_leaf = true;

        /*
        word.chars().for_each(|c| {
            current.is_leaf = false;
            if current.get_child(c).is_none() {
                current.add_child(Node::new(c));
            }
            // Error: "Borrowed data escapes outside of closure". Use a for loop instead to avoid this error.
            current = &mut current
                .get_child_mut(c)
                .expect(format!("Child node does not exist: {}", c).as_str());
        });
         */
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            match current.get_child(c) {
                Some(next) => current = next,
                None => return false,
            }
        }
        true
    }
}

pub struct Node {
    pub value: char,
    pub children: HashMap<char, Node>,
    pub is_leaf: bool,
}

impl Node {
    fn new(value: char) -> Self {
        Self {
            value,
            children: HashMap::new(),
            is_leaf: false,
        }
    }

    fn get_child(&self, value: char) -> Option<&Node> {
        self.children.get(&value)
    }

    fn get_child_mut(&mut self, value: char) -> Option<&mut Node> {
        self.children.get_mut(&value)
    }

    fn add_child(&mut self, node: Node) {
        self.children.insert(node.value, node);
    }
}

#[cfg(test)]
mod tests {
    use crate::Trie;

    #[test]
    fn returns_true_when_inserting_and_searching_for_inserted_word() {
        let mut trie = Trie::default();
        trie.insert("hello");
        let result = trie.search("hello");
        assert!(result)
    }

    #[test]
    fn returns_false_when_searching_for_non_existant_word() {
        let trie = Trie::default();
        let result = trie.search("hello");
        assert!(!result)
    }

    #[test]
    fn returns_true_when_searching_for_substring() {
        let mut trie = Trie::default();
        trie.insert("hello");
        let result = trie.search("he");
        assert!(result)
    }
}
