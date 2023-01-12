use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::modifier::MethodModifierDefinitionGroup;
use crate::tree::definition::modifier::PromotedPropertyModifierDefinitionGroup;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::definition::template::TemplateGroupDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeReturnTypeDefinition {
    pub colon: usize,
    pub type_definition: TypeDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeParameterDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub type_definition: TypeDefinition,
    pub ellipsis: Option<usize>,
    pub variable: Variable,
    pub default: Option<FunctionLikeParameterDefaultValueDefinition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeParameterDefaultValueDefinition {
    pub equals: usize,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FunctionLikeParameterListDefinition {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub parameters: CommaSeparated<FunctionLikeParameterDefinition>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstructorParameterDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    pub comments: CommentGroup,
    #[serde(flatten)]
    pub modifiers: PromotedPropertyModifierDefinitionGroup,
    pub type_definition: TypeDefinition,
    pub ellipsis: Option<usize>,
    pub variable: Variable,
    pub default: Option<FunctionLikeParameterDefaultValueDefinition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstructorParameterListDefinition {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub parameters: CommaSeparated<ConstructorParameterDefinition>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AbstractConstructorDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    #[serde(flatten)]
    pub modifiers: MethodModifierDefinitionGroup,
    pub function: Keyword,
    pub name: Identifier,
    pub parameters: FunctionLikeParameterListDefinition,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConcreteConstructorDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    #[serde(flatten)]
    pub modifiers: MethodModifierDefinitionGroup,
    pub function: Keyword,
    pub name: Identifier,
    pub parameters: ConstructorParameterListDefinition,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MethodTypeConstraintDefinition {
    pub comments: CommentGroup,
    pub identifier: Identifier,
    pub r#is: Keyword,
    pub type_definition: TypeDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MethodTypeConstraintGroupDefinition {
    pub comments: CommentGroup,
    pub r#where: Keyword,
    pub constraints: CommaSeparated<MethodTypeConstraintDefinition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AbstractMethodDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    #[serde(flatten)]
    pub modifiers: MethodModifierDefinitionGroup,
    pub function: Keyword,
    pub name: Identifier,
    pub templates: Option<TemplateGroupDefinition>,
    pub parameters: FunctionLikeParameterListDefinition,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub constraints: Option<MethodTypeConstraintGroupDefinition>,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConcreteMethodDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    #[serde(flatten)]
    pub modifiers: MethodModifierDefinitionGroup,
    pub function: Keyword,
    pub name: Identifier,
    pub templates: Option<TemplateGroupDefinition>,
    pub parameters: FunctionLikeParameterListDefinition,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub constraints: Option<MethodTypeConstraintGroupDefinition>,
    pub body: BlockStatement,
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

impl Node for ConstructorParameterDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
        }

        if let Some(modifier) = self.modifiers.modifiers.first() {
            return modifier.initial_position();
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
        "constructor parameter definition".to_string()
    }
}

impl Node for ConstructorParameterListDefinition {
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
        "constructor parameter list definition".to_string()
    }
}

impl Node for AbstractConstructorDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
        }

        if let Some(modifier) = self.modifiers.modifiers.first() {
            return modifier.initial_position();
        }

        self.function.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        for modifier in &self.modifiers.modifiers {
            children.push(modifier);
        }

        children.push(&self.function);
        children.push(&self.name);
        children.push(&self.parameters);

        children
    }

    fn get_description(&self) -> String {
        "abstract constructor definition".to_string()
    }
}

impl Node for ConcreteConstructorDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
        }

        if let Some(modifier) = self.modifiers.modifiers.first() {
            return modifier.initial_position();
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

        for modifier in &self.modifiers.modifiers {
            children.push(modifier);
        }

        children.push(&self.function);
        children.push(&self.name);
        children.push(&self.parameters);
        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "concrete constructor definition".to_string()
    }
}

impl Node for AbstractMethodDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
        }

        if let Some(modifier) = self.modifiers.modifiers.first() {
            return modifier.initial_position();
        }

        self.function.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        for modifier in &self.modifiers.modifiers {
            children.push(modifier);
        }

        children.push(&self.function);
        children.push(&self.name);

        if let Some(templates) = &self.templates {
            children.push(templates);
        }

        children.push(&self.parameters);
        children.push(&self.return_type);

        if let Some(constraints) = &self.constraints {
            children.push(constraints);
        }

        children
    }

    fn get_description(&self) -> String {
        "abstract method definition".to_string()
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

impl Node for ConcreteMethodDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            return attributes.initial_position();
        }

        if let Some(modifier) = self.modifiers.modifiers.first() {
            return modifier.initial_position();
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

        for modifier in &self.modifiers.modifiers {
            children.push(modifier);
        }

        children.push(&self.function);
        children.push(&self.name);

        if let Some(templates) = &self.templates {
            children.push(templates);
        }

        children.push(&self.parameters);
        children.push(&self.return_type);

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
