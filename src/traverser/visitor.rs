use std::fmt::Debug;

use crate::tree::Node;

pub trait NodeVisitor<E: Debug> {
    fn visit_node(
        &mut self,
        source: &String,
        node: &dyn Node,
        parent: Option<&dyn Node>,
    ) -> Result<(), E> {
        self.visit(source, node, parent)?;

        for child in node.children() {
            self.visit_node(source, child, Some(node))?;
        }

        Ok(())
    }

    fn visit(
        &mut self,
        source: &String,
        node: &dyn Node,
        parent: Option<&dyn Node>,
    ) -> Result<(), E>;
}
