use crate::lexer::token::TokenKind;
use crate::parser::internal::identifier;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::issue;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::statement::r#try::TryStatement;
use crate::tree::statement::r#try::TryStatementCatchBlock;
use crate::tree::statement::r#try::TryStatementCatchType;
use crate::tree::statement::r#try::TryStatementFinallyBlock;

pub fn try_statement(state: &mut State) -> ParseResult<TryStatement> {
    let comments = state.iterator.comments();
    let r#try = state.iterator.current().position;

    state.iterator.next();

    let body = block::block_statement(state)?;

    let mut catches = Vec::new();
    loop {
        let current = state.iterator.current();
        if current.kind != TokenKind::Catch {
            break;
        }

        let catch = current.position;

        state.iterator.next();
        let left_parenthesis = utils::skip_left_parenthesis(state)?;

        let types = try_statement_catch_type(state)?;
        let var = if state.iterator.current().kind == TokenKind::RightParen {
            None
        } else {
            Some(variable::parse(state)?)
        };

        let right_parenthesis = utils::skip_right_parenthesis(state)?;

        let body = block::block_statement(state)?;

        catches.push(TryStatementCatchBlock {
            comments: state.iterator.comments(),
            catch,
            left_parenthesis,
            types,
            variable: var,
            right_parenthesis,
            block: body,
        })
    }

    let current = state.iterator.current();
    let finally = if current.kind == TokenKind::Finally {
        Some(TryStatementFinallyBlock {
            comments: state.iterator.comments(),
            finally: current.position,
            block: {
                state.iterator.next();
                block::block_statement(state)?
            },
        })
    } else {
        None
    };

    let missing_catch_or_finally = catches.is_empty() && finally.is_none();

    let statement = TryStatement {
        comments,
        r#try,
        block: body,
        catches,
        finally,
    };

    if missing_catch_or_finally {
        issue::report!(state, try_statement_must_have_catch_or_finally(&statement))
    }

    Ok(statement)
}

#[inline(always)]
fn try_statement_catch_type(state: &mut State) -> ParseResult<TryStatementCatchType> {
    let id = identifier::fully_qualified_type_identifier(state)?;

    if state.iterator.current().kind == TokenKind::Pipe {
        state.iterator.next();

        let mut types = vec![id];

        while !state.iterator.is_eof() {
            let id = identifier::fully_qualified_type_identifier(state)?;
            types.push(id);

            if state.iterator.current().kind != TokenKind::Pipe {
                break;
            }

            state.iterator.next();
        }

        return Ok(TryStatementCatchType::Union(types));
    }

    Ok(TryStatementCatchType::Identifier(id))
}
