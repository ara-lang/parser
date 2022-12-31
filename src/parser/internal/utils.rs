use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::parser::issue;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

pub fn skip_semicolon(state: &mut State) -> ParseResult<Span> {
    let current = state.iterator.current();

    if current.kind == TokenKind::SemiColon {
        state.iterator.next();

        Ok(current.span)
    } else {
        issue::bail!(state, unexpected_token(vec![";"], current));
    }
}

pub fn skip_left_brace(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::LeftBrace)
}

pub fn skip_right_brace(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightBrace)
}

pub fn skip_left_parenthesis(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::LeftParen)
}

pub fn skip_right_parenthesis(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightParen)
}

pub fn skip_double_arrow(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::DoubleArrow)
}

pub fn skip_double_colon(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::DoubleColon)
}

pub fn skip_colon(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::Colon)
}

pub fn skip(state: &mut State, kind: TokenKind) -> ParseResult<Span> {
    let current = state.iterator.current();

    if current.kind == kind {
        let end = current.span;

        state.iterator.next();

        Ok(end)
    } else {
        issue::bail!(state, unexpected_token(vec![kind], current));
    }
}

/// Parse a comma-separated list of items, allowing a trailing comma.
pub fn comma_separated<T: Node>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
    until: TokenKind,
) -> ParseResult<CommaSeparated<T>> {
    let mut inner: Vec<T> = vec![];
    let mut commas: Vec<Span> = vec![];
    let mut current = state.iterator.current();

    while current.kind != until {
        inner.push(func(state)?);

        current = state.iterator.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.span);

        state.iterator.next();

        current = state.iterator.current();
    }

    Ok(CommaSeparated { inner, commas })
}

/// Parse a comma-separated list of items, requiring at least one item, and not allowing trailing commas.
pub fn at_least_one_comma_separated<T: Node>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
    until: TokenKind,
) -> ParseResult<CommaSeparated<T>> {
    let mut inner: Vec<T> = vec![];
    let mut commas: Vec<Span> = vec![];

    loop {
        inner.push(func(state)?);

        let mut current = state.iterator.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.span);

        state.iterator.next();

        current = state.iterator.current();

        if current.kind == until {
            break;
        }
    }

    Ok(CommaSeparated { inner, commas })
}
