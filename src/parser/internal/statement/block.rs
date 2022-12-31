use crate::lexer::token::OpenTagKind;
use crate::lexer::token::TokenKind;
use crate::parser::internal::statement;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::statement::block::BlockStatement;
use crate::tree::statement::Statement;

pub fn block_statement(state: &mut State) -> ParseResult<BlockStatement> {
    Ok(BlockStatement {
        comments: state.iterator.comments(),
        left_brace: utils::skip_left_brace(state)?,
        statements: multiple_statements_until(state, &TokenKind::RightBrace)?,
        right_brace: utils::skip_right_brace(state)?,
    })
}

pub fn multiple_statements_until(
    state: &mut State,
    until: &TokenKind,
) -> ParseResult<Vec<Statement>> {
    let mut statements = Vec::new();

    let mut current = state.iterator.current();
    while &current.kind != until {
        if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
            state.iterator.next();

            current = state.iterator.current();
            continue;
        }

        statements.push(statement::statement(state)?);
        current = state.iterator.current();
    }

    Ok(statements)
}
