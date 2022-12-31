use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::function::FunctionLikeParameterListDefinition;
use crate::tree::definition::function::FunctionLikeReturnTypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArrowFunctionExpression {
    pub comments: CommentGroup,
    pub r#static: Option<usize>,
    pub r#fn: usize,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub parameters: FunctionLikeParameterListDefinition,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub double_arrow: usize,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#static: Option<usize>,
    pub function: usize,
    pub parameters: FunctionLikeParameterListDefinition,
    pub use_clause: Option<AnonymousFunctionUseClauseExpression>,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionUseClauseExpression {
    pub comments: CommentGroup,
    pub r#use: usize,
    pub left_parenthesis: usize,
    pub variables: CommaSeparated<AnonymousFunctionUseClauseVariableExpression>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionUseClauseVariableExpression {
    pub comments: CommentGroup,
    pub variable: Variable,
}

impl Node for ArrowFunctionExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#fn
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.parameters);
        children.push(&self.return_type);
        children.push(self.body.as_ref());

        children
    }
}

impl Node for AnonymousFunctionExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.function
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.parameters);
        children.push(&self.return_type);
        children.push(&self.body);

        children
    }
}

impl Node for AnonymousFunctionUseClauseExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#use
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for variable in &self.variables.inner {
            children.push(variable);
        }

        children
    }
}

impl Node for AnonymousFunctionUseClauseVariableExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.variable.initial_position()
    }

    fn final_position(&self) -> usize {
        self.variable.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.variable]
    }
}
