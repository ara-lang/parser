use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VecExpression {
    pub comments: CommentGroup,
    pub vec: usize,
    pub left_bracket: usize,
    pub members: CommaSeparated<VecExpressionItem>,
    pub right_bracket: usize,
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
    pub dict: usize,
    pub left_bracket: usize,
    pub members: CommaSeparated<DictExpressionItem>,
    pub right_bracket: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DictExpressionItem {
    pub key: Expression,
    pub double_arrow: usize,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TupleExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub members: CommaSeparated<Expression>,
    pub right_parenthesis: usize,
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
        self.vec
    }

    fn final_position(&self) -> usize {
        self.right_bracket + 1
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
        self.dict
    }

    fn final_position(&self) -> usize {
        self.right_bracket + 1
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
        self.left_parenthesis
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .inner
            .iter()
            .map(|item| item as &dyn Node)
            .collect()
    }
}
