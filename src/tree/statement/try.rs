use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::identifier::Identifier;
use crate::tree::statement::block::BlockStatement;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TryStatement {
    pub comments: CommentGroup,
    pub r#try: usize,
    pub block: BlockStatement,
    pub catches: Vec<TryCatchBlockStatement>,
    pub finally: Option<TryFinallyBlockStatement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TryCatchBlockStatement {
    pub comments: CommentGroup,
    pub catch: usize,
    pub left_parenthesis: usize,
    pub types: TryCatchTypeStatement,
    pub variable: Option<Variable>,
    pub right_parenthesis: usize,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TryCatchTypeStatement {
    Identifier(Identifier),
    Union(Vec<Identifier>),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TryFinallyBlockStatement {
    pub comments: CommentGroup,
    pub finally: usize,
    pub block: BlockStatement,
}

impl Node for TryStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#try
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.block];

        for catch in &self.catches {
            children.push(catch);
        }

        if let Some(finally) = &self.finally {
            children.push(finally);
        }

        children
    }
}

impl Node for TryCatchBlockStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.catch
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.types];

        if let Some(variable) = &self.variable {
            children.push(variable);
        }

        children.push(&self.block);

        children
    }
}

impl Node for TryFinallyBlockStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.finally
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.block]
    }
}

impl Node for TryCatchTypeStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match self {
            TryCatchTypeStatement::Identifier(identifier) => identifier.initial_position(),
            TryCatchTypeStatement::Union(identifiers) => identifiers[0].initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            TryCatchTypeStatement::Identifier(identifier) => identifier.final_position(),
            TryCatchTypeStatement::Union(identifiers) => {
                identifiers[identifiers.len() - 1].final_position()
            }
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            TryCatchTypeStatement::Identifier(identifier) => vec![identifier],
            TryCatchTypeStatement::Union(identifiers) => identifiers
                .iter()
                .map(|identifier| identifier as &dyn Node)
                .collect(),
        }
    }
}
