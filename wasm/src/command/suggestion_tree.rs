use std::collections::HashMap;

const COMMANDS: [&str; 3] = ["Filter", "Average", "Sum"];

#[derive(Default, Debug)]
pub struct TreeNode {
    is_leaf: bool,
    childrens: HashMap<u8, Box<TreeNode>>,
}

#[derive(Default, Debug)]
pub struct SuggestionTree {
    root: Option<TreeNode>,
}

impl SuggestionTree {
    pub fn append(&mut self, word: &str) {
        let mut ptr = self.root.get_or_insert_default();

        for byte in word.as_bytes() {
            ptr = ptr
                .childrens
                .entry(*byte)
                .or_insert_with(|| Box::new(TreeNode::default()));
        }

        ptr.is_leaf = true;
    }

    fn find(&self, word: &str) -> Option<&TreeNode> {
        let mut ptr = self.root.as_ref().unwrap();

        for byte in word.as_bytes() {
            ptr = ptr.childrens.get(byte)?;
        }

        Some(ptr)
    }

    fn get_suggestions(node: &TreeNode, word: &str, ret: &mut Vec<String>) {
        let mut s = word.to_string();

        if node.is_leaf {
            ret.push(s);
        } else {
            for (byte, node) in node.childrens.iter() {
                s.push(*byte as char);
                Self::get_suggestions(node, &s, ret);
            }
        }
    }

    pub fn suggest(&self, word: &str) -> Vec<String> {
        let mut ret = Vec::new();
        let node = self.find(word).unwrap();
        Self::get_suggestions(node, word, &mut ret);

        ret
    }

    pub fn base_tree() -> Self {
        let mut tree = Self::default();
        COMMANDS.iter().for_each(|&c| tree.append(c));
        tree
    }
}

#[cfg(test)]
mod test {
    use super::SuggestionTree;

    #[test]
    fn test() {
        let mut tree = SuggestionTree::default();
        tree.append("foo");
        tree.append("bar");
        tree.append("baz");

        let suggestions = tree.suggest("b");
        assert_eq!(suggestions.len(), 2);
    }
}
