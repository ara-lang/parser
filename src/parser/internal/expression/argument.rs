use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::argument::ArgumentExpression;
use crate::tree::expression::argument::ArgumentListExpression;

pub fn argument_list_expression(state: &mut State) -> ParseResult<ArgumentListExpression> {
    Ok(ArgumentListExpression {
        comments: state.iterator.comments(),
        left_parenthesis: utils::skip_left_parenthesis(state)?,
        arguments: utils::comma_separated(state, &argument_expression, TokenKind::RightParen)?,
        right_parenthesis: utils::skip_right_parenthesis(state)?,
    })
}

fn argument_expression(state: &mut State) -> ParseResult<ArgumentExpression> {
    if identifier::is_identifier_maybe_reserved(&state.iterator.current().kind)
        && state.iterator.lookahead(1).kind == TokenKind::Colon
    {
        let name = identifier::identifier_maybe_reserved(state)?;
        let colon = utils::skip(state, TokenKind::Colon)?;
        let ellipsis = if state.iterator.current().kind == TokenKind::Ellipsis {
            Some(utils::skip(state, TokenKind::Ellipsis)?)
        } else {
            None
        };
        let value = expression::create(state)?;

        return Ok(ArgumentExpression::Named {
            comments: state.iterator.comments(),
            name,
            colon,
            ellipsis,
            value,
        });
    }

    let ellipsis = if state.iterator.current().kind == TokenKind::Ellipsis {
        Some(utils::skip(state, TokenKind::Ellipsis)?)
    } else {
        None
    };

    let value = expression::create(state)?;

    Ok(ArgumentExpression::Positional {
        comments: state.iterator.comments(),
        ellipsis,
        value,
    })
}
