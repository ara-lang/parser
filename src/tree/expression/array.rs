use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VecExpression {
    pub comments: CommentGroup,
    pub vec: Span,
    pub left_bracket: Span,
    pub members: CommaSeparated<VecExpressionItem>,
    pub right_bracket: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VecExpressionItem {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DictExpression {
    pub comments: CommentGroup,
    pub dict: Span,
    pub left_bracket: Span,
    pub members: CommaSeparated<DictExpressionItem>,
    pub right_bracket: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DictExpressionItem {
    pub key: Expression,
    pub double_arrow: Span,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TupleExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,
    pub members: CommaSeparated<Expression>,
    pub right_parenthesis: Span,
}

impl Node for VecExpressionItem {
    fn initial_position(&self) -> usize {
        self.value.initial_position()
    }

    fn final_position(&self) -> usize {
        self.value.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.value]
    }
}

impl Node for VecExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.vec.position
    }

    fn final_position(&self) -> usize {
        self.right_bracket.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .inner
            .iter()
            .map(|item| item as &dyn Node)
            .collect()
    }
}

impl Node for DictExpressionItem {
    fn initial_position(&self) -> usize {
        self.key.initial_position()
    }

    fn final_position(&self) -> usize {
        self.value.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.key, &self.value]
    }
}

impl Node for DictExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.dict.position
    }

    fn final_position(&self) -> usize {
        self.right_bracket.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .inner
            .iter()
            .map(|item| item as &dyn Node)
            .collect()
    }
}

impl Node for TupleExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.left_parenthesis.position
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .inner
            .iter()
            .map(|item| item as &dyn Node)
            .collect()
    }
}
