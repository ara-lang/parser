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
pub struct MatchExpression {
    pub comments: CommentGroup,
    pub r#match: Keyword,
    pub expression: Option<Box<Expression>>,
    pub body: MatchBodyExpression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MatchBodyExpression {
    pub left_brace: usize,
    pub arms: CommaSeparated<MatchArmExpression>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MatchArmExpression {
    pub condition: MatchArmConditionExpression,
    pub arrow: usize,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchArmConditionExpression {
    Expressions(CommaSeparated<Expression>),
    Default(Keyword),
}

impl Node for MatchExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#match.initial_position()
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#match, &self.body];
        if let Some(expression) = &self.expression {
            children.push(expression.as_ref());
        }
        children
    }

    fn get_description(&self) -> String {
        "match expression".to_string()
    }
}

impl Node for MatchBodyExpression {
    fn initial_position(&self) -> usize {
        self.left_brace
    }

    fn final_position(&self) -> usize {
        self.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.arms.inner.iter().map(|arm| arm as &dyn Node).collect()
    }

    fn get_description(&self) -> String {
        "match body expression".to_string()
    }
}

impl Node for MatchArmExpression {
    fn initial_position(&self) -> usize {
        self.condition.initial_position()
    }

    fn final_position(&self) -> usize {
        self.expression.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.condition, &self.expression]
    }

    fn get_description(&self) -> String {
        "match arm expression".to_string()
    }
}

impl Node for MatchArmConditionExpression {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Expressions(expressions) => expressions.inner.first().unwrap().initial_position(),
            Self::Default(default) => default.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Expressions(expressions) => expressions.inner.last().unwrap().final_position(),
            Self::Default(default) => default.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Expressions(expressions) => expressions
                .inner
                .iter()
                .map(|expression| expression as &dyn Node)
                .collect(),
            Self::Default(default) => vec![default],
        }
    }

    fn get_description(&self) -> String {
        "match arm condition expression".to_string()
    }
}
