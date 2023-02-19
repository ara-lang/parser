use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::modifier::ModifierGroupDefinition;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstantDefinition {
    pub comments: CommentGroup,
    pub r#const: Keyword,
    pub type_definition: TypeDefinition,
    pub name: Identifier,
    pub equals: usize,
    pub value: Expression,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassishConstantDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub modifiers: ModifierGroupDefinition,
    pub r#const: Keyword,
    pub type_definition: TypeDefinition,
    pub name: Identifier,
    pub equals: usize,
    pub value: Expression,
    pub semicolon: usize,
}

impl Node for ConstantDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#const.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![
            &self.r#const,
            &self.type_definition,
            &self.name,
            &self.value,
        ]
    }

    fn get_description(&self) -> String {
        "constant definition".to_string()
    }
}

impl Node for ClassishConstantDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        self.modifiers.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];
        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.modifiers);
        children.push(&self.r#const);
        children.push(&self.type_definition);
        children.push(&self.name);
        children.push(&self.value);

        children
    }

    fn get_description(&self) -> String {
        "classish constant definition".to_string()
    }
}

impl std::fmt::Display for ConstantDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} = {};",
            self.r#const, self.type_definition, self.name, self.value
        )
    }
}

impl std::fmt::Display for ClassishConstantDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} = {};",
            self.modifiers, self.r#const, self.type_definition, self.name, self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::definition::modifier::ModifierDefinition;
    use crate::tree::definition::r#type::SignedIntegerTypeDefinition;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;

    #[test]
    fn test_constant_definition_display() {
        let constant_definition = ConstantDefinition {
            comments: CommentGroup { comments: vec![] },
            r#const: Keyword::new(ByteString::from("const"), 0),
            type_definition: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(
                Keyword::new(ByteString::from("i64"), 15),
            )),
            name: Identifier {
                position: 0,
                value: ByteString::from("FOO"),
            },
            equals: 0,
            value: Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("1"),
            })),
            semicolon: 0,
        };

        assert_eq!(
            constant_definition.to_string(),
            "const i64 FOO = 1;".to_string()
        );
    }

    #[test]
    fn test_classish_constant_definition_display() {
        let classish_constant_definition = ClassishConstantDefinition {
            comments: CommentGroup { comments: vec![] },
            attributes: vec![],
            modifiers: ModifierGroupDefinition {
                position: 0,
                modifiers: vec![ModifierDefinition::Private(Keyword::new(
                    ByteString::from("private"),
                    0,
                ))],
            },
            r#const: Keyword::new(ByteString::from("const"), 0),
            type_definition: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(
                Keyword::new(ByteString::from("i64"), 15),
            )),
            name: Identifier {
                position: 0,
                value: ByteString::from("FOO"),
            },
            equals: 0,
            value: Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("1"),
            })),
            semicolon: 0,
        };

        assert_eq!(
            classish_constant_definition.to_string(),
            "private const i64 FOO = 1;".to_string()
        );
    }
}
