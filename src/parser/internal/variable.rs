use crate::lexer::token::TokenKind;
use crate::parser::issue;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::variable::Variable;

pub fn parse(state: &mut State) -> ParseResult<Variable> {
    let current = state.iterator.current();
    if let TokenKind::Variable = &current.kind {
        let span = current.span;
        let name = current.value.clone();
        state.iterator.next();

        return Ok(Variable { span, name });
    }

    issue::bail!(state, unexpected_token(vec!["a variable"], current));
}
