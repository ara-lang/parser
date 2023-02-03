use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::function::FunctionLikeParameterListDefinition;
use crate::tree::definition::function::FunctionLikeReturnTypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArrowFunctionExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#static: Option<Keyword>,
    pub r#fn: Keyword,
    pub parameters: FunctionLikeParameterListDefinition,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub double_arrow: usize,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#static: Option<Keyword>,
    pub function: Keyword,
    pub parameters: FunctionLikeParameterListDefinition,
    pub use_clause: Option<AnonymousFunctionUseClauseExpression>,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionUseClauseExpression {
    pub comments: CommentGroup,
    pub r#use: Keyword,
    pub left_parenthesis: usize,
    pub variables: CommaSeparated<AnonymousFunctionUseClauseVariableExpression>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        if let Some(r#static) = &self.r#static {
            return r#static.initial_position();
        }

        self.r#fn.initial_position()
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(r#static) = &self.r#static {
            children.push(r#static);
        }

        children.push(&self.r#fn);

        children.push(&self.parameters);
        children.push(&self.return_type);
        children.push(self.body.as_ref());

        children
    }

    fn get_description(&self) -> String {
        "arrow function expression".to_string()
    }
}

impl Node for AnonymousFunctionExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        if let Some(r#static) = &self.r#static {
            return r#static.initial_position();
        }

        self.function.initial_position()
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(r#static) = &self.r#static {
            children.push(r#static);
        }

        children.push(&self.function);
        children.push(&self.parameters);
        children.push(&self.return_type);
        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "anonymous function expression".to_string()
    }
}

impl Node for AnonymousFunctionUseClauseExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#use.initial_position()
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#use];

        for variable in &self.variables.inner {
            children.push(variable);
        }

        children
    }

    fn get_description(&self) -> String {
        "anonymous function use clause expression".to_string()
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

    fn get_description(&self) -> String {
        "anonymous function use clause variable expression".to_string()
    }
}
