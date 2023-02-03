use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VecExpression {
    pub comments: CommentGroup,
    pub vec: Keyword,
    pub left_bracket: usize,
    pub elements: CommaSeparated<VecElementExpression>,
    pub right_bracket: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VecElementExpression {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DictExpression {
    pub comments: CommentGroup,
    pub dict: Keyword,
    pub left_bracket: usize,
    pub elements: CommaSeparated<DictElementExpression>,
    pub right_bracket: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DictElementExpression {
    pub key: Expression,
    pub double_arrow: usize,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TupleExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub elements: CommaSeparated<Expression>,
    pub right_parenthesis: usize,
}

impl Node for VecElementExpression {
    fn initial_position(&self) -> usize {
        self.value.initial_position()
    }

    fn final_position(&self) -> usize {
        self.value.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.value]
    }

    fn get_description(&self) -> String {
        "vec element expression".to_string()
    }
}

impl Node for VecExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.vec.initial_position()
    }

    fn final_position(&self) -> usize {
        self.right_bracket + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.vec];
        for element in &self.elements.inner {
            children.push(element);
        }

        children
    }

    fn get_description(&self) -> String {
        "vec expression".to_string()
    }
}

impl Node for DictElementExpression {
    fn initial_position(&self) -> usize {
        self.key.initial_position()
    }

    fn final_position(&self) -> usize {
        self.value.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.key, &self.value]
    }

    fn get_description(&self) -> String {
        "dict element expression".to_string()
    }
}

impl Node for DictExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.dict.initial_position()
    }

    fn final_position(&self) -> usize {
        self.right_bracket + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.dict];
        for element in &self.elements.inner {
            children.push(element);
        }

        children
    }

    fn get_description(&self) -> String {
        "dict expression".to_string()
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
        self.elements
            .inner
            .iter()
            .map(|element| element as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "tuple expression".to_string()
    }
}
