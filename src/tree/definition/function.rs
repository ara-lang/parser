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

impl std::fmt::Display for FunctionLikeReturnTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ": {}", self.type_definition)
    }
}

impl std::fmt::Display for FunctionLikeParameterDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_definition)?;

        if self.ellipsis.is_some() {
            write!(f, "...")?;
        }

        write!(f, " {}", self.variable)?;

        if let Some(default) = self.default.as_ref() {
            write!(f, " = {}", default)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for FunctionLikeParameterListDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.parameters)
    }
}

impl std::fmt::Display for FunctionLikeParameterDefaultValueDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.function, self.name)?;

        if let Some(templates) = &self.templates {
            write!(f, "{}", templates)?;
        }

        write!(f, "{}{} {}", self.parameters, self.return_type, self.body)
    }
}

impl std::fmt::Display for MethodParameterDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.modifiers)?;

        write!(f, "{} {}", self.type_definition, self.variable,)?;

        if let Some(default_value) = &self.default {
            write!(f, " = {}", default_value)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for MethodParameterListDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.parameters)
    }
}

impl std::fmt::Display for MethodTypeConstraintDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.type_definition, self.r#is, self.type_definition
        )
    }
}

impl std::fmt::Display for MethodTypeConstraintGroupDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#where, self.constraints)
    }
}

impl std::fmt::Display for MethodBodyDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MethodBodyDefinition::Concrete(block) => write!(f, "{}", block),
            MethodBodyDefinition::Abstract(..) => write!(f, ";"),
        }
    }
}

impl std::fmt::Display for MethodDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.modifiers, self.function, self.name)?;

        if let Some(templates) = &self.templates {
            write!(f, "{}", templates)?;
        }

        write!(f, "{}", self.parameters)?;

        if let Some(return_type) = &self.return_type {
            write!(f, "{}", return_type)?;
        }

        if let Some(constraints) = &self.constraints {
            write!(f, "{}", constraints)?;
        }

        write!(f, " {}", self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::definition::modifier::ModifierDefinition;
    use crate::tree::definition::r#type::SignedIntegerTypeDefinition;
    use crate::tree::definition::template::TemplateDefinition;
    use crate::tree::definition::template::TemplateDefinitionTypeConstraint;
    use crate::tree::definition::template::TemplateDefinitionVariance;
    use crate::tree::identifier::TemplatedIdentifier;

    #[test]
    fn test_function_definition_display() {
        let function_definition = FunctionDefinition {
            function: Keyword::new(ByteString::from("function"), 0),
            name: Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            },
            templates: None,
            parameters: FunctionLikeParameterListDefinition {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                parameters: CommaSeparated {
                    inner: vec![FunctionLikeParameterDefinition {
                        attributes: vec![],
                        comments: CommentGroup { comments: vec![] },
                        type_definition: TypeDefinition::SignedInteger(
                            SignedIntegerTypeDefinition::I32(Keyword::new(
                                ByteString::from("i32"),
                                15,
                            )),
                        ),
                        ellipsis: None,
                        variable: Variable {
                            position: 0,
                            name: ByteString::from("foo"),
                        },
                        default: None,
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
            return_type: FunctionLikeReturnTypeDefinition {
                colon: 0,
                type_definition: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(
                    Keyword::new(ByteString::from("i64"), 15),
                )),
            },
            body: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
            comments: CommentGroup { comments: vec![] },
            attributes: vec![],
        };

        assert_eq!(
            function_definition.to_string(),
            "function Foo(i32 $foo): i64 { /* ... */ }"
        );
    }

    #[test]
    fn test_method_definition_display() {
        let method_definition = MethodDefinition {
            modifiers: ModifierGroupDefinition {
                position: 0,
                modifiers: vec![ModifierDefinition::Public(Keyword::new(
                    ByteString::from("public"),
                    0,
                ))],
            },
            function: Keyword::new(ByteString::from("function"), 0),
            name: Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            },
            templates: Some(TemplateGroupDefinition {
                comments: CommentGroup { comments: vec![] },
                less_than: 0,
                members: CommaSeparated {
                    inner: vec![
                        TemplateDefinition {
                            variance: TemplateDefinitionVariance::Invaraint,
                            name: Identifier {
                                position: 1,
                                value: ByteString::from("T"),
                            },
                            constraint: TemplateDefinitionTypeConstraint::None,
                        },
                        TemplateDefinition {
                            variance: TemplateDefinitionVariance::Invaraint,
                            name: Identifier {
                                position: 1,
                                value: ByteString::from("U"),
                            },
                            constraint: TemplateDefinitionTypeConstraint::None,
                        },
                    ],
                    commas: vec![],
                },
                greater_than: 4,
            }),
            parameters: MethodParameterListDefinition {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                parameters: CommaSeparated {
                    inner: vec![
                        MethodParameterDefinition {
                            attributes: vec![],
                            comments: CommentGroup { comments: vec![] },
                            modifiers: ModifierGroupDefinition {
                                position: 0,
                                modifiers: vec![],
                            },
                            type_definition: TypeDefinition::Identifier(TemplatedIdentifier {
                                name: Identifier {
                                    position: 0,
                                    value: ByteString::from("T"),
                                },
                                templates: None,
                            }),
                            ellipsis: None,
                            variable: Variable {
                                position: 0,
                                name: ByteString::from("bar"),
                            },
                            default: None,
                        },
                        MethodParameterDefinition {
                            attributes: vec![],
                            comments: CommentGroup { comments: vec![] },
                            modifiers: ModifierGroupDefinition {
                                position: 0,
                                modifiers: vec![],
                            },
                            type_definition: TypeDefinition::Identifier(TemplatedIdentifier {
                                name: Identifier {
                                    position: 0,
                                    value: ByteString::from("U"),
                                },
                                templates: None,
                            }),
                            ellipsis: None,
                            variable: Variable {
                                position: 0,
                                name: ByteString::from("baz"),
                            },
                            default: None,
                        },
                    ],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
            return_type: None,
            constraints: None,
            body: MethodBodyDefinition::Concrete(BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            }),
            comments: CommentGroup { comments: vec![] },
            attributes: vec![],
        };

        assert_eq!(
            method_definition.to_string(),
            "public function Foo<T, U>(T $bar, U $baz) { /* ... */ }"
        );
    }
}
