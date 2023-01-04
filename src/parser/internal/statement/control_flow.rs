use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::statement::control_flow::IfElseBlockStatement;
use crate::tree::statement::control_flow::IfElseIfStatement;
use crate::tree::statement::control_flow::IfElseStatement;
use crate::tree::statement::control_flow::IfStatement;
use crate::tree::statement::control_flow::UsingAssignmentStatement;
use crate::tree::statement::control_flow::UsingIfClauseStatement;
use crate::tree::statement::control_flow::UsingStatement;
use crate::tree::utils::CommaSeparated;

pub fn if_statement(state: &mut State) -> ParseResult<IfStatement> {
    let comments = state.iterator.comments();
    let r#if = utils::skip(state, TokenKind::If)?;
    let condition = expression::create(state)?;

    let statement = block::block_statement(state)?;

    let mut elseifs: Vec<IfElseIfStatement> = vec![];
    let mut current = state.iterator.current();
    while current.kind == TokenKind::ElseIf {
        state.iterator.next();

        elseifs.push(IfElseIfStatement {
            comments: state.iterator.comments(),
            elseif: current.position,
            condition: expression::create(state)?,
            block: block::block_statement(state)?,
        });

        current = state.iterator.current();
    }

    let r#else = if current.kind == TokenKind::Else {
        state.iterator.next();

        Some(IfElseStatement {
            comments: state.iterator.comments(),
            r#else: current.position,
            block: if state.iterator.current().kind == TokenKind::If {
                IfElseBlockStatement::If(Box::new(if_statement(state)?))
            } else {
                IfElseBlockStatement::Block(block::block_statement(state)?)
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

pub fn using_statement(state: &mut State) -> ParseResult<UsingStatement> {
    let comments = state.iterator.comments();
    let r#using = utils::skip(state, TokenKind::Using)?;
    let mut inner = vec![];
    let mut commas = vec![];

    loop {
        inner.push({
            let comments = state.iterator.comments();
            let variable = variable::parse(state)?;
            let equals = utils::skip(state, TokenKind::Equals)?;
            let expression = expression::create(state)?;

            UsingAssignmentStatement {
                comments,
                variable,
                equals,
                expression,
            }
        });

        let mut current = state.iterator.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.position);

        state.iterator.next();

        current = state.iterator.current();

        if current.kind == TokenKind::If || current.kind == TokenKind::LeftParen {
            break;
        }
    }

    let assignments = CommaSeparated { inner, commas };
    let if_clause = if state.iterator.current().kind == TokenKind::If {
        Some(UsingIfClauseStatement {
            comments: state.iterator.comments(),
            r#if: utils::skip(state, TokenKind::If)?,
            condition: expression::create(state)?,
        })
    } else {
        None
    };

    let block = block::block_statement(state)?;

    Ok(UsingStatement {
        comments,
        using,
        assignments,
        if_clause,
        block,
    })
}
