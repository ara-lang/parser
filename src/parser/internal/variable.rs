use crate::lexer::token::TokenKind;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::variable::Variable;

pub fn parse(state: &mut State) -> ParseResult<Variable> {
    let current = state.iterator.current();
    if let TokenKind::Variable = &current.kind {
        let position = current.position;
        let name = current.value.clone();
        state.iterator.next();

        return Ok(Variable { position, name });
    }

    crate::parser_bail!(state, unexpected_token(vec!["a variable"], current));
}
