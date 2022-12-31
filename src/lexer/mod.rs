use ara_source::source::Source;

use crate::lexer::byte_string::ByteString;
use crate::lexer::result::SyntaxResult;
use crate::lexer::state::State;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

pub mod byte_string;
pub mod issue;
pub mod iterator;
pub mod result;
pub mod token;

pub(crate) mod internal;
pub(crate) mod state;

pub fn lex(source: &Source) -> SyntaxResult<Vec<Token>> {
    let mut state = State::new(source);
    let mut tokens = Vec::new();

    while !state.bytes.eof() {
        while let Some(true) = state.bytes.current().map(|u: &u8| u.is_ascii_whitespace()) {
            state.bytes.next();
        }

        // If we have consumed whitespace and then reached the end of the file, we should break.
        if state.bytes.eof() {
            break;
        }

        tokens.push(internal::tokenize(&mut state)?);
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        span: state.bytes.span(),
        value: ByteString::default(),
    });

    Ok(tokens)
}
