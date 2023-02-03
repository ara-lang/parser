use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::identifier::Identifier;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TryStatement {
    pub comments: CommentGroup,
    pub r#try: Keyword,
    pub block: BlockStatement,
    pub catches: Vec<TryCatchBlockStatement>,
    pub finally: Option<TryFinallyBlockStatement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TryCatchBlockStatement {
    pub comments: CommentGroup,
    pub catch: Keyword,
    pub left_parenthesis: usize,
    pub types: TryCatchTypeStatement,
    pub variable: Option<Variable>,
    pub right_parenthesis: usize,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TryCatchTypeStatement {
    Identifier(Identifier),
    Union(Vec<Identifier>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TryFinallyBlockStatement {
    pub comments: CommentGroup,
    pub finally: Keyword,
    pub block: BlockStatement,
}

impl Node for TryStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#try.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#try, &self.block];

        for catch in &self.catches {
            children.push(catch);
        }

        if let Some(finally) = &self.finally {
            children.push(finally);
        }

        children
    }

    fn get_description(&self) -> String {
        "try statement".to_string()
    }
}

impl Node for TryCatchBlockStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.catch.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.catch, &self.types];

        if let Some(variable) = &self.variable {
            children.push(variable);
        }

        children.push(&self.block);

        children
    }

    fn get_description(&self) -> String {
        "try catch block statement".to_string()
    }
}

impl Node for TryFinallyBlockStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.finally.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.finally, &self.block]
    }

    fn get_description(&self) -> String {
        "try finally block statement".to_string()
    }
}

impl Node for TryCatchTypeStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Identifier(identifier) => identifier.initial_position(),
            Self::Union(identifiers) => identifiers[0].initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Identifier(identifier) => identifier.final_position(),
            Self::Union(identifiers) => identifiers[identifiers.len() - 1].final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Identifier(identifier) => vec![identifier],
            Self::Union(identifiers) => identifiers
                .iter()
                .map(|identifier| identifier as &dyn Node)
                .collect(),
        }
    }

    fn get_description(&self) -> String {
        "try catch type statement".to_string()
    }
}
