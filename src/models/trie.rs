#[derive(Debug)]
pub struct TrieTree {
    head: Option<Box<TrieNode>>
}

#[derive(Debug)]
struct TrieNode {
    pub letter: Option<char>,
    leaf: bool,
    values: Vec<String>,
    children: Vec<Box<TrieNode>>,
}

impl TrieTree {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert(&mut self, word: String, value: String) -> bool {
        if word.len() == 0 {
            return false;
        }
        match self.head {
            None => self.head = Some(Box::new(TrieNode {
                letter: None,
                leaf: false,
                values: vec![],
                children: vec![],
            })),
            _ => ()
        };

        match &mut self.head {
            Some(head) => head.insert(word, value),
            None => false,
        }
    }

    pub fn collect(&self, word: String) -> Vec<String> {
        match &self.head {
            Some(head) => head.collect(word),
            None => vec![],
        }
    }
}

impl TrieNode {
    pub fn collect(&self, mut word: String) -> Vec<String> {
        if word.len() == 0 {
            let mut out = match self.leaf {
                true => self.values.clone(),
                false => vec![],
            };
            for child in self.children.iter() {
                let mut ch = child.collect(word.clone());
                out.append(&mut ch);
            }
            return out;
        } else {
            let first_char = word.remove(0);
            let mut next_child_index = None;
            // do we have the child with letter?
            for i in 0..self.children.len() {
                if self.children[i].letter == Some(first_char) {
                    next_child_index = Some(i);
                    break;
                }
            }
            match next_child_index {
                None => vec![],
                Some(index) => self.children[index].collect(word)
            }
        }
    }

    fn insert(&mut self, mut word: String, value: String) -> bool {
        // we are the last node
        if word.len() == 0 {
            self.leaf = true;
            self.values.push(value);
            return true;
        } else {
            let first_char = word.remove(0);
            let mut next_child_index = None;
            // do we have the child with letter?
            for i in 0..self.children.len() {
                if self.children[i].letter == Some(first_char) {
                    next_child_index = Some(i);
                    break;
                }
            }
            // if not create one
            match next_child_index {
                None => self.children.push(Box::new(TrieNode {
                    letter: Some(first_char),
                    leaf: false,
                    values: vec![],
                    children: vec![],
                })),
                _ => ()
            }
            // recursive insert into next child
            let last_index = self.children.len() - 1;
            match next_child_index {
                Some(index) => self.children[index].insert(word, value),
                None => self.children[last_index].insert(word, value),
            }
        }
        
    }
}