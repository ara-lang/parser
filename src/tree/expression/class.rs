use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::class::ClassDefinitionExtends;
use crate::tree::definition::class::ClassDefinitionImplements;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::function::ConcreteConstructorDefinition;
use crate::tree::definition::function::ConcreteMethodDefinition;
use crate::tree::definition::property::PropertyDefinition;
use crate::tree::expression::argument::ArgumentListExpression;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousClassExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub class: Keyword,
    pub arguments: ArgumentListExpression,
    pub extends: Option<ClassDefinitionExtends>,
    pub implements: Option<ClassDefinitionImplements>,
    pub body: AnonymousClassExpressionBody,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousClassExpressionBody {
    pub left_brace: usize,
    pub members: Vec<AnonymousClassExpressionMember>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum AnonymousClassExpressionMember {
    Constant(ClassishConstantDefinition),
    Property(PropertyDefinition),
    ConcreteMethod(ConcreteMethodDefinition),
    ConcreteConstructor(ConcreteConstructorDefinition),
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

impl Node for AnonymousClassExpressionMember {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Constant(constant) => constant.initial_position(),
            Self::Property(property) => property.initial_position(),
            Self::ConcreteMethod(method) => method.initial_position(),
            Self::ConcreteConstructor(constructor) => constructor.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Constant(constant) => constant.final_position(),
            Self::Property(property) => property.final_position(),
            Self::ConcreteMethod(method) => method.final_position(),
            Self::ConcreteConstructor(constructor) => constructor.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Constant(constant) => vec![constant],
            Self::Property(property) => vec![property],
            Self::ConcreteMethod(method) => vec![method],
            Self::ConcreteConstructor(constructor) => vec![constructor],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Constant(constant) => constant.get_description(),
            Self::Property(property) => property.get_description(),
            Self::ConcreteMethod(method) => method.get_description(),
            Self::ConcreteConstructor(constructor) => constructor.get_description(),
        }
    }
}

impl Node for AnonymousClassExpressionBody {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

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
        "anonymous class expression body".to_string()
    }
}
