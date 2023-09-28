use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::function::MethodDefinition;
use crate::tree::definition::modifier::ModifierGroupDefinition;
use crate::tree::definition::property::PropertyDefinition;
use crate::tree::definition::template::TemplateGroupDefinition;
use crate::tree::identifier::Identifier;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub modifiers: ModifierGroupDefinition,
    pub class: Keyword,
    pub name: Identifier,
    pub templates: Option<TemplateGroupDefinition>,
    pub extends: Option<ClassDefinitionExtends>,
    pub implements: Option<ClassDefinitionImplements>,
    pub body: ClassDefinitionBody,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassDefinitionExtends {
    pub extends: Keyword,
    pub parent: TemplatedIdentifier,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassDefinitionImplements {
    pub implements: Keyword,
    pub interfaces: CommaSeparated<TemplatedIdentifier>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassDefinitionBody {
    pub left_brace: usize,
    pub members: Vec<ClassDefinitionMember>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ClassDefinitionMember {
    Constant(ClassishConstantDefinition),
    Property(PropertyDefinition),
    Method(MethodDefinition),
}

impl Node for ClassDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
        }

        self.modifiers.initial_position()
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.class];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.modifiers);
        children.push(&self.name);

        if let Some(templates) = &self.templates {
            children.push(templates);
        }

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
        "class definition".to_string()
    }
}

impl Node for ClassDefinitionExtends {
    fn initial_position(&self) -> usize {
        self.extends.initial_position()
    }

    fn final_position(&self) -> usize {
        self.parent.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.extends, &self.parent]
    }

    fn get_description(&self) -> String {
        "class extends definition".to_string()
    }
}

impl Node for ClassDefinitionImplements {
    fn initial_position(&self) -> usize {
        self.implements.initial_position()
    }

    fn final_position(&self) -> usize {
        if let Some(last_interface) = self.interfaces.inner.last() {
            let last_interface_position = last_interface.final_position();
            if let Some(last_comma) = self.interfaces.commas.last() {
                let last_comma_position = last_comma + 1;
                if last_comma_position > last_interface_position {
                    return last_comma_position;
                }
            }

            return last_interface_position;
        }

        self.implements.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.implements];

        for interface in &self.interfaces.inner {
            children.push(interface);
        }

        children
    }

    fn get_description(&self) -> String {
        "class implements definition".to_string()
    }
}

impl Node for ClassDefinitionBody {
    fn initial_position(&self) -> usize {
        self.left_brace
    }

    fn final_position(&self) -> usize {
        self.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .iter()
            .map(|member| member as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "class body definition".to_string()
    }
}

impl Node for ClassDefinitionMember {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Constant(constant) => constant.comments(),
            Self::Property(property) => property.comments(),
            Self::Method(method) => method.comments(),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Constant(constant) => constant.initial_position(),
            Self::Property(property) => property.initial_position(),
            Self::Method(method) => method.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Constant(constant) => constant.final_position(),
            Self::Property(property) => property.final_position(),
            Self::Method(method) => method.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Constant(constant) => vec![constant],
            Self::Property(property) => vec![property],
            Self::Method(method) => vec![method],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Constant(constant) => constant.get_description(),
            Self::Property(property) => property.get_description(),
            Self::Method(method) => method.get_description(),
        }
    }
}

impl std::fmt::Display for ClassDefinitionExtends {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.extends, self.parent)
    }
}

impl std::fmt::Display for ClassDefinitionImplements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.implements, self.interfaces)
    }
}

impl std::fmt::Display for ClassDefinitionMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Constant(constant) => write!(f, "{}", constant),
            Self::Property(property) => write!(f, "{}", property),
            Self::Method(method) => write!(f, "{}", method),
        }
    }
}

impl std::fmt::Display for ClassDefinitionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ /* ... */ }}")
    }
}

impl std::fmt::Display for ClassDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.modifiers, self.class, self.name)?;

        if let Some(templates) = &self.templates {
            write!(f, " {}", templates)?;
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
    use crate::tree::definition::modifier::ModifierDefinition;

    #[test]
    fn test_class_definition_display() {
        let class_definition = ClassDefinition {
            comments: CommentGroup { comments: vec![] },
            class: Keyword::new(ByteString::from("class"), 0),
            attributes: vec![],
            modifiers: ModifierGroupDefinition {
                position: 0,
                modifiers: vec![ModifierDefinition::Abstract(Keyword::new(
                    ByteString::from("abstract"),
                    0,
                ))],
            },
            name: Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            },
            templates: None,
            extends: Some(ClassDefinitionExtends {
                extends: Keyword::new(ByteString::from("extends"), 0),
                parent: TemplatedIdentifier {
                    name: Identifier {
                        position: 0,
                        value: ByteString::from("Bar"),
                    },
                    templates: None,
                },
            }),
            implements: None,
            body: ClassDefinitionBody {
                left_brace: 19,
                members: vec![],
                right_brace: 20,
            },
        };

        assert_eq!(
            class_definition.to_string(),
            "abstract class Foo extends Bar { /* ... */ }"
        );
    }
}
