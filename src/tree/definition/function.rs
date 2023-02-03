use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::modifier::ModifierGroupDefinition;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::definition::template::TemplateGroupDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeReturnTypeDefinition {
    pub colon: usize,
    pub type_definition: TypeDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeParameterDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub type_definition: TypeDefinition,
    pub ellipsis: Option<usize>,
    pub variable: Variable,
    pub default: Option<FunctionLikeParameterDefaultValueDefinition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeParameterDefaultValueDefinition {
    pub equals: usize,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeParameterListDefinition {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub parameters: CommaSeparated<FunctionLikeParameterDefinition>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub function: Keyword,
    pub name: Identifier,
    pub templates: Option<TemplateGroupDefinition>,
    pub parameters: FunctionLikeParameterListDefinition,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MethodParameterDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    pub comments: CommentGroup,
    pub modifiers: ModifierGroupDefinition,
    pub type_definition: TypeDefinition,
    pub ellipsis: Option<usize>,
    pub variable: Variable,
    pub default: Option<FunctionLikeParameterDefaultValueDefinition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MethodParameterListDefinition {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub parameters: CommaSeparated<MethodParameterDefinition>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MethodTypeConstraintDefinition {
    pub comments: CommentGroup,
    pub identifier: Identifier,
    pub r#is: Keyword,
    pub type_definition: TypeDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MethodTypeConstraintGroupDefinition {
    pub comments: CommentGroup,
    pub r#where: Keyword,
    pub constraints: CommaSeparated<MethodTypeConstraintDefinition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MethodBodyDefinition {
    Concrete(BlockStatement),
    Abstract(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MethodDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub modifiers: ModifierGroupDefinition,
    pub function: Keyword,
    pub name: Identifier,
    pub templates: Option<TemplateGroupDefinition>,
    pub parameters: MethodParameterListDefinition,
    pub return_type: Option<FunctionLikeReturnTypeDefinition>,
    pub constraints: Option<MethodTypeConstraintGroupDefinition>,
    pub body: MethodBodyDefinition,
}

impl Node for FunctionLikeReturnTypeDefinition {
    fn initial_position(&self) -> usize {
        self.colon
    }

    fn final_position(&self) -> usize {
        self.type_definition.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.type_definition]
    }

    fn get_description(&self) -> String {
        "function like return type definition".to_string()
    }
}

impl Node for FunctionLikeParameterDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
        }

        self.type_definition.initial_position()
    }

    fn final_position(&self) -> usize {
        if let Some(default) = &self.default {
            return default.final_position();
        }

        self.variable.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.type_definition, &self.variable];

        if let Some(default) = &self.default {
            children.push(default);
        }

        children
    }

    fn get_description(&self) -> String {
        "function like parameter definition".to_string()
    }
}

impl Node for FunctionLikeParameterDefaultValueDefinition {
    fn initial_position(&self) -> usize {
        self.equals
    }

    fn final_position(&self) -> usize {
        self.value.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.value]
    }

    fn get_description(&self) -> String {
        "function like parameter default value definition".to_string()
    }
}

impl Node for FunctionLikeParameterListDefinition {
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
        let mut children: Vec<&dyn Node> = vec![];

        for parameter in &self.parameters.inner {
            children.push(parameter);
        }

        children
    }

    fn get_description(&self) -> String {
        "function like parameter list definition".to_string()
    }
}

impl Node for FunctionDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
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

        children.push(&self.function);
        children.push(&self.name);

        if let Some(templates) = &self.templates {
            children.push(templates);
        }

        children.push(&self.parameters);
        children.push(&self.return_type);
        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "function definition".to_string()
    }
}

impl Node for MethodParameterDefinition {
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
        if let Some(default) = &self.default {
            return default.final_position();
        }

        self.variable.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.type_definition, &self.variable];

        if let Some(default) = &self.default {
            children.push(default);
        }

        children
    }

    fn get_description(&self) -> String {
        "method parameter definition".to_string()
    }
}

impl Node for MethodParameterListDefinition {
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
        let mut children: Vec<&dyn Node> = vec![];

        for parameter in &self.parameters.inner {
            children.push(parameter);
        }

        children
    }

    fn get_description(&self) -> String {
        "method parameter list definition".to_string()
    }
}

impl Node for MethodTypeConstraintDefinition {
    fn initial_position(&self) -> usize {
        self.identifier.initial_position()
    }

    fn final_position(&self) -> usize {
        self.type_definition.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.identifier, &self.r#is, &self.type_definition]
    }

    fn get_description(&self) -> String {
        "method type constraint definition".to_string()
    }
}

impl Node for MethodTypeConstraintGroupDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#where.initial_position()
    }

    fn final_position(&self) -> usize {
        if let Some(last_constraint) = self.constraints.inner.last() {
            let last_constraint_position = last_constraint.final_position();
            if let Some(last_comma) = self.constraints.commas.last() {
                let last_comma_position = last_comma + 1;
                if last_comma_position >= last_constraint_position {
                    return last_comma_position;
                }
            }

            last_constraint_position
        } else {
            self.r#where.final_position()
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#where];

        for constraint in &self.constraints.inner {
            children.push(constraint);
        }

        children
    }

    fn get_description(&self) -> String {
        "method type constraint group definition".to_string()
    }
}

impl Node for MethodBodyDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            MethodBodyDefinition::Concrete(block) => block.initial_position(),
            MethodBodyDefinition::Abstract(semicolon) => *semicolon,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            MethodBodyDefinition::Concrete(block) => block.final_position(),
            MethodBodyDefinition::Abstract(semicolon) => semicolon + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            MethodBodyDefinition::Concrete(block) => vec![block],
            MethodBodyDefinition::Abstract(..) => vec![],
        }
    }

    fn get_description(&self) -> String {
        "method body definition".to_string()
    }
}

impl Node for MethodDefinition {
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
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.modifiers);
        children.push(&self.function);
        children.push(&self.name);

        if let Some(templates) = &self.templates {
            children.push(templates);
        }

        children.push(&self.parameters);
        if let Some(return_type) = &self.return_type {
            children.push(return_type);
        }

        if let Some(constraints) = &self.constraints {
            children.push(constraints);
        }

        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "concrete method definition".to_string()
    }
}
