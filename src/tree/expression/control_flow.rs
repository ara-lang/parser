use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MatchExpression {
    pub comments: CommentGroup,
    pub r#match: usize,
    pub expression: Box<Expression>,
    pub body: MatchBodyExpression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MatchBodyExpression {
    pub left_brace: usize,
    pub arms: CommaSeparated<MatchArmExpression>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MatchArmExpression {
    pub condition: MatchArmConditionExpression,
    pub arrow: usize,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchArmConditionExpression {
    Expressions(CommaSeparated<Expression>),
    Default(usize),
}

impl Node for MatchExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#match
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![self.expression.as_ref(), &self.body]
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
}

impl Node for MatchArmConditionExpression {
    fn initial_position(&self) -> usize {
        match self {
            Self::Expressions(expressions) => expressions.inner.first().unwrap().initial_position(),
            Self::Default(default) => *default,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            Self::Expressions(expressions) => expressions.inner.last().unwrap().final_position(),
            Self::Default(default) => default + 7,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Self::Expressions(expressions) => expressions
                .inner
                .iter()
                .map(|expression| expression as &dyn Node)
                .collect(),
            Self::Default(_) => vec![],
        }
    }
}
