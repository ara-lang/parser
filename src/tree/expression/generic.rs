use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GenericGroupExpression {
    pub double_colon_less_than: Span,
    pub types: CommaSeparated<TypeDefinition>,
    pub greater_than: Span,
}

impl Node for GenericGroupExpression {
    fn initial_position(&self) -> usize {
        self.double_colon_less_than.position
    }

    fn final_position(&self) -> usize {
        self.greater_than.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.types.inner.iter().map(|t| t as &dyn Node).collect()
    }
}
