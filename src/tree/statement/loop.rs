use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::literal::LiteralInteger;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ForeachStatement {
    pub comments: CommentGroup,
    pub foreach: Keyword,
    pub iterator: ForeachIteratorStatement,
    pub block: BlockStatement,
    pub r#else: Option<Keyword>,
    pub else_block: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ForeachIteratorStatement {
    Value {
        expression: Expression,
        r#as: Keyword,
        value: Variable,
    },
    ParenthesizedValue {
        left_parenthesis: usize,
        expression: Expression,
        r#as: Keyword,
        value: Variable,
        right_parenthesis: usize,
    },
    KeyAndValue {
        expression: Expression,
        r#as: Keyword,
        key: Variable,
        double_arrow: usize,
        value: Variable,
    },
    ParenthesizedKeyAndValue {
        left_parenthesis: usize,
        expression: Expression,
        r#as: Keyword,
        key: Variable,
        double_arrow: usize,
        value: Variable,
        right_parenthesis: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ForStatement {
    pub comments: CommentGroup,
    pub r#for: Keyword,
    pub iterator: ForIteratorStatement,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ForIteratorStatement {
    Standalone {
        initializations: CommaSeparated<Expression>,
        initializations_semicolon: usize,
        conditions: CommaSeparated<Expression>,
        conditions_semicolon: usize,
        r#loop: CommaSeparated<Expression>,
    },
    Parenthesized {
        left_parenthesis: usize,
        initializations: CommaSeparated<Expression>,
        initializations_semicolon: usize,
        conditions: CommaSeparated<Expression>,
        conditions_semicolon: usize,
        r#loop: CommaSeparated<Expression>,
        right_parenthesis: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DoWhileStatement {
    pub comments: CommentGroup,
    pub r#do: Keyword,
    pub block: BlockStatement,
    pub r#while: Keyword,
    pub conditions: CommaSeparated<Expression>,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhileStatement {
    pub comments: CommentGroup,
    pub r#while: Keyword,
    pub conditions: CommaSeparated<Expression>,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BreakStatement {
    pub comments: CommentGroup,
    pub r#break: Keyword,
    pub level: Option<LiteralInteger>,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ContinueStatement {
    pub comments: CommentGroup,
    pub r#continue: Keyword,
    pub level: Option<LiteralInteger>,
    pub semicolon: usize,
}

impl ForeachIteratorStatement {
    pub fn expression(&self) -> &Expression {
        match &self {
            Self::Value { expression, .. } => expression,
            Self::ParenthesizedValue { expression, .. } => expression,
            Self::KeyAndValue { expression, .. } => expression,
            Self::ParenthesizedKeyAndValue { expression, .. } => expression,
        }
    }

    pub fn key(&self) -> Option<&Variable> {
        match &self {
            Self::Value { .. } => None,
            Self::ParenthesizedValue { .. } => None,
            Self::KeyAndValue { key, .. } => Some(key),
            Self::ParenthesizedKeyAndValue { key, .. } => Some(key),
        }
    }

    pub fn value(&self) -> &Variable {
        match &self {
            Self::Value { value, .. } => value,
            Self::ParenthesizedValue { value, .. } => value,
            Self::KeyAndValue { value, .. } => value,
            Self::ParenthesizedKeyAndValue { value, .. } => value,
        }
    }
}

impl Node for ForeachStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.foreach.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.foreach, &self.iterator, &self.block];

        if let Some(r#else) = &self.r#else {
            children.push(r#else);
        }

        if let Some(else_block) = &self.else_block {
            children.push(else_block);
        }

        children
    }

    fn get_description(&self) -> String {
        "foreach statement".to_string()
    }
}

impl Node for ForeachIteratorStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Value { expression, .. } | Self::KeyAndValue { expression, .. } => {
                expression.initial_position()
            }
            Self::ParenthesizedValue {
                left_parenthesis, ..
            }
            | Self::ParenthesizedKeyAndValue {
                left_parenthesis, ..
            } => *left_parenthesis,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Value { value, .. } | Self::KeyAndValue { value, .. } => value.final_position(),
            Self::ParenthesizedValue {
                right_parenthesis, ..
            }
            | Self::ParenthesizedKeyAndValue {
                right_parenthesis, ..
            } => right_parenthesis + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Value {
                expression,
                r#as,
                value,
                ..
            }
            | Self::ParenthesizedValue {
                expression,
                r#as,
                value,
                ..
            } => {
                vec![expression, r#as, value]
            }
            Self::KeyAndValue {
                expression,
                r#as,
                key,
                value,
                ..
            }
            | Self::ParenthesizedKeyAndValue {
                expression,
                r#as,
                key,
                value,
                ..
            } => {
                vec![expression, r#as, key, value]
            }
        }
    }

    fn get_description(&self) -> String {
        "foreach iterator".to_string()
    }
}

impl Node for ForStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#for.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.r#for, &self.iterator, &self.block]
    }

    fn get_description(&self) -> String {
        "for statement".to_string()
    }
}

impl Node for ForIteratorStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Standalone {
                initializations,
                initializations_semicolon,
                ..
            } => {
                if let Some(expression) = initializations.inner.first() {
                    expression.initial_position()
                } else {
                    *initializations_semicolon
                }
            }
            Self::Parenthesized {
                left_parenthesis, ..
            } => *left_parenthesis,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Standalone {
                conditions_semicolon,
                r#loop,
                ..
            } => {
                if let Some(expression) = r#loop.inner.last() {
                    expression.final_position()
                } else {
                    conditions_semicolon + 1
                }
            }
            Self::Parenthesized {
                right_parenthesis, ..
            } => right_parenthesis + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Standalone {
                initializations,
                conditions,
                r#loop,
                ..
            }
            | Self::Parenthesized {
                initializations,
                conditions,
                r#loop,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![];

                for expression in &initializations.inner {
                    children.push(expression);
                }

                for expression in &conditions.inner {
                    children.push(expression);
                }

                for expression in &r#loop.inner {
                    children.push(expression);
                }

                children
            }
        }
    }

    fn get_description(&self) -> String {
        "for iterator statement".to_string()
    }
}

impl Node for DoWhileStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#do.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#do, &self.block, &self.r#while];

        for condition in &self.conditions.inner {
            children.push(condition);
        }

        children
    }

    fn get_description(&self) -> String {
        "do-while statement".to_string()
    }
}

impl Node for WhileStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#while.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#while, &self.block];

        for condition in &self.conditions.inner {
            children.push(condition);
        }

        children
    }

    fn get_description(&self) -> String {
        "while statement".to_string()
    }
}

impl Node for BreakStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#break.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        if let Some(level) = &self.level {
            vec![&self.r#break, level]
        } else {
            vec![&self.r#break]
        }
    }

    fn get_description(&self) -> String {
        "break statement".to_string()
    }
}

impl Node for ContinueStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#continue.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        if let Some(level) = &self.level {
            vec![&self.r#continue, level]
        } else {
            vec![&self.r#continue]
        }
    }

    fn get_description(&self) -> String {
        "continue statement".to_string()
    }
}
