use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ArgumentExpression {
    Value {
        comments: CommentGroup,
        value: Expression,
    },
    Spread {
        comments: CommentGroup,
        ellipsis: usize,
        value: Expression,
    },
    ReverseSpread {
        comments: CommentGroup,
        value: Expression,
        ellipsis: usize,
    },
    Named {
        comments: CommentGroup,
        name: Identifier,
        colon: usize,
        value: Expression,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArgumentListExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub arguments: CommaSeparated<ArgumentExpression>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArgumentPlaceholderExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub ellipsis: usize,
    pub right_parenthesis: usize,
}

impl Node for ArgumentExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Value { comments, .. }
            | Self::Spread { comments, .. }
            | Self::ReverseSpread { comments, .. }
            | Self::Named { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Value { value, .. } | Self::ReverseSpread { value, .. } => {
                value.initial_position()
            }
            Self::Spread { ellipsis, .. } => *ellipsis,
            Self::Named { name, .. } => name.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Value { value, .. } | Self::Spread { value, .. } => value.final_position(),
            Self::ReverseSpread { ellipsis, .. } => *ellipsis,
            Self::Named { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Value { value, .. }
            | Self::Spread { value, .. }
            | Self::ReverseSpread { value, .. } => vec![value],
            Self::Named { name, value, .. } => vec![name, value],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Value { .. } => "value argument expression".to_string(),
            Self::Spread { .. } => "spread argument expression".to_string(),
            Self::ReverseSpread { .. } => "reverse spread argument expression".to_string(),
            Self::Named { .. } => "named argument expression".to_string(),
        }
    }
}

impl Node for ArgumentListExpression {
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
        self.arguments
            .inner
            .iter()
            .map(|a| a as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "argument list expression".to_string()
    }
}

impl Node for ArgumentPlaceholderExpression {
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
        vec![]
    }

    fn get_description(&self) -> String {
        "argument placeholder expression".to_string()
    }
}

impl std::fmt::Display for ArgumentExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentExpression::Value { value, .. } => write!(f, "{}", value),
            ArgumentExpression::Spread { value, .. } => write!(f, "...{}", value),
            ArgumentExpression::ReverseSpread { value, .. } => write!(f, "{}...", value),
            ArgumentExpression::Named { name, value, .. } => write!(f, "{}: {}", name, value),
        }
    }
}

impl std::fmt::Display for ArgumentListExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.arguments)
    }
}

impl std::fmt::Display for ArgumentPlaceholderExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(...)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;
    use crate::tree::expression::Expression;
    use crate::tree::identifier::Identifier;
    use crate::tree::variable::Variable;

    #[test]
    fn test_argument_expression_display() {
        let argument = ArgumentExpression::Value {
            comments: CommentGroup { comments: vec![] },
            value: Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("a"),
            }),
        };
        assert_eq!(argument.to_string(), "$a");

        let argument = ArgumentExpression::Spread {
            comments: CommentGroup { comments: vec![] },
            ellipsis: 0,
            value: Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("a"),
            }),
        };
        assert_eq!(argument.to_string(), "...$a");

        let argument = ArgumentExpression::ReverseSpread {
            comments: CommentGroup { comments: vec![] },
            value: Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("a"),
            }),
            ellipsis: 0,
        };
        assert_eq!(argument.to_string(), "$a...");

        let argument = ArgumentExpression::Named {
            comments: CommentGroup { comments: vec![] },
            name: Identifier {
                position: 0,
                value: ByteString::from("a"),
            },
            colon: 1,
            value: Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("1"),
            })),
        };
        assert_eq!(argument.to_string(), "a: 1");
    }
}
