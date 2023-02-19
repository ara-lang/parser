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

impl std::fmt::Display for VecExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.vec, self.elements)
    }
}

impl std::fmt::Display for VecElementExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for DictExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.dict, self.elements)
    }
}

impl std::fmt::Display for DictElementExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.key, self.value)
    }
}

impl std::fmt::Display for TupleExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::Literal::String;
    use crate::tree::expression::literal::LiteralInteger;
    use crate::tree::expression::literal::LiteralString;
    use crate::tree::expression::Expression;

    #[test]
    fn test_vec_expression_display() {
        let vec_expression = VecExpression {
            comments: CommentGroup { comments: vec![] },
            vec: Keyword::new(ByteString::from("vec"), 0),
            left_bracket: 0,
            elements: CommaSeparated {
                inner: vec![
                    VecElementExpression {
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("1"),
                        })),
                    },
                    VecElementExpression {
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("2"),
                        })),
                    },
                ],
                commas: vec![],
            },
            right_bracket: 0,
        };

        assert_eq!(vec_expression.to_string(), "vec[1, 2]");
    }

    #[test]
    fn test_dict_expression_display() {
        let dict_expression = DictExpression {
            comments: CommentGroup { comments: vec![] },
            dict: Keyword::new(ByteString::from("dict"), 0),
            left_bracket: 0,
            elements: CommaSeparated {
                inner: vec![
                    DictElementExpression {
                        key: Expression::Literal(String(LiteralString {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("\"a\""),
                        })),
                        double_arrow: 0,
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("1"),
                        })),
                    },
                    DictElementExpression {
                        key: Expression::Literal(String(LiteralString {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("\"b\""),
                        })),
                        double_arrow: 0,
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("2"),
                        })),
                    },
                ],
                commas: vec![],
            },
            right_bracket: 0,
        };

        assert_eq!(dict_expression.to_string(), "dict[\"a\" => 1, \"b\" => 2]");
    }

    #[test]
    fn test_tuple_expression_display() {
        let tuple_expression = TupleExpression {
            comments: CommentGroup { comments: vec![] },
            left_parenthesis: 0,
            elements: CommaSeparated {
                inner: vec![
                    Expression::Literal(Integer(LiteralInteger {
                        comments: CommentGroup { comments: vec![] },
                        position: 0,
                        value: ByteString::from("1"),
                    })),
                    Expression::Literal(Integer(LiteralInteger {
                        comments: CommentGroup { comments: vec![] },
                        position: 0,
                        value: ByteString::from("2"),
                    })),
                    Expression::Literal(Integer(LiteralInteger {
                        comments: CommentGroup { comments: vec![] },
                        position: 0,
                        value: ByteString::from("3"),
                    })),
                ],
                commas: vec![],
            },
            right_parenthesis: 0,
        };

        assert_eq!(tuple_expression.to_string(), "(1, 2, 3)");
    }
}
