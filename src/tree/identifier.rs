use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Identifier {
    pub span: Span,
    pub value: ByteString, // `Bar`
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TemplatedIdentifier {
    pub name: Identifier,                               // `Bar`
    pub templates: Option<TypeTemplateGroupDefinition>, // `<T>`
}

impl Node for Identifier {
    fn initial_position(&self) -> usize {
        self.span.position
    }

    fn final_position(&self) -> usize {
        self.span.position + self.value.len()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for TemplatedIdentifier {
    fn initial_position(&self) -> usize {
        self.name.initial_position()
    }

    fn final_position(&self) -> usize {
        match &self.templates {
            Some(templates) => templates.final_position(),
            None => self.name.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.name];

        match &self.templates {
            Some(templates) => {
                children.push(templates);

                children
            }
            None => children,
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for TemplatedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(templates) = &self.templates {
            write!(f, "{}", templates)?;
        }

        Ok(())
    }
}
