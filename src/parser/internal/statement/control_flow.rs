use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::statement::control_flow::IfStatement;
use crate::tree::statement::control_flow::IfStatementElse;
use crate::tree::statement::control_flow::IfStatementElseBlock;
use crate::tree::statement::control_flow::IfStatementElseIf;

pub fn if_statement(state: &mut State) -> ParseResult<IfStatement> {
    let comments = state.iterator.comments();
    let r#if = utils::skip(state, TokenKind::If)?;
    let condition = expression::create(state)?;

    let statement = block::block_statement(state)?;

    let mut elseifs: Vec<IfStatementElseIf> = vec![];
    let mut current = state.iterator.current();
    while current.kind == TokenKind::ElseIf {
        state.iterator.next();

        elseifs.push(IfStatementElseIf {
            comments: state.iterator.comments(),
            elseif: current.position,
            condition: expression::create(state)?,
            block: block::block_statement(state)?,
        });

        current = state.iterator.current();
    }

    let r#else = if current.kind == TokenKind::Else {
        state.iterator.next();

        Some(IfStatementElse {
            comments: state.iterator.comments(),
            r#else: current.position,
            block: if state.iterator.current().kind == TokenKind::If {
                IfStatementElseBlock::If(Box::new(if_statement(state)?))
            } else {
                IfStatementElseBlock::Block(block::block_statement(state)?)
            },
        })
    } else {
        None
    };

    Ok(IfStatement {
        comments,
        r#if,
        condition,
        block: statement,
        elseifs,
        r#else,
    })
}
