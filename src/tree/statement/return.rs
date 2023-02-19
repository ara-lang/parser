use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ReturnStatement {
    Explicit {
        comments: CommentGroup,
        r#return: Keyword,
        expression: Option<Expression>,
        semicolon: usize,
    },
    Implicit {
        comments: CommentGroup,
        expression: Expression,
    },
}

impl Node for ReturnStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Explicit { comments, .. } => Some(comments),
            Self::Implicit { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Explicit { r#return, .. } => r#return.initial_position(),
            Self::Implicit { expression, .. } => expression.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Explicit { semicolon, .. } => semicolon + 1,
            Self::Implicit { expression, .. } => expression.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Explicit {
                r#return,
                expression,
                ..
            } => {
                if let Some(expression) = expression {
                    vec![r#return, expression]
                } else {
                    vec![r#return]
                }
            }
            Self::Implicit { expression, .. } => vec![expression],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Explicit { .. } => "explicit return statement".to_string(),
            Self::Implicit { .. } => "implicit return statement".to_string(),
        }
    }
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Explicit {
                r#return,
                expression,
                ..
            } => {
                if let Some(expression) = expression {
                    write!(f, "{} {};", r#return, expression)
                } else {
                    write!(f, "{};", r#return)
                }
            }
            Self::Implicit { expression, .. } => write!(f, "{}", expression),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;

    #[test]
    pub fn test_return_statement_display() {
        let explicit_return_statement = ReturnStatement::Explicit {
            comments: CommentGroup { comments: vec![] },
            r#return: Keyword::new(ByteString::from("return"), 0),
            expression: Some(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("10"),
            }))),
            semicolon: 0,
        };

        assert_eq!(explicit_return_statement.to_string(), "return 10;");

        let implicit_return_statement = ReturnStatement::Implicit {
            comments: CommentGroup { comments: vec![] },
            expression: Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("10"),
            })),
        };

        assert_eq!(implicit_return_statement.to_string(), "10");
    }
}
