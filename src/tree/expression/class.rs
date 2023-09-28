use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::class::ClassDefinitionBody;
use crate::tree::definition::class::ClassDefinitionExtends;
use crate::tree::definition::class::ClassDefinitionImplements;
use crate::tree::expression::argument::ArgumentListExpression;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousClassExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub class: Keyword,
    pub arguments: ArgumentListExpression,
    pub extends: Option<ClassDefinitionExtends>,
    pub implements: Option<ClassDefinitionImplements>,
    pub body: ClassDefinitionBody,
}

impl Node for AnonymousClassExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.class.initial_position()
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.class];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.arguments);

        if let Some(extends) = &self.extends {
            children.push(extends);
        }

        if let Some(implements) = &self.implements {
            children.push(implements);
        }

        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "anonymous class expression".to_string()
    }
}

impl std::fmt::Display for AnonymousClassExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class)?;

        if !self.arguments.arguments.inner.is_empty() {
            write!(f, "{}", self.arguments)?;
        }

        if let Some(extends) = &self.extends {
            write!(f, " {}", extends)?;
        }

        if let Some(implements) = &self.implements {
            write!(f, " {}", implements)?;
        }

        write!(f, " {}", self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::expression::argument::ArgumentExpression;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;
    use crate::tree::expression::Expression;
    use crate::tree::identifier::Identifier;
    use crate::tree::identifier::TemplatedIdentifier;
    use crate::tree::utils::CommaSeparated;

    #[test]
    fn anonymous_class_expression_display() {
        let anonymous_class = AnonymousClassExpression {
            comments: CommentGroup { comments: vec![] },
            attributes: vec![],
            class: Keyword::new(ByteString::from("class"), 0),
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
            extends: Some(ClassDefinitionExtends {
                extends: Keyword::new(ByteString::from("extends"), 0),
                parent: TemplatedIdentifier {
                    name: Identifier {
                        position: 0,
                        value: ByteString::from("Foo"),
                    },
                    templates: None,
                },
            }),
            implements: Some(ClassDefinitionImplements {
                implements: Keyword::new(ByteString::from("implements"), 0),
                interfaces: CommaSeparated {
                    inner: vec![
                        TemplatedIdentifier {
                            name: Identifier {
                                position: 0,
                                value: ByteString::from("Bar"),
                            },
                            templates: None,
                        },
                        TemplatedIdentifier {
                            name: Identifier {
                                position: 0,
                                value: ByteString::from("Baz"),
                            },
                            templates: None,
                        },
                    ],
                    commas: vec![],
                },
            }),
            body: ClassDefinitionBody {
                left_brace: 0,
                members: vec![],
                right_brace: 0,
            },
        };

        assert_eq!(
            anonymous_class.to_string(),
            "class(1) extends Foo implements Bar, Baz { /* ... */ }"
        );
    }
}
