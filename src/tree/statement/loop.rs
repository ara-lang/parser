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

impl std::fmt::Display for ForeachStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r#foreach, self.iterator, self.block)?;

        if let (Some(r#else), Some(else_block)) = (&self.r#else, &self.else_block) {
            write!(f, " {} {}", r#else, else_block)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ForeachIteratorStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Value {
                expression,
                r#as,
                value,
            } => {
                write!(f, "{} {} {}", expression, r#as, value)
            }
            Self::ParenthesizedValue {
                expression,
                r#as,
                value,
                ..
            } => {
                write!(f, "({} {} {})", expression, r#as, value)
            }
            Self::KeyAndValue {
                expression,
                r#as,
                key,
                value,
                ..
            } => {
                write!(f, "{} {} {} => {}", expression, r#as, key, value)
            }
            Self::ParenthesizedKeyAndValue {
                expression,
                r#as,
                key,
                value,
                ..
            } => {
                write!(f, "({} {} {} => {})", expression, r#as, key, value)
            }
        }
    }
}

impl std::fmt::Display for ForIteratorStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Standalone {
                initializations,
                conditions,
                r#loop,
                ..
            } => {
                write!(f, "{}; {}; {}", initializations, conditions, r#loop)
            }
            Self::Parenthesized {
                initializations,
                conditions,
                r#loop,
                ..
            } => {
                write!(f, "({}; {}; {})", initializations, conditions, r#loop)
            }
        }
    }
}

impl std::fmt::Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r#for, self.iterator, self.block)
    }
}

impl std::fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r#while, self.conditions, self.block)
    }
}

impl std::fmt::Display for DoWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {};",
            self.r#do, self.block, self.r#while, self.conditions
        )
    }
}

impl std::fmt::Display for BreakStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#break)?;

        if let Some(level) = &self.level {
            write!(f, " {}", level)?;
        }

        write!(f, ";")
    }
}

