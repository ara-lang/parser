use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::generic::GenericGroupExpression;
use crate::tree::utils::CommaSeparated;

pub fn generic_group(state: &mut State) -> ParseResult<GenericGroupExpression> {
    Ok(GenericGroupExpression {
        double_colon_less_than: utils::skip(state, TokenKind::Generic)?,
        types: {
            let mut inner = vec![];
            let mut commas = vec![];

            let mut current = state.iterator.current();
            while current.kind != TokenKind::GreaterThan && current.kind != TokenKind::RightShift {
                inner.push(r#type::type_definition(state)?);

                current = state.iterator.current();
                if current.kind != TokenKind::Comma {
                    break;
                }

                commas.push(current.position);

                state.iterator.next();
                current = state.iterator.current();
            }

            CommaSeparated { inner, commas }
        },
        greater_than: {
            let current = state.iterator.current();

            if let Some(token) = state.ignored_shift_at {
                utils::skip(state, TokenKind::RightShift)?;
                state.ignored_shift_at = None;
                token.position + 1
            } else if current.kind == TokenKind::RightShift {
                state.ignored_shift_at = Some(current);

                current.position
            } else {
                utils::skip(state, TokenKind::GreaterThan)?
            }
        },
    })
}
