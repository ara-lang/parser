use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::modifier;
use crate::parser::internal::definition::parameter;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::definition::template;
use crate::parser::internal::identifier;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::function::FunctionDefinition;
use crate::tree::definition::function::FunctionLikeReturnTypeDefinition;
use crate::tree::definition::function::MethodBodyDefinition;
use crate::tree::definition::function::MethodDefinition;
use crate::tree::definition::function::MethodTypeConstraintDefinition;
use crate::tree::definition::function::MethodTypeConstraintGroupDefinition;
use crate::tree::definition::modifier::ModifierGroupDefinition;

pub fn function_definition(state: &mut State) -> ParseResult<FunctionDefinition> {
    Ok(FunctionDefinition {
        comments: state.iterator.comments(),
        attributes: state.get_attributes(),
        modifiers: modifier::collect_some(state)?,
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
    modifiers: ModifierGroupDefinition,
) -> ParseResult<MethodDefinition> {
    Ok(MethodDefinition {
        comments: state.iterator.comments(),
        attributes: state.get_attributes(),
        modifiers,
        function: utils::skip_keyword(state, TokenKind::Function)?,
        name: identifier::identifier_maybe_reserved(state)?,
        templates: if state.iterator.current().kind == TokenKind::LessThan {
            Some(template::template_group_definition(state)?)
        } else {
            None
        },
        parameters: parameter::method_parameter_list_definition(state)?,
        return_type: if state.iterator.current().kind == TokenKind::Colon {
            Some(FunctionLikeReturnTypeDefinition {
                colon: utils::skip_colon(state)?,
                type_definition: r#type::type_definition(state)?,
            })
        } else {
            None
        },
        constraints: if state.iterator.current().kind == TokenKind::Where {
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
        },
        body: if state.iterator.current().kind == TokenKind::SemiColon {
            MethodBodyDefinition::Abstract(utils::skip_semicolon(state)?)
        } else {
            MethodBodyDefinition::Concrete(block::block_statement(state)?)
        },
    })
}
