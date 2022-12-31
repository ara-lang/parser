use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::expression::literal::LiteralInteger;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ForeachStatement {
    pub comments: CommentGroup,
    pub foreach: Span,
    pub iterator: ForeachStatementIterator,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ForeachStatementIterator {
    Value {
        expression: Expression,
        r#as: Span,
        value: Variable,
    },
    ParenthesizedValue {
        left_parenthesis: Span,
        expression: Expression,
        r#as: Span,
        value: Variable,
        right_parenthesis: Span,
    },
    KeyAndValue {
        expression: Expression,
        r#as: Span,
        key: Variable,
        double_arrow: Span,
        value: Variable,
    },
    ParenthesizedKeyAndValue {
        left_parenthesis: Span,
        expression: Expression,
        r#as: Span,
        key: Variable,
        double_arrow: Span,
        value: Variable,
        right_parenthesis: Span,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ForStatement {
    pub comments: CommentGroup,
    pub r#for: Span,
    pub iterator: ForStatementIterator,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ForStatementIterator {
    Standalone {
        initializations: CommaSeparated<Expression>,
        initializations_semicolon: Span,
        conditions: CommaSeparated<Expression>,
        conditions_semicolon: Span,
        r#loop: CommaSeparated<Expression>,
    },
    Parenthesized {
        left_parenthesis: Span,
        initializations: CommaSeparated<Expression>,
        initializations_semicolon: Span,
        conditions: CommaSeparated<Expression>,
        conditions_semicolon: Span,
        r#loop: CommaSeparated<Expression>,
        right_parenthesis: Span,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DoWhileStatement {
    pub comments: CommentGroup,
    pub r#do: Span,
    pub block: BlockStatement,
    pub r#while: Span,
    pub condition: Expression,
    pub semicolon: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhileStatement {
    pub comments: CommentGroup,
    pub r#while: Span,
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BreakStatement {
    pub comments: CommentGroup,
    pub r#break: Span,
    pub level: Option<LiteralInteger>,
    pub semicolon: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ContinueStatement {
    pub comments: CommentGroup,
    pub r#continue: Span,
    pub level: Option<LiteralInteger>,
    pub semicolon: Span,
}

impl ForeachStatementIterator {
    pub fn expression(&self) -> &Expression {
        match self {
            ForeachStatementIterator::Value { expression, .. } => expression,
            ForeachStatementIterator::ParenthesizedValue { expression, .. } => expression,
            ForeachStatementIterator::KeyAndValue { expression, .. } => expression,
            ForeachStatementIterator::ParenthesizedKeyAndValue { expression, .. } => expression,
        }
    }

    pub fn key(&self) -> Option<&Variable> {
        match self {
            ForeachStatementIterator::Value { .. } => None,
            ForeachStatementIterator::ParenthesizedValue { .. } => None,
            ForeachStatementIterator::KeyAndValue { key, .. } => Some(key),
            ForeachStatementIterator::ParenthesizedKeyAndValue { key, .. } => Some(key),
        }
    }

    pub fn value(&self) -> &Variable {
        match self {
            ForeachStatementIterator::Value { value, .. } => value,
            ForeachStatementIterator::ParenthesizedValue { value, .. } => value,
            ForeachStatementIterator::KeyAndValue { value, .. } => value,
            ForeachStatementIterator::ParenthesizedKeyAndValue { value, .. } => value,
        }
    }
}

impl Node for ForeachStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.foreach.position
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.iterator, &self.block]
    }
}

impl Node for ForeachStatementIterator {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match self {
            ForeachStatementIterator::Value { expression, .. }
            | ForeachStatementIterator::KeyAndValue { expression, .. } => {
                expression.initial_position()
            }
            ForeachStatementIterator::ParenthesizedValue {
                left_parenthesis, ..
            }
            | ForeachStatementIterator::ParenthesizedKeyAndValue {
                left_parenthesis, ..
            } => left_parenthesis.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ForeachStatementIterator::Value { value, .. }
            | ForeachStatementIterator::KeyAndValue { value, .. } => value.final_position(),
            ForeachStatementIterator::ParenthesizedValue {
                right_parenthesis, ..
            }
            | ForeachStatementIterator::ParenthesizedKeyAndValue {
                right_parenthesis, ..
            } => right_parenthesis.position + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            ForeachStatementIterator::Value {
                expression, value, ..
            }
            | ForeachStatementIterator::ParenthesizedValue {
                expression, value, ..
            } => {
                vec![expression, value]
            }
            ForeachStatementIterator::KeyAndValue {
                expression,
                key,
                value,
                ..
            }
            | ForeachStatementIterator::ParenthesizedKeyAndValue {
                expression,
                key,
                value,
                ..
            } => {
                vec![expression, key, value]
            }
        }
    }
}

impl Node for ForStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#for.position
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.iterator, &self.block]
    }
}

impl Node for ForStatementIterator {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match self {
            ForStatementIterator::Standalone {
                initializations,
                initializations_semicolon,
                ..
            } => {
                if let Some(expression) = initializations.inner.first() {
                    expression.initial_position()
                } else {
                    initializations_semicolon.position
                }
            }
            ForStatementIterator::Parenthesized {
                left_parenthesis, ..
            } => left_parenthesis.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ForStatementIterator::Standalone {
                conditions_semicolon,
                r#loop,
                ..
            } => {
                if let Some(expression) = r#loop.inner.last() {
                    expression.final_position()
                } else {
                    conditions_semicolon.position + 1
                }
            }
            ForStatementIterator::Parenthesized {
                right_parenthesis, ..
            } => right_parenthesis.position + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ForStatementIterator::Standalone {
                initializations,
                conditions,
                r#loop,
                ..
            }
            | ForStatementIterator::Parenthesized {
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
}

impl Node for DoWhileStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#do.position
    }

    fn final_position(&self) -> usize {
        self.semicolon.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.block, &self.condition]
    }
}

impl Node for WhileStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#while.position
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.condition, &self.block]
    }
}

impl Node for BreakStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#break.position
    }

    fn final_position(&self) -> usize {
        self.semicolon.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        if let Some(level) = &self.level {
            vec![level]
        } else {
            vec![]
        }
    }
}

impl Node for ContinueStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#continue.position
    }

    fn final_position(&self) -> usize {
        self.semicolon.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        if let Some(level) = &self.level {
            vec![level]
        } else {
            vec![]
        }
    }
}
