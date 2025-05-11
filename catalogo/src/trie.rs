use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return vec![];
            }
        }

        let mut results = Vec::new();
        let mut current_word = prefix.to_string();
        Self::collect_words(node, &mut current_word, &mut results);
        results
    }

    fn collect_words(node: &TrieNode, prefix: &mut String, results: &mut Vec<String>) {
        if node.is_end_of_word {
            results.push(prefix.clone());
        }
        for (ch, child) in &node.children {
            prefix.push(*ch);
            Self::collect_words(child, prefix, results);
            prefix.pop();
        }
    }
}