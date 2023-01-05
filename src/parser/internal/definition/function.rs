use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::parameter;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::definition::template;
use crate::parser::internal::identifier;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::function::AbstractConstructorDefinition;
use crate::tree::definition::function::AbstractMethodDefinition;
use crate::tree::definition::function::ConcreteConstructorDefinition;
use crate::tree::definition::function::ConcreteMethodDefinition;
use crate::tree::definition::function::FunctionDefinition;
use crate::tree::definition::function::FunctionLikeReturnTypeDefinition;
use crate::tree::definition::function::MethodTypeConstraintDefinition;
use crate::tree::definition::function::MethodTypeConstraintGroupDefinition;
use crate::tree::definition::modifier::MethodModifierDefinitionGroup;
use crate::tree::identifier::Identifier;

pub enum MethodDefinitionType {
    Abstract,
    Concrete,
    DependingOnModifiers,
}

pub enum MethodDefinitionReference {
    Abstract(AbstractMethodDefinition),
    Concrete(ConcreteMethodDefinition),
    AbstractConstructor(AbstractConstructorDefinition),
    ConcreteConstructor(ConcreteConstructorDefinition),
}

pub fn function_definition(state: &mut State) -> ParseResult<FunctionDefinition> {
    let comments = state.iterator.comments();
    let attributes = state.get_attributes();

    Ok(FunctionDefinition {
        comments,
        attributes,
        function: utils::skip_keyword(state, TokenKind::Function)?,
        name: identifier::identifier_maybe_soft_reserved(state)?,
        templates: if state.iterator.current().kind == TokenKind::LessThan {
            Some(template::template_group_definition(state)?)
        } else {
            None
        },
        parameters: parameter::function_like_parameter_list_definition(state)?,
        return_type: FunctionLikeReturnTypeDefinition {
            colon: utils::skip_colon(state)?,
            type_definition: r#type::type_definition(state)?,
        },
        body: block::block_statement(state)?,
    })
}

pub fn method_definition(
    state: &mut State,
    r#type: MethodDefinitionType,
    modifiers: MethodModifierDefinitionGroup,
    class: Option<&Identifier>,
) -> ParseResult<MethodDefinitionReference> {
    let comments = state.iterator.comments();
    let attributes = state.get_attributes();
    let function = utils::skip_keyword(state, TokenKind::Function)?;

    let name = identifier::identifier_maybe_reserved(state)?;
    let has_body = match r#type {
        MethodDefinitionType::Abstract => false,
        MethodDefinitionType::Concrete => true,
        MethodDefinitionType::DependingOnModifiers => !modifiers.has_abstract(),
    };

    if name.to_string().to_lowercase() == "__construct" {
        return if has_body {
            let parameters = parameter::constructor_parameter_list_definition(state, class)?;

            Ok(MethodDefinitionReference::ConcreteConstructor(
                ConcreteConstructorDefinition {
                    comments,
                    attributes,
                    modifiers,
                    function,
                    name,
                    parameters,
                    body: block::block_statement(state)?,
                },
            ))
        } else {
            let parameters = parameter::function_like_parameter_list_definition(state)?;
            let semicolon = utils::skip_semicolon(state)?;

            Ok(MethodDefinitionReference::AbstractConstructor(
                AbstractConstructorDefinition {
                    comments,
                    attributes,
                    modifiers,
                    function,
                    name,
                    parameters,
                    semicolon,
                },
            ))
        };
    }

    let templates = if state.iterator.current().kind == TokenKind::LessThan {
        Some(template::template_group_definition(state)?)
    } else {
        None
    };

    let parameters = parameter::function_like_parameter_list_definition(state)?;
    let return_type = FunctionLikeReturnTypeDefinition {
        colon: utils::skip_colon(state)?,
        type_definition: r#type::type_definition(state)?,
    };
    let current = state.iterator.current();
    let constraints = if current.kind == TokenKind::Where {
        Some(MethodTypeConstraintGroupDefinition {
            comments: state.iterator.comments(),
            r#where: utils::skip_keyword(state, TokenKind::Where)?,
            constraints: utils::comma_separated(
                state,
                &|state| {
                    Ok(MethodTypeConstraintDefinition {
                        comments: state.iterator.comments(),
                        identifier: identifier::identifier_maybe_reserved(state)?,
                        r#is: utils::skip_keyword(state, TokenKind::Is)?,
                        type_definition: r#type::type_definition(state)?,
                    })
                },
                TokenKind::LeftBrace,
            )?,
        })
    } else {
        None
    };

    if has_body {
        Ok(MethodDefinitionReference::Concrete(
            ConcreteMethodDefinition {
                comments,
                attributes,
                modifiers,
                function,
                name,
                templates,
                parameters,
                return_type,
                constraints,
                body: block::block_statement(state)?,
            },
        ))
    } else {
        Ok(MethodDefinitionReference::Abstract(
            AbstractMethodDefinition {
                comments,
                attributes,
                modifiers,
                function,
                name,
                templates,
                parameters,
                return_type,
                constraints,
                semicolon: utils::skip_semicolon(state)?,
            },
        ))
    }
}
