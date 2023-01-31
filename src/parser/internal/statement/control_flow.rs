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
    let r#if = utils::skip_keyword(state, TokenKind::If)?;

    let conditions = utils::comma_separated(state, &expression::create, TokenKind::LeftBrace)?;
    let statement = block::block_statement(state)?;

    let mut elseifs: Vec<IfElseIfStatement> = vec![];
    let mut current = state.iterator.current();
    while current.kind == TokenKind::ElseIf {
        elseifs.push(IfElseIfStatement {
            comments: state.iterator.comments(),
            elseif: utils::skip_keyword(state, TokenKind::ElseIf)?,
            condition: expression::create(state)?,
            block: block::block_statement(state)?,
        });

        current = state.iterator.current();
    }

    let r#else = if current.kind == TokenKind::Else {
        Some(IfElseStatement {
            comments: state.iterator.comments(),
            r#else: utils::skip_keyword(state, TokenKind::Else)?,
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
        conditions,
        block: statement,
        elseifs,
        r#else,
    })
}

pub fn using_statement(state: &mut State) -> ParseResult<UsingStatement> {
    let comments = state.iterator.comments();
    let r#using = utils::skip_keyword(state, TokenKind::Using)?;
    let mut inner = vec![];
    let mut commas = vec![];

    let mut current = state.iterator.current();
    loop {
        if current.kind == TokenKind::If || current.kind == TokenKind::LeftBrace {
            break;
        }

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

        current = state.iterator.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.position);

        state.iterator.next();

        current = state.iterator.current();
    }

    let assignments = CommaSeparated { inner, commas };
    let if_clause = if state.iterator.current().kind == TokenKind::If {
        Some(UsingIfClauseStatement {
            comments: state.iterator.comments(),
            r#if: utils::skip_keyword(state, TokenKind::If)?,
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