impl std::fmt::Display for ContinueStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#continue)?;

        if let Some(level) = &self.level {
            write!(f, " {}", level)?;
        }

        write!(f, ";")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::operator::ArithmeticOperationExpression;
    use crate::tree::expression::operator::ComparisonOperationExpression;

    #[test]
    pub fn test_foreach_statement_display() {
        let foreach = ForeachStatement {
            r#foreach: Keyword::new(ByteString::from("foreach"), 0),
            iterator: ForeachIteratorStatement::Value {
                expression: Expression::Variable(Variable {
                    position: 0,
                    name: ByteString::from("foo"),
                }),
                r#as: Keyword::new(ByteString::from("as"), 0),
                value: Variable {
                    position: 0,
                    name: ByteString::from("bar"),
                },
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            r#else: None,
            else_block: None,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(foreach.to_string(), "foreach $foo as $bar { /* ... */ }");

        let foreach_with_else = ForeachStatement {
            r#foreach: Keyword::new(ByteString::from("foreach"), 0),
            iterator: ForeachIteratorStatement::Value {
                expression: Expression::Variable(Variable {
                    position: 0,
                    name: ByteString::from("foo"),
                }),
                r#as: Keyword::new(ByteString::from("as"), 0),
                value: Variable {
                    position: 0,
                    name: ByteString::from("bar"),
                },
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            r#else: Some(Keyword::new(ByteString::from("else"), 0)),
            else_block: Some(BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            }),
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(
            foreach_with_else.to_string(),
            "foreach $foo as $bar { /* ... */ } else { /* ... */ }"
        );

        let foreach_with_parenthesized_value = ForeachStatement {
            r#foreach: Keyword::new(ByteString::from("foreach"), 0),
            iterator: ForeachIteratorStatement::ParenthesizedValue {
                expression: Expression::Variable(Variable {
                    position: 0,
                    name: ByteString::from("foo"),
                }),
                r#as: Keyword::new(ByteString::from("as"), 0),
                value: Variable {
                    position: 0,
                    name: ByteString::from("bar"),
                },
                left_parenthesis: 0,
                right_parenthesis: 0,
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            r#else: None,
            else_block: None,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(
            foreach_with_parenthesized_value.to_string(),
            "foreach ($foo as $bar) { /* ... */ }"
        );

        let foreach_with_key_and_value = ForeachStatement {
            r#foreach: Keyword::new(ByteString::from("foreach"), 0),
            iterator: ForeachIteratorStatement::KeyAndValue {
                expression: Expression::Variable(Variable {
                    position: 0,
                    name: ByteString::from("foo"),
                }),
                r#as: Keyword::new(ByteString::from("as"), 0),
                key: Variable {
                    position: 0,
                    name: ByteString::from("bar"),
                },
                double_arrow: 0,
                value: Variable {
                    position: 0,
                    name: ByteString::from("baz"),
                },
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            r#else: None,
            else_block: None,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(
            foreach_with_key_and_value.to_string(),
            "foreach $foo as $bar => $baz { /* ... */ }"
        );

        let foreach_with_parenthesized_key_and_value = ForeachStatement {
            r#foreach: Keyword::new(ByteString::from("foreach"), 0),
            iterator: ForeachIteratorStatement::ParenthesizedKeyAndValue {
                expression: Expression::Variable(Variable {
                    position: 0,
                    name: ByteString::from("foo"),
                }),
                r#as: Keyword::new(ByteString::from("as"), 0),
                key: Variable {
                    position: 0,
                    name: ByteString::from("bar"),
                },
                double_arrow: 0,
                value: Variable {
                    position: 0,
                    name: ByteString::from("baz"),
                },
                left_parenthesis: 0,
                right_parenthesis: 0,
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            r#else: None,
            else_block: None,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(
            foreach_with_parenthesized_key_and_value.to_string(),
            "foreach ($foo as $bar => $baz) { /* ... */ }"
        );
    }

    #[test]
    pub fn test_for_statement_display() {
        let standalone_for_statement = ForStatement {
            r#for: Keyword::new(ByteString::from("for"), 0),
            iterator: ForIteratorStatement::Standalone {
                initializations: CommaSeparated {
                    inner: vec![Expression::Variable(Variable {
                        position: 0,
                        name: ByteString::from("foo"),
                    })],
                    commas: vec![],
                },
                initializations_semicolon: 0,
                conditions: CommaSeparated {
                    inner: vec![Expression::ComparisonOperation(
                        ComparisonOperationExpression::LessThan {
                            comments: CommentGroup { comments: vec![] },
                            left: Box::from(Expression::Variable(Variable {
                                position: 0,
                                name: ByteString::from("foo"),
                            })),
                            right: Box::from(Expression::Literal(Integer(LiteralInteger {
                                comments: CommentGroup { comments: vec![] },
                                position: 0,
                                value: ByteString::from("10"),
                            }))),
                            less_than: 0,
                        },
                    )],
                    commas: vec![],
                },
                conditions_semicolon: 0,
                r#loop: CommaSeparated {
                    inner: vec![Expression::ArithmeticOperation(
                        ArithmeticOperationExpression::PostIncrement {
                            left: Box::from(Expression::Variable(Variable {
                                position: 0,
                                name: ByteString::from("foo"),
                            })),
                            increment: 0,
                        },
                    )],
                    commas: vec![],
                },
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(
            standalone_for_statement.to_string(),
            "for $foo; $foo < 10; $foo++ { /* ... */ }"
        );

        let for_statement_with_parentheses = ForStatement {
            r#for: Keyword::new(ByteString::from("for"), 0),
            iterator: ForIteratorStatement::Parenthesized {
                left_parenthesis: 0,
                initializations: CommaSeparated {
                    inner: vec![Expression::Variable(Variable {
                        position: 0,
                        name: ByteString::from("foo"),
                    })],
                    commas: vec![],
                },
                initializations_semicolon: 0,
                conditions: CommaSeparated {
                    inner: vec![Expression::ComparisonOperation(
                        ComparisonOperationExpression::LessThan {
                            comments: CommentGroup { comments: vec![] },
                            left: Box::from(Expression::Variable(Variable {
                                position: 0,
                                name: ByteString::from("foo"),
                            })),
                            right: Box::from(Expression::Literal(Integer(LiteralInteger {
                                comments: CommentGroup { comments: vec![] },
                                position: 0,
                                value: ByteString::from("10"),
                            }))),
                            less_than: 0,
                        },
                    )],
                    commas: vec![],
                },
                conditions_semicolon: 0,
                r#loop: CommaSeparated {
                    inner: vec![Expression::ArithmeticOperation(
                        ArithmeticOperationExpression::PostIncrement {
                            left: Box::from(Expression::Variable(Variable {
                                position: 0,
                                name: ByteString::from("foo"),
                            })),
                            increment: 0,
                        },
                    )],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(
            for_statement_with_parentheses.to_string(),
            "for ($foo; $foo < 10; $foo++) { /* ... */ }"
        );
    }

    #[test]
    pub fn test_while_statement_display() {
        let while_statement = WhileStatement {
            r#while: Keyword::new(ByteString::from("while"), 0),
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            comments: CommentGroup { comments: vec![] },
            conditions: CommaSeparated {
                inner: vec![Expression::ComparisonOperation(
                    ComparisonOperationExpression::LessThan {
                        comments: CommentGroup { comments: vec![] },
                        left: Box::from(Expression::Variable(Variable {
                            position: 0,
                            name: ByteString::from("foo"),
                        })),
                        right: Box::from(Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("10"),
                        }))),
                        less_than: 0,
                    },
                )],
                commas: vec![],
            },
        };

        assert_eq!(while_statement.to_string(), "while $foo < 10 { /* ... */ }");
    }

    #[test]
    pub fn test_do_while_statement_display() {
        let do_while_statement = DoWhileStatement {
            r#do: Keyword::new(ByteString::from("do"), 0),
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            comments: CommentGroup { comments: vec![] },
            r#while: Keyword::new(ByteString::from("while"), 0),
            conditions: CommaSeparated {
                inner: vec![Expression::ComparisonOperation(
                    ComparisonOperationExpression::LessThan {
                        comments: CommentGroup { comments: vec![] },
                        left: Box::from(Expression::Variable(Variable {
                            position: 0,
                            name: ByteString::from("foo"),
                        })),
                        right: Box::from(Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("10"),
                        }))),
                        less_than: 0,
                    },
                )],
                commas: vec![],
            },
            semicolon: 0,
        };

        assert_eq!(
            do_while_statement.to_string(),
            "do { /* ... */ } while $foo < 10;"
        );
    }

    #[test]
    pub fn test_continue_statement_display() {
        let continue_statement = ContinueStatement {
            r#continue: Keyword::new(ByteString::from("continue"), 0),
            level: None,
            semicolon: 0,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(continue_statement.to_string(), "continue;");

        let continue_statement_with_level = ContinueStatement {
            r#continue: Keyword::new(ByteString::from("continue"), 0),
            level: Some(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("2"),
            }),
            semicolon: 0,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(continue_statement_with_level.to_string(), "continue 2;");
    }

    #[test]
    pub fn test_break_statement_display() {
        let break_statement = BreakStatement {
            r#break: Keyword::new(ByteString::from("break"), 0),
            level: None,
            semicolon: 0,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(break_statement.to_string(), "break;");

        let break_statement_with_level = BreakStatement {
            r#break: Keyword::new(ByteString::from("break"), 0),
            level: Some(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("2"),
            }),
            semicolon: 0,
            comments: CommentGroup { comments: vec![] },
        };

        assert_eq!(break_statement_with_level.to_string(), "break 2;");
    }
}
