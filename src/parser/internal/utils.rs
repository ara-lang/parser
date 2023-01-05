use crate::lexer::token::TokenKind;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;
use crate::tree::token::Keyword;

pub fn skip_semicolon(state: &mut State) -> ParseResult<usize> {
    let current = state.iterator.current();

    if current.kind == TokenKind::SemiColon {
        state.iterator.next();

        Ok(current.position)
    } else {
        crate::parser_bail!(state, unexpected_token(vec![";"], current));
    }
}

pub fn skip_left_brace(state: &mut State) -> ParseResult<usize> {
    skip(state, TokenKind::LeftBrace)
}

pub fn skip_right_brace(state: &mut State) -> ParseResult<usize> {
    skip(state, TokenKind::RightBrace)
}

pub fn skip_left_parenthesis(state: &mut State) -> ParseResult<usize> {
    skip(state, TokenKind::LeftParen)
}

pub fn skip_right_parenthesis(state: &mut State) -> ParseResult<usize> {
    skip(state, TokenKind::RightParen)
}

pub fn skip_double_arrow(state: &mut State) -> ParseResult<usize> {
    skip(state, TokenKind::DoubleArrow)
}

pub fn skip_double_colon(state: &mut State) -> ParseResult<usize> {
    skip(state, TokenKind::DoubleColon)
}

pub fn skip_colon(state: &mut State) -> ParseResult<usize> {
    skip(state, TokenKind::Colon)
}

pub fn skip_keyword(state: &mut State, kind: TokenKind) -> ParseResult<Keyword>
{
    let current = state.iterator.current();

    if current.kind == kind {
        let keyword = Keyword::new(current.value.clone(), current.position);

        state.iterator.next();

        Ok(keyword)
    } else {
        crate::parser_bail!(state, unexpected_token(vec![kind], current));
    }
}

pub fn skip(state: &mut State, kind: TokenKind) -> ParseResult<usize> {
    let current = state.iterator.current();

    if current.kind == kind {
        let end = current.position;

        state.iterator.next();

        Ok(end)
    } else {
        crate::parser_bail!(state, unexpected_token(vec![kind], current));
    }
}

/// Parse a comma-separated list of items, allowing a trailing comma.
pub fn comma_separated<T: Node>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
    until: TokenKind,
) -> ParseResult<CommaSeparated<T>> {
    let mut inner: Vec<T> = vec![];
    let mut commas: Vec<usize> = vec![];
    let mut current = state.iterator.current();

    while current.kind != until {
        inner.push(func(state)?);

        current = state.iterator.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.position);

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
    let mut commas: Vec<usize> = vec![];

    loop {
        inner.push(func(state)?);

        let mut current = state.iterator.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.position);

        state.iterator.next();

        current = state.iterator.current();

        if current.kind == until {
            break;
        }
    }

    Ok(CommaSeparated { inner, commas })
}
