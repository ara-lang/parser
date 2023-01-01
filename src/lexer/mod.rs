use ara_reporting::issue::Issue;
use ara_source::source::Source;

use crate::lexer::byte_string::ByteString;
use crate::lexer::state::State;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

pub mod byte_string;
pub mod issue;
pub mod iterator;
pub mod token;

pub(in crate::lexer) mod internal;
pub(in crate::lexer) mod macros;
pub(in crate::lexer) mod result;
pub(in crate::lexer) mod state;

pub fn lex(source: &Source) -> Result<Vec<Token>, Box<Issue>> {
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
        position: state.bytes.position(),
        value: ByteString::default(),
    });

    Ok(tokens)
}
