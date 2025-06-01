use anyhow::{Ok, Result};
use core::fmt;
use std::cmp::min;
use std::fmt::Display;

#[derive(Debug)]
pub enum TrieNodeContent {
    Internal { children: Vec<usize> }, // Using i32 for child indices
    Leaf { data: char },
}

pub struct TrieNode {
    pub value: Box<[u8]>, //ASCII string
    pub parent: usize,
    pub content: TrieNodeContent,
}

impl TrieNode {
    // Adds child if the node is internal
    fn try_add_child(&mut self, child_idx: usize) -> bool {
        if let TrieNodeContent::Internal { children } = &mut self.content {
            children.push(child_idx);
            return true;
        }
        false
    }

    // Removes child if the node is internal
    fn try_remove_child(&mut self, child_idx: usize) -> bool {
        if let TrieNodeContent::Internal { children } = &mut self.content {
            if let Some(pos) = children.iter().position(|&x| x == child_idx) {
                children.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn value_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }
}

//root node index = 0
pub struct Trie {
    pub nodes: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        // Initialize a new Trie with an empty root node
        let root = TrieNode {
            value: Box::new([]),
            parent: 0,
            content: TrieNodeContent::Internal {
                children: Vec::new(),
            },
        };
        Trie { nodes: vec![root] }
    }

    /// Finds the index of the best match (1. largest match 2. higher in the tree) and return the length of the match
    ///
    /// The best match may be an internal node or a leaf
    ///
    /// Return type = (idx : usize, len : usize)
    pub fn find_max_match(&self, input: &[u8]) -> (usize, usize) {
        if input.len() == 0 {
            return (0, 0);
        } //we can assume length >= 1

        let mut ndidx: usize = 0; //index for node traversal
        let mut chidx: usize = 0; //index to keep track of the character needed for matching
                                  //chidx is always set to the index value that should be checked next

        'outer: loop {
            let cur_node = &self.nodes[ndidx];
            let nd_chars = &cur_node.value;

            // we try to make chidx large as possible (within the bounds)
            let min_len: usize = min(nd_chars.len(), input.len());
            while chidx < min_len && input[chidx] == nd_chars[chidx] {
                chidx += 1
            }

            if chidx == input.len() || chidx < nd_chars.len() {
                //this is the best match, case 1: the match length cannot get larger, case 2: we don't need to care children
                break;
            }
            //we can do cur_node[chidx] now

            match &cur_node.content {
                TrieNodeContent::Leaf { .. } => break, //if leaf, end

                TrieNodeContent::Internal { children } => {
                    //if internal, try finding a match among children
                    for child in children {
                        let ch_chars = &self.nodes[*child].value;
                        //child may have same value as parent! But we don't care because the parent is a better match
                        if ch_chars.len() > chidx && input[chidx] == ch_chars[chidx] {
                            chidx += 1;
                            ndidx = *child;
                            continue 'outer;
                        }
                    }
                    //no match was found!
                    break;
                }
            }
        }

        // we return chidx because it equals the length of max matching substring
        (ndidx, chidx)
    }

