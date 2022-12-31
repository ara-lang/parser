use crate::lexer::token::TokenKind;
use crate::parser::internal::expression::argument;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::attribute::AttributeDefinition;
use crate::tree::definition::attribute::AttributeGroupDefinition;

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
                        Some(argument::argument_list_expression(state)?)
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
