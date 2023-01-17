use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::modifier;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::expression;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::function::ConstructorParameterDefinition;
use crate::tree::definition::function::ConstructorParameterListDefinition;
use crate::tree::definition::function::FunctionLikeParameterDefaultValueDefinition;
use crate::tree::definition::function::FunctionLikeParameterDefinition;
use crate::tree::definition::function::FunctionLikeParameterListDefinition;

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

            let current = state.iterator.current();
            let default = if current.kind == TokenKind::Equals {
                state.iterator.next();

                Some(FunctionLikeParameterDefaultValueDefinition {
                    equals: current.position,
                    value: {
                        let expression = expression::create(state)?;

                        if !expression.is_constant(true) {
                            crate::parser_report!(
                                state,
                                invalid_constant_initialization_expression(&expression)
                            );
                        }

                        expression
                    },
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
) -> ParseResult<ConstructorParameterListDefinition> {
    let comments = state.iterator.comments();

    let left_parenthesis = utils::skip_left_parenthesis(state)?;
    let parameters = utils::comma_separated::<ConstructorParameterDefinition>(
        state,
        &|state| {
            attribute::gather(state)?;

            let modifiers = modifier::collect(state)?;
            let type_definition = r#type::type_definition(state)?;
            let current = state.iterator.current();
            let (ellipsis, variable) = if matches!(current.kind, TokenKind::Ellipsis) {
                state.iterator.next();
                let variable = variable::parse(state)?;

                (Some(current.position), variable)
            } else {
                (None, variable::parse(state)?)
            };

            let current = state.iterator.current();
            let default = if current.kind == TokenKind::Equals {
                state.iterator.next();

                Some(FunctionLikeParameterDefaultValueDefinition {
                    equals: current.position,
                    value: {
                        let expression = expression::create(state)?;

                        if !expression.is_constant(true) {
                            crate::parser_report!(
                                state,
                                invalid_constant_initialization_expression(&expression)
                            );
                        }

                        expression
                    },
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
