use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::statement::expression::ExpressionStatement;
use crate::tree::statement::r#return::ReturnStatement;
use crate::tree::statement::Statement;

pub mod block;
pub mod control_flow;
pub mod r#loop;
pub mod r#try;

fn statement(state: &mut State) -> ParseResult<Statement> {
    let current = state.iterator.current();

    if matches!(current.kind, TokenKind::OpenTag(_)) {
        state.iterator.next();

        crate::parser_report!(state, php_opening_tag_not_supported(current));

        return statement(state);
    }

    if matches!(current.kind, TokenKind::CloseTag) {
        state.iterator.next();

        crate::parser_report!(state, php_closing_tag_not_supported(current));

        return statement(state);
    }

    let statement = match &current.kind {
        TokenKind::Do => Statement::DoWhile(Box::new(r#loop::do_while_statement(state)?)),
        TokenKind::While => Statement::While(Box::new(r#loop::while_statement(state)?)),
        TokenKind::For => Statement::For(Box::new(r#loop::for_statement(state)?)),
        TokenKind::Foreach => Statement::Foreach(Box::new(r#loop::foreach_statement(state)?)),
        TokenKind::Continue => Statement::Continue(Box::new(r#loop::continue_statement(state)?)),
        TokenKind::Break => Statement::Break(Box::new(r#loop::break_statement(state)?)),
        TokenKind::If => Statement::If(Box::new(control_flow::if_statement(state)?)),
        TokenKind::Try => Statement::Try(Box::new(r#try::try_statement(state)?)),
        TokenKind::LeftBrace => Statement::Block(Box::new(block::block_statement(state)?)),
        TokenKind::Using if matches!(state.iterator.lookahead(1).kind, TokenKind::Variable) => {
            Statement::Using(Box::new(control_flow::using_statement(state)?))
        }
        TokenKind::Return => Statement::Return(Box::new(ReturnStatement::Explicit {
            comments: state.iterator.comments(),
            r#return: utils::skip_keyword(state, TokenKind::Return)?,
            expression: if matches!(state.iterator.current().kind, TokenKind::SemiColon) {
                None
            } else {
                expression::create(state).map(Some)?
            },
            semicolon: utils::skip_semicolon(state)?,
        })),
        TokenKind::SemiColon => Statement::Empty(utils::skip_semicolon(state)?),
        _ => {
            let comments = state.iterator.comments();
            let expression = expression::create(state)?;

            if state.iterator.current().kind == TokenKind::SemiColon {
                Statement::Expression(Box::new(ExpressionStatement {
                    comments,
                    expression,
                    semicolon: utils::skip_semicolon(state)?,
                }))
            } else {
                Statement::Return(Box::new(ReturnStatement::Implicit {
                    comments,
                    expression,
                }))
            }
        }
    };

    Ok(statement)
}
