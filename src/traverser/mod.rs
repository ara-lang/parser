use std::fmt::Debug;

use crate::traverser::visitor::NodeVisitor;
use crate::tree::Tree;
use crate::tree::TreeMap;

pub mod visitor;

pub struct TreeTraverser<E: Debug> {
    visitors: Vec<Box<dyn NodeVisitor<E>>>,
}

impl<E: Debug> TreeTraverser<E> {
    pub fn new(visitors: Vec<Box<dyn NodeVisitor<E>>>) -> Self {
        Self { visitors }
    }

    pub fn traverse(&mut self, map: &TreeMap) -> Result<(), Vec<E>> {
        let mut errors = Vec::new();

        for tree in &map.trees {
            if let Err(mut error) = self.traverse_tree(tree) {
                errors.append(&mut error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn traverse_tree(&mut self, tree: &Tree) -> Result<(), Vec<E>> {
        let mut errors = Vec::new();

        for visitor in &mut self.visitors {
            if let Err(error) = visitor.visit_node(&tree.source, &tree.definitions, None) {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
