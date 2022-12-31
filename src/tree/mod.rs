use std::any::{Any, TypeId};

use ara_source::source::Source;
use ara_source::SourceMap;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::DefinitionTree;

pub mod comment;
pub mod definition;
pub mod expression;
pub mod identifier;
pub mod statement;
pub mod utils;
pub mod variable;

#[derive(Debug)]
pub struct TreeMap<'a> {
    pub map: &'a SourceMap,
    pub trees: Vec<Tree<'a>>,
}

#[derive(Debug)]
pub struct Tree<'a> {
    pub source: &'a Source,
    pub definitions: DefinitionTree,
}

pub trait Node: Any {
    /// The comments associated with the node.
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    /// The position of the first token in the node.
    fn initial_position(&self) -> usize;

    /// The position of the last token in the node, including the last token itself.
    ///
    /// This is not necessarily the same as the last token in the node's children.
    fn final_position(&self) -> usize;

    /// The children of the node.
    ///
    /// This is used for traversing the tree.
    fn children(&self) -> Vec<&dyn Node>;
}

pub fn cast<T: Node + 'static>(node: &dyn Node) -> Option<&T> {
    // Get `TypeId` of the type this function is instantiated with.
    let t = TypeId::of::<T>();

    // Get `TypeId` of the node we want to downcast.
    let concrete = node.type_id();

    // Compare both `TypeId`s on equality.
    if t == concrete {
        // Get the concrete type pointer from the trait object.
        let concrete = node as *const dyn Node as *const T;

        // Convert it to a reference and return it.
        //
        // SAFETY: This is safe because we know for sure that the pointer
        // is valid and references are only handed out for the lifetime
        // of the function.
        let concrete = unsafe { &*concrete };

        Some(concrete)
    } else {
        None
    }
}
