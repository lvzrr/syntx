///! Tree for AST construction (structures/hash_tree.rs)
///!
///! This file holds the implementation and logic for a data structure optimized fot traversal
///! lookups based in hashmaps for big grammar trees, fast insertion time, and rearragngeability/movement.
use ahash::AHashMap;
use bumpalo::collections::Vec as BVec;
use bumpalo::*;
use std::cell::RefCell;
use std::fmt::Debug;

/// Node struct definition
#[derive(Debug)]
pub struct Node<'bump, T>
where
    T: std::hash::Hash + std::cmp::Eq + Clone,
{
    pub leafs: AHashMap<T, &'bump RefCell<Node<'bump, T>>>,
    pub left: Option<&'bump RefCell<Node<'bump, T>>>,
    pub right: Option<&'bump RefCell<Node<'bump, T>>>,
    pub is_ast_node: bool,
    pub weight: usize,
    pub value: Option<T>,
    pub end: bool,
}

/// Tree definition that holds the stack for backtracking, the cursor (current node), the root,
/// and the arena that allocates the nodes.
#[derive(Debug)]
pub struct Tree<'bump, T>
where
    T: std::hash::Hash + std::cmp::Eq + Clone,
{
    pub arena: &'bump Bump,
    pub cursor: &'bump RefCell<Node<'bump, T>>,
    pub root: &'bump RefCell<Node<'bump, T>>,
    pub stack: BVec<'bump, &'bump RefCell<Node<'bump, T>>>,
}

/// Default movement methods needed to traverse the tree
impl<'bump, T> Tree<'bump, T>
where
    T: std::hash::Hash + std::cmp::Eq + Clone,
{
    /// Moves the cursor to a Node in the tree passed as an argument
    pub fn move_cursor_to(&mut self, dest: &'bump RefCell<Node<'bump, T>>) {
        self.stack.push(self.cursor);
        self.cursor = dest;
    }
    /// Clears the stack and rewinds to root
    pub fn rewind_root(&mut self) {
        self.stack.clear();
        self.cursor = self.root;
    }
    /// Rewinds the cursor n positions using the stack of visited nodes.
    pub fn rewind_stack_n(&mut self, n: usize) {
        if self.stack.len() <= n {
            self.rewind_root();
        } else {
            let new_len = self.stack.len() - n;
            self.stack.truncate(new_len);
            if let Some(last) = self.stack.last() {
                self.cursor = *last;
            }
        }
    }
}

impl<'bump, T> Tree<'bump, T>
where
    T: std::hash::Hash + std::cmp::Eq + Clone,
{
    pub fn new_in(arena: &'bump Bump) -> Self {
        let root = arena.alloc(RefCell::new(Node::<T>::default()));
        Tree {
            arena,
            cursor: root,
            root,
            stack: BVec::new_in(arena),
        }
    }
}

/// For manual grammar tree construction, and explicit rule declarations for grammars
pub trait Structured<'bump, T> {
    fn insert_sequence(&mut self, s: Vec<T>);
    fn insert_parallel(&mut self, s: Vec<T>);
    fn insert_leaf(&mut self, c: T);
    fn is_immediate(&self, c: &T) -> bool;
}

impl<'bump, T> Structured<'bump, T> for Tree<'bump, T>
where
    T: std::hash::Hash + std::cmp::Eq + Clone,
{
    /// Insert chained nodes one linked to the previous, then rewinds the tree to where it was
    /// before the insertion
    fn insert_sequence(&mut self, s: Vec<T>) {
        let rewind = s.len();
        for key in s.into_iter() {
            self.insert_leaf(key.clone());
            let moveto = self.cursor.borrow_mut().leafs.get(&key).unwrap().to_owned();
            self.move_cursor_to(moveto);
        }
        self.rewind_stack_n(rewind);
    }
    /// Inserts nodes in parallel from the cursor node, all the nodes this produces have the
    /// current cursor as their parent.
    fn insert_parallel(&mut self, s: Vec<T>) {
        for key in s.into_iter() {
            self.insert_leaf(key.clone());
        }
    }
    /// Branches out the tree, creating a entrance in a HashMap
    fn insert_leaf(&mut self, c: T) {
        let exists = {
            let cursor_ref = self.cursor.borrow();
            cursor_ref.leafs.get(&c).cloned()
        };

        if let Some(existing_node) = exists {
            let mut node_mut = existing_node.borrow_mut();
            node_mut.weight = node_mut.weight.saturating_add(1);
        } else {
            let new_node = self.arena.alloc(RefCell::new(Node::default()));
            new_node.borrow_mut().value = Some(c.clone());
            self.cursor.borrow_mut().leafs.insert(c, new_node);
            self.cursor.borrow_mut().end = false;
        }
    }
    /// Looks if a node with a key exists immediatly under the current one
    fn is_immediate(&self, c: &T) -> bool {
        self.cursor.borrow_mut().leafs.get(&c).is_some()
    }
}

impl<'bump, T> Default for Node<'bump, T>
where
    T: std::hash::Hash + std::cmp::Eq + Clone,
{
    fn default() -> Self {
        Self {
            leafs: AHashMap::new(),
            value: None,
            left: None,
            right: None,
            is_ast_node: false,
            end: true,
            weight: 1,
        }
    }
}
