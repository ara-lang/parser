use crate::lexer::token::TokenKind;
use crate::parser::internal::expression::argument;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::attribute::AttributeDefinition;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::expression::argument::ArgumentExpression;

pub fn gather(state: &mut State) -> ParseResult<bool> {
    if state.iterator.current().kind != TokenKind::Attribute {
        return Ok(false);
    }

    let attributes = AttributeGroupDefinition {
        hash_left_bracket: utils::skip(state, TokenKind::Attribute)?,
        members: utils::comma_separated(
            state,
            &|state| {
                Ok(AttributeDefinition {
                    name: identifier::fully_qualified_type_identifier_including_self(state)?,
                    arguments: if state.iterator.current().kind == TokenKind::LeftParen {
                        let arguments = argument::argument_list_expression(state)?;

                        for argument in &arguments.arguments.inner {
                            match argument {
                                ArgumentExpression::Value { value, .. }
                                | ArgumentExpression::Named { value, .. } => {
                                    if !value.is_constant(true) {
                                        crate::parser_report!(
                                            state,
                                            invalid_constant_initialization_expression(value)
                                        );
                                    }
                                }
                                expression => {
                                    crate::parser_report!(
                                        state,
                                        invalid_constant_initialization_expression(expression)
                                    );
                                }
                            }
                        }

                        Some(arguments)
                    } else {
                        None
                    },
                })
            },
            TokenKind::RightBracket,
        )?,
        right_bracket: utils::skip(state, TokenKind::RightBracket)?,
    };

    state.attribute(attributes);

    // recursive, looking for multiple attribute brackets after each other.
    gather(state).map(|_| true)
}
