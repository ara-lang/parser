use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::modifier;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::expression;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::issue;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::function::ConstructorParameterDefinition;
use crate::tree::definition::function::ConstructorParameterListDefinition;
use crate::tree::definition::function::FunctionLikeParameterDefaultValueDefinition;
use crate::tree::definition::function::FunctionLikeParameterDefinition;
use crate::tree::definition::function::FunctionLikeParameterListDefinition;
use crate::tree::identifier::Identifier;

pub fn function_like_parameter_list_definition(
    state: &mut State,
) -> ParseResult<FunctionLikeParameterListDefinition> {
    let comments = state.iterator.comments();
    let left_parenthesis = utils::skip_left_parenthesis(state)?;
    let parameters = utils::comma_separated(
        state,
        &|state| {
            attribute::gather(state)?;

            let type_definition = r#type::type_definition(state)?;

            let current = state.iterator.current();
            let ellipsis = if current.kind == TokenKind::Ellipsis {
                state.iterator.next();

                Some(current.position)
            } else {
                None
            };

            // 2. Then expect a variable.
            let variable = variable::parse(state)?;

            if type_definition.is_bottom() {
                issue::report!(
                    state,
                    bottom_type_cannot_be_used_for_parameter(&type_definition, &variable)
                );
            }

            let current = state.iterator.current();
            let default = if current.kind == TokenKind::Equals {
                state.iterator.next();

                Some(FunctionLikeParameterDefaultValueDefinition {
                    equals: current.position,
                    value: expression::create(state)?,
                })
            } else {
                None
            };

            Ok(FunctionLikeParameterDefinition {
                comments: state.iterator.comments(),
                variable,
                attributes: state.get_attributes(),
                type_definition,
                ellipsis,
                default,
            })
        },
        TokenKind::RightParen,
    )?;

    let right_parenthesis = utils::skip_right_parenthesis(state)?;

    Ok(FunctionLikeParameterListDefinition {
        comments,
        left_parenthesis,
        parameters,
        right_parenthesis,
    })
}

pub fn constructor_parameter_list_definition(
    state: &mut State,
    class: Option<&Identifier>,
) -> ParseResult<ConstructorParameterListDefinition> {
    let comments = state.iterator.comments();

    let left_parenthesis = utils::skip_left_parenthesis(state)?;
    let parameters = utils::comma_separated::<ConstructorParameterDefinition>(
        state,
        &|state| {
            attribute::gather(state)?;

            let modifiers = modifier::collect(state)?;
            let modifiers =
                modifier::promoted_property_modifier_definition_group(state, modifiers)?;

            let type_definition = r#type::type_definition(state)?;

            let current = state.iterator.current();
            let (ellipsis, variable) = if matches!(current.kind, TokenKind::Ellipsis) {
                state.iterator.next();
                let variable = variable::parse(state)?;

                (Some(current.position), variable)
            } else {
                (None, variable::parse(state)?)
            };

            if type_definition.is_bottom() {
                issue::report!(
                    state,
                    bottom_type_cannot_be_used_for_parameter(&type_definition, &variable)
                );
            }

            let current = state.iterator.current();
            let default = if current.kind == TokenKind::Equals {
                state.iterator.next();

                Some(FunctionLikeParameterDefaultValueDefinition {
                    equals: current.position,
                    value: expression::create(state)?,
                })
            } else {
                None
            };

            let parameter = ConstructorParameterDefinition {
                comments: state.iterator.comments(),
                variable,
                attributes: state.get_attributes(),
                type_definition,
                ellipsis,
                default,
                modifiers,
            };

            if !parameter.modifiers.is_empty() && parameter.ellipsis.is_some() {
                issue::report!(
                    state,
                    promoted_property_cannot_be_variadic(class, &parameter)
                );
            }

            Ok(parameter)
        },
        TokenKind::RightParen,
    )?;

    let right_parenthesis = utils::skip_right_parenthesis(state)?;

    Ok(ConstructorParameterListDefinition {
        comments,
        left_parenthesis,
        parameters,
        right_parenthesis,
    })
}