    /// Searches for the leaf with value "input", and returns its value
    ///
    /// Returns Err if input is not ascii or empty
    ///
    /// Also returns Err if there is no leaf which is an exact match
    pub fn find_value(&self, input: &str) -> Result<char> {
        if input.len() == 0 || !input.is_ascii() {
            return Err(anyhow::anyhow!("Input string is empty or is not ASCII"));
        }

        let (midx, mlen) = self.find_max_match(input.as_bytes());
        if mlen != input.len() {
            return Err(anyhow::anyhow!("No leaf with exact match was found"));
        }
        match &self.nodes[midx].content {
            TrieNodeContent::Internal { children } => {
                for cidx in children {
                    if self.nodes[*cidx].value.len() == mlen {
                        if let TrieNodeContent::Leaf { data } = &self.nodes[*cidx].content {
                            return Result::Ok(*data);
                        } else {
                            panic!("An internal node with the same name as its parent was found")
                        }
                    }
                }
                Err(anyhow::anyhow!("No leaf with exact match was found"))
            }
            TrieNodeContent::Leaf { data } => Result::Ok(*data),
        }
    }
    /// Appends a leaf and creates internal nodes if needed
    ///
    /// Value must be given as a nonempty ASCII string
    ///
    /// If the same leaf already exists the input will be ignored
    pub fn append_leaf(&mut self, input: String, data: char) -> Result<()> {
        assert!(input.len() > 0, "Value must be a non-empty string");
        assert!(input.is_ascii(), "Value must be an ASCII string");

        let in_chars = input.into_boxed_str().into_boxed_bytes();
        let (ndidx, match_len) = self.find_max_match(&in_chars);
        //case 1: internal && inclusion(match node value is strictly contained inside input string) -> add leaf as child of the match node
        //this is also the case when the match node is the root
        //case 2: internal && match node value = input string -> check is there is a child of same value. If not, add the leaf, else error
        //case 3: internal && conflict || leaf && nonperfect match -> add new node with matching substring and add the new leaf and matching node as children
        //case 4: leaf && perfect match -> error

        let match_node = &self.nodes[ndidx];

        match &match_node.content {
            TrieNodeContent::Internal { children } => {
                if match_len == match_node.value.len() {
                    //case 1 & case 2
                    if match_len < in_chars.len()
                        || children
                            .iter()
                            .all(|chidx| self.nodes[*chidx].value.len() > match_len)
                    {
                        let leaf_idx = self.nodes.len();
                        self.nodes.push(TrieNode {
                            value: in_chars,
                            parent: ndidx,
                            content: TrieNodeContent::Leaf { data },
                        });
                        debug_assert!(self.nodes[ndidx].try_add_child(leaf_idx));
                        return Ok(()); //case 1 & case 2
                    }
                    return Err(anyhow::anyhow!("Leaf with same value already exists"));
                }
                //else: continue on to case 3
            }
            TrieNodeContent::Leaf { .. } => {
                if match_len == match_node.value.len() && match_len == in_chars.len() {
                    //perfect match
                    return Err(anyhow::anyhow!("Leaf with same value already exists"));
                }
                //continue on to case 3
            }
        };
        //case 3
        let inter_value = match_node.value[..match_len].to_vec().into_boxed_slice();
        let par_idx = match_node.parent;
        let inter_node = TrieNode {
            value: inter_value,
            parent: par_idx,
            content: TrieNodeContent::Internal {
                children: vec![ndidx],
            },
        };
        let inter_idx = self.nodes.len();
        self.nodes.push(inter_node);
        debug_assert!(self.nodes[par_idx].try_remove_child(ndidx));
        debug_assert!(self.nodes[par_idx].try_add_child(inter_idx));
        self.nodes[ndidx].parent = inter_idx;
        //add the new leaf
        let leaf_idx = self.nodes.len();
        self.nodes.push(TrieNode {
            value: in_chars,
            parent: inter_idx,
            content: TrieNodeContent::Leaf { data },
        });
        debug_assert!(self.nodes[inter_idx].try_add_child(leaf_idx));
        Ok(())
    }

    /// Returns an iterator over all nodes
    ///
    /// It gets the index by a refernce, but it won't be much of a nuisance when using
    pub fn iter<'a>(&'a self, node_idx: &'a usize) -> TrieIter<'a> {
        return TrieIter {
            trie: &self,
            desc_stack: vec![std::slice::from_ref(node_idx).into_iter()],
        };
    }
}

impl Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trie_iter = self.iter(&0);
        let indent = "    ";

        for (idx, depth) in trie_iter {
            let node = &self.nodes[idx];
            write!(f, "{}{}", &indent.repeat(depth), node.value_str())?; //indentation
            if let TrieNodeContent::Leaf { data } = node.content {
                write!(f, "({})", data)?;
            }
            write!(f, "\n")?; //newline
        }
        return std::fmt::Result::Ok(());
    }
}

pub struct TrieIter<'a> {
    trie: &'a Trie,
    desc_stack: Vec<std::slice::Iter<'a, usize>>, //stack of children iterators
}

//iterates over all descendants (not contain root)
//depth of root is 0, depth of direct child is 1
impl Iterator for TrieIter<'_> {
    type Item = (usize, usize); //idx, depth
    fn next(&mut self) -> Option<Self::Item> {
        if self.desc_stack.len() == 0 {
            return None;
        }
        //desc_stack.len() >= 1
        loop {
            if let Some(idx) = self.desc_stack.last_mut().unwrap().next() {
                let node = &self.trie.nodes[*idx];
                match &node.content {
                    TrieNodeContent::Internal { children } => {
                        self.desc_stack.push(children.iter()); //move further into tree
                        return Some((*idx, self.desc_stack.len() - 2)); //then return
                    }
                    TrieNodeContent::Leaf { .. } => return Some((*idx, self.desc_stack.len() - 1)),
                }
            } else {
                self.desc_stack.pop();
                if self.desc_stack.len() == 0 {
                    return None;
                }
            }
        }
    }
}
