use crate::lexer::token::TokenKind;
use crate::parser::internal::identifier;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::statement::r#try::TryCatchBlockStatement;
use crate::tree::statement::r#try::TryCatchTypeStatement;
use crate::tree::statement::r#try::TryFinallyBlockStatement;
use crate::tree::statement::r#try::TryStatement;

pub fn try_statement(state: &mut State) -> ParseResult<TryStatement> {
    let statement = TryStatement {
        comments: state.iterator.comments(),
        r#try: utils::skip_keyword(state, TokenKind::Try)?,
        block: block::block_statement(state)?,
        catches: {
            let mut catches = Vec::new();
            loop {
                let current = state.iterator.current();
                if current.kind != TokenKind::Catch {
                    break;
                }

                catches.push(TryCatchBlockStatement {
                    comments: state.iterator.comments(),
                    catch: utils::skip_keyword(state, TokenKind::Catch)?,
                    left_parenthesis: utils::skip_left_parenthesis(state)?,
                    types: try_statement_catch_type(state)?,
                    variable: if state.iterator.current().kind == TokenKind::RightParen {
                        None
                    } else {
                        Some(variable::parse(state)?)
                    },
                    right_parenthesis: utils::skip_right_parenthesis(state)?,
                    block: block::block_statement(state)?,
                })
            }

            catches
        },
        finally: if state.iterator.current().kind == TokenKind::Finally {
            Some(TryFinallyBlockStatement {
                comments: state.iterator.comments(),
                finally: utils::skip_keyword(state, TokenKind::Finally)?,
                block: block::block_statement(state)?,
            })
        } else {
            None
        },
    };

    if statement.catches.is_empty() && statement.finally.is_none() {
        crate::parser_report!(state, try_statement_must_have_catch_or_finally(&statement))
    }

    Ok(statement)
}

#[inline(always)]
fn try_statement_catch_type(state: &mut State) -> ParseResult<TryCatchTypeStatement> {
    let id = identifier::fully_qualified_type_identifier(state)?;

    if state.iterator.current().kind == TokenKind::Pipe {
        state.iterator.next();

        let mut types = vec![id];
        loop {
            types.push(identifier::fully_qualified_type_identifier(state)?);

            if state.iterator.current().kind != TokenKind::Pipe {
                break;
            }

            state.iterator.next();
        }

        return Ok(TryCatchTypeStatement::Union(types));
    }

    Ok(TryCatchTypeStatement::Identifier(id))
}
