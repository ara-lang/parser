use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfStatement {
    pub comments: CommentGroup,
    pub r#if: Keyword,
    pub conditions: CommaSeparated<Expression>,
    pub block: BlockStatement,
    pub elseifs: Vec<IfElseIfStatement>,
    pub r#else: Option<IfElseStatement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfElseIfStatement {
    pub comments: CommentGroup,
    pub elseif: Keyword,
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfElseStatement {
    pub comments: CommentGroup,
    pub r#else: Keyword,
    pub block: IfElseBlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IfElseBlockStatement {
    If(Box<IfStatement>),
    Block(BlockStatement),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingStatement {
    pub comments: CommentGroup,
    pub r#using: Keyword,
    pub assignments: CommaSeparated<UsingAssignmentStatement>,
    pub if_clause: Option<UsingIfClauseStatement>,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingAssignmentStatement {
    pub comments: CommentGroup,
    pub variable: Variable,
    pub equals: usize,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingIfClauseStatement {
    pub comments: CommentGroup,
    pub r#if: Keyword,
    pub condition: Expression,
}

impl Node for IfStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#if.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#if, &self.block];

        for condition in &self.conditions.inner {
            children.push(condition);
        }

        for elseif in &self.elseifs {
            children.push(elseif);
        }

        if let Some(r#else) = &self.r#else {
            children.push(r#else);
        }

        children
    }

    fn get_description(&self) -> String {
        "if statement".to_string()
    }
}

impl Node for IfElseIfStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.elseif.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.elseif, &self.condition, &self.block]
    }

    fn get_description(&self) -> String {
        "elseif statement".to_string()
    }
}

impl Node for IfElseStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#else.initial_position()
    }

    fn final_position(&self) -> usize {
        match &self.block {
            IfElseBlockStatement::If(r#if) => r#if.final_position(),
            IfElseBlockStatement::Block(block) => block.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self.block {
            IfElseBlockStatement::If(r#if) => vec![&self.r#else, r#if.as_ref()],
            IfElseBlockStatement::Block(block) => vec![&self.r#else, block],
        }
    }

    fn get_description(&self) -> String {
        "else statement".to_string()
    }
}

impl Node for UsingStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#using.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#using];
        for assignment in &self.assignments.inner {
            children.push(assignment);
        }

        if let Some(if_clause) = &self.if_clause {
            children.push(if_clause);
        }

        children.push(&self.block);

        children
    }

    fn get_description(&self) -> String {
        "using statement".to_string()
    }
}

impl Node for UsingAssignmentStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.variable.initial_position()
    }

    fn final_position(&self) -> usize {
        self.expression.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.variable, &self.expression]
    }

    fn get_description(&self) -> String {
        "using assignment statement".to_string()
    }
}

impl Node for UsingIfClauseStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#if.initial_position()
    }

    fn final_position(&self) -> usize {
        self.condition.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.r#if, &self.condition]
    }

    fn get_description(&self) -> String {
        "using if clause statement".to_string()
    }
}

impl std::fmt::Display for IfElseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#else, self.block)
    }
}

impl std::fmt::Display for IfElseIfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.elseif, self.condition, self.block)
    }
}

impl std::fmt::Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r#if, self.conditions, self.block)?;

        for elseif in &self.elseifs {
            write!(f, " {}", elseif)?;
        }

        if let Some(r#else) = &self.r#else {
            write!(f, " {}", r#else)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for UsingStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.r#using)?;

        for assignment in &self.assignments.inner {
            write!(f, "{} ", assignment)?;
        }

        if let Some(if_clause) = &self.if_clause {
            write!(f, "{} ", if_clause)?;
        }

        write!(f, "{}", self.block)
    }
}

impl std::fmt::Display for UsingAssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.variable, self.expression)
    }
}

impl std::fmt::Display for UsingIfClauseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#if, self.condition)
    }
}

