use crate::traverser::visitor::NodeVisitor;
use crate::tree::Tree;
use std::fmt::Debug;

pub mod visitor;

pub struct TreeTraverser<'a, E: Debug> {
    tree: &'a Tree<'a>,
    visitors: Vec<Box<dyn NodeVisitor<E>>>,
}

impl<'a, E: Debug> TreeTraverser<'a, E> {
    pub fn new(tree: &'a Tree<'a>, visitors: Vec<Box<dyn NodeVisitor<E>>>) -> Self {
        Self { tree, visitors }
    }

    pub fn traverse(&mut self) -> Result<(), Vec<E>> {
        let mut errors = Vec::new();
        for visitor in &mut self.visitors {
            if let Err(error) = visitor.visit_node(self.tree.source, &self.tree.definitions, None) {
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
