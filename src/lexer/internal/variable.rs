use crate::lexer::byte_string::ByteString;
use crate::lexer::internal::identifier;
use crate::lexer::state::State;
use crate::lexer::token::TokenKind;

pub fn tokenize(state: &mut State) -> (TokenKind, ByteString) {
    let mut var = state.bytes.read_and_skip(1).to_vec();

    var.extend(identifier::consume(state));

    (TokenKind::Variable, var.into())
}
