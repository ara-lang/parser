use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::constant::ConstantDefinition;
use crate::tree::definition::constant::ConstantDefinitionEntry;
use crate::tree::definition::modifier::ConstantModifierDefinitionGroup;

pub fn constant_definition(state: &mut State) -> ParseResult<ConstantDefinition> {
    Ok(ConstantDefinition {
        comments: state.iterator.comments(),
        r#const: utils::skip(state, TokenKind::Const)?,
        entries: utils::at_least_one_comma_separated(
            state,
            &|state| {
                Ok(ConstantDefinitionEntry {
                    name: identifier::constant_identifier(state)?,
                    equals: utils::skip(state, TokenKind::Equals)?,
                    value: {
                        let expression = expression::create(state)?;

                        if !expression.is_constant(false) {
                            if expression.is_constant(true) {
                                crate::parser_report!(
                                    state,
                                    invalid_initialization_in_constant_expression(&expression)
                                );
                            } else {
                                crate::parser_report!(
                                    state,
                                    invalid_constant_expression(&expression)
                                );
                            }
                        }

                        expression
                    },
                })
            },
            TokenKind::SemiColon,
        )?,
        semicolon: utils::skip_semicolon(state)?,
    })
}

pub fn classish_constant_definition(
    state: &mut State,
    modifiers: ConstantModifierDefinitionGroup,
) -> ParseResult<ClassishConstantDefinition> {
    Ok(ClassishConstantDefinition {
        comments: state.iterator.comments(),
        attributes: state.get_attributes(),
        modifiers,
        r#const: utils::skip(state, TokenKind::Const)?,
        entries: utils::at_least_one_comma_separated(
            state,
            &|state| {
                Ok(ConstantDefinitionEntry {
                    name: identifier::constant_identifier(state)?,
                    equals: utils::skip(state, TokenKind::Equals)?,
                    value: {
                        let expression = expression::create(state)?;

                        if !expression.is_constant(false) {
                            if expression.is_constant(true) {
                                crate::parser_report!(
                                    state,
                                    invalid_initialization_in_constant_expression(&expression)
                                );
                            } else {
                                crate::parser_report!(
                                    state,
                                    invalid_constant_expression(&expression)
                                );
                            }
                        }

                        expression
                    },
                })
            },
            TokenKind::SemiColon,
        )?,
        semicolon: utils::skip_semicolon(state)?,
    })
}