impl std::fmt::Display for IfElseBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IfElseBlockStatement::If(r#if) => write!(f, "{}", r#if),
            IfElseBlockStatement::Block(block) => write!(f, "{}", block),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::expression::argument::ArgumentExpression;
    use crate::tree::expression::argument::ArgumentListExpression;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;
    use crate::tree::expression::operator::ComparisonOperationExpression;
    use crate::tree::expression::operator::FunctionOperationExpression;
    use crate::tree::identifier::Identifier;

    #[test]
    pub fn test_if_statement_display() {
        let r#if = IfStatement {
            comments: CommentGroup { comments: vec![] },
            r#if: Keyword::new(ByteString::from("if"), 0),
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
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            elseifs: vec![],
            r#else: None,
        };

        assert_eq!(r#if.to_string(), "if $foo < 10 { /* ... */ }");

        let if_elseif = IfStatement {
            comments: CommentGroup { comments: vec![] },
            r#if: Keyword::new(ByteString::from("if"), 0),
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
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            elseifs: vec![IfElseIfStatement {
                comments: CommentGroup { comments: vec![] },
                elseif: Keyword::new(ByteString::from("elseif"), 0),
                condition: Expression::ComparisonOperation(
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
                ),
                block: BlockStatement {
                    comments: CommentGroup { comments: vec![] },
                    left_brace: 0,
                    statements: vec![],
                    right_brace: 0,
                },
            }],
            r#else: None,
        };

        assert_eq!(
            if_elseif.to_string(),
            "if $foo < 10 { /* ... */ } elseif $foo < 10 { /* ... */ }"
        );

        let if_else_if = IfStatement {
            comments: CommentGroup { comments: vec![] },
            r#if: Keyword::new(ByteString::from("if"), 0),
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
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            elseifs: vec![],
            r#else: Some(IfElseStatement {
                comments: CommentGroup { comments: vec![] },
                r#else: Keyword::new(ByteString::from("else"), 0),
                block: IfElseBlockStatement::If(Box::from(IfStatement {
                    comments: CommentGroup { comments: vec![] },
                    r#if: Keyword::new(ByteString::from("if"), 0),
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
                    block: BlockStatement {
                        comments: CommentGroup { comments: vec![] },
                        left_brace: 0,
                        statements: vec![],
                        right_brace: 0,
                    },
                    elseifs: vec![],
                    r#else: Some(IfElseStatement {
                        comments: CommentGroup { comments: vec![] },
                        r#else: Keyword::new(ByteString::from("else"), 0),
                        block: IfElseBlockStatement::Block(BlockStatement {
                            comments: CommentGroup { comments: vec![] },
                            left_brace: 0,
                            statements: vec![],
                            right_brace: 0,
                        }),
                    }),
                })),
            }),
        };

        assert_eq!(
            if_else_if.to_string(),
            "if $foo < 10 { /* ... */ } else if $foo < 10 { /* ... */ } else { /* ... */ }"
        );
    }

    #[test]
    pub fn test_using_statement_display() {
        let using = UsingStatement {
            comments: CommentGroup { comments: vec![] },
            r#using: Keyword::new(ByteString::from("using"), 0),
            assignments: CommaSeparated {
                inner: vec![UsingAssignmentStatement {
                    comments: CommentGroup { comments: vec![] },
                    variable: Variable {
                        position: 0,
                        name: ByteString::from("foo"),
                    },
                    equals: 0,
                    expression: Expression::FunctionOperation(FunctionOperationExpression::Call {
                        comments: CommentGroup { comments: vec![] },
                        function: Box::new(Expression::Identifier(Identifier {
                            position: 0,
                            value: ByteString::from("bar"),
                        })),
                        generics: None,
                        arguments: ArgumentListExpression {
                            comments: CommentGroup { comments: vec![] },
                            left_parenthesis: 0,
                            arguments: CommaSeparated {
                                inner: vec![ArgumentExpression::Value {
                                    comments: CommentGroup { comments: vec![] },
                                    value: Expression::Literal(Integer(LiteralInteger {
                                        comments: CommentGroup { comments: vec![] },
                                        position: 0,
                                        value: ByteString::from("1"),
                                    })),
                                }],
                                commas: vec![],
                            },
                            right_parenthesis: 0,
                        },
                    }),
                }],
                commas: vec![],
            },
            block: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            if_clause: Some(UsingIfClauseStatement {
                comments: CommentGroup { comments: vec![] },
                r#if: Keyword::new(ByteString::from("if"), 0),
                condition: Expression::ComparisonOperation(
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
                ),
            }),
        };

        assert_eq!(
            using.to_string(),
            "using $foo = bar(1) if $foo < 10 { /* ... */ }"
        );
    }
}
