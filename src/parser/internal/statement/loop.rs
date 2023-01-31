use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::literal::LiteralInteger;
use crate::tree::expression::Expression;
use crate::tree::expression::ParenthesizedExpression;
use crate::tree::statement::r#loop::BreakStatement;
use crate::tree::statement::r#loop::ContinueStatement;
use crate::tree::statement::r#loop::DoWhileStatement;
use crate::tree::statement::r#loop::ForIteratorStatement;
use crate::tree::statement::r#loop::ForStatement;
use crate::tree::statement::r#loop::ForeachIteratorStatement;
use crate::tree::statement::r#loop::ForeachStatement;
use crate::tree::statement::r#loop::WhileStatement;
use crate::tree::utils::CommaSeparated;

pub fn foreach_statement(state: &mut State) -> ParseResult<ForeachStatement> {
    let comments = state.iterator.comments();
    let foreach = utils::skip_keyword(state, TokenKind::Foreach)?;

    let iterator = 'iterator: {
        let expression = if state.iterator.current().kind == TokenKind::LeftParen {
            // this could be either:
            // 1. foreach ($array as $value)
            // 2. foreach ($array as $key => $value) { ... }
            // 3. foreach ($array) as $value { ... }
            // 4. foreach ($array) as $key => $value { ... }
            let left_parenthesis = utils::skip(state, TokenKind::LeftParen)?;
            let expression = expression::create(state)?;
            if state.iterator.current().kind == TokenKind::As {
                // this is either 1 or 2
                let r#as = utils::skip_keyword(state, TokenKind::As)?;
                let mut value = variable::parse(state)?;
                let current = state.iterator.current();

                break 'iterator if current.kind == TokenKind::DoubleArrow {
                    state.iterator.next();
                    let double_arrow = current.position;
                    let mut key = variable::parse(state)?;
                    std::mem::swap(&mut value, &mut key);

                    ForeachIteratorStatement::ParenthesizedKeyAndValue {
                        left_parenthesis,
                        expression,
                        r#as,
                        key,
                        double_arrow,
                        value,
                        right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                    }
                } else {
                    ForeachIteratorStatement::ParenthesizedValue {
                        left_parenthesis,
                        expression,
                        r#as,
                        value,
                        right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                    }
                };
            } else {
                // this is either 3 or 4
                Expression::Parenthesized(ParenthesizedExpression {
                    comments: state.iterator.comments(),
                    left_parenthesis,
                    expression: Box::new(expression),
                    right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                })
            }
        } else {
            expression::create(state)?
        };

        // this could be either:
        // 1. foreach $array as $value { ... }
        // 2. foreach $array as $key => $value { ... }
        let r#as = utils::skip_keyword(state, TokenKind::As)?;

        let mut value = variable::parse(state)?;

        let current = state.iterator.current();
        if current.kind == TokenKind::DoubleArrow {
            state.iterator.next();
            let double_arrow = current.position;
            let mut key = variable::parse(state)?;
            std::mem::swap(&mut value, &mut key);

            ForeachIteratorStatement::KeyAndValue {
                expression,
                r#as,
                key,
                double_arrow,
                value,
            }
        } else {
            ForeachIteratorStatement::Value {
                expression,
                r#as,
                value,
            }
        }
    };

    let block = block::block_statement(state)?;

    if state.iterator.current().kind == TokenKind::Else {
        Ok(ForeachStatement {
            comments,
            foreach,
            iterator,
            block,
            r#else: Some(utils::skip_keyword(state, TokenKind::Else)?),
            else_block: Some(block::block_statement(state)?),
        })
    } else {
        Ok(ForeachStatement {
            comments,
            foreach,
            iterator,
            block,
            r#else: None,
            else_block: None,
        })
    }
}

pub fn for_statement(state: &mut State) -> ParseResult<ForStatement> {
    Ok(ForStatement {
        comments: state.iterator.comments(),
        r#for: utils::skip_keyword(state, TokenKind::For)?,
        iterator: 'iterator: {
            let initializations = if state.iterator.current().kind == TokenKind::LeftParen {
                // this could be either:
                // 1. for ($i = 0; $i < 10; $i++) { ... }
                // 2. for ($i = 0); $i < 10; $i++ { ... }
                // 3. for (;;) { ... }
                let left_parenthesis = utils::skip(state, TokenKind::LeftParen)?;

                let expression_reference;
                let expression_array;

                let (expressions, standalone) = if state.iterator.current().kind
                    == TokenKind::SemiColon
                {
                    ([].as_slice(), false)
                } else {
                    expression_reference = expression::create(state)?;

                    if state.iterator.current().kind != TokenKind::RightParen {
                        expression_array = [expression_reference];

                        (expression_array.as_slice(), false)
                    } else {
                        expression_array = [Expression::Parenthesized(ParenthesizedExpression {
                            comments: state.iterator.comments(),
                            left_parenthesis,
                            expression: Box::new(expression_reference),
                            right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                        })];

                        (expression_array.as_slice(), true)
                    }
                };

                let initializations = if state.iterator.current().kind == TokenKind::Comma {
                    let commas = [utils::skip(state, TokenKind::Comma)?];

                    let mut initializations =
                        utils::comma_separated(state, &expression::create, TokenKind::SemiColon)?;

                    initializations.inner =
                        [expressions, initializations.inner.as_slice()].concat();
                    initializations.commas =
                        [commas.as_slice(), initializations.commas.as_slice()].concat();

                    initializations
                } else {
                    CommaSeparated {
                        inner: expressions.to_vec(),
                        commas: vec![],
                    }
                };

                if !standalone {
                    break 'iterator ForIteratorStatement::Parenthesized {
                        left_parenthesis,
                        initializations,
                        initializations_semicolon: utils::skip_semicolon(state)?,
                        conditions: utils::comma_separated(
                            state,
                            &expression::create,
                            TokenKind::SemiColon,
                        )?,
                        conditions_semicolon: utils::skip_semicolon(state)?,
                        r#loop: utils::comma_separated(
                            state,
                            &expression::create,
                            TokenKind::RightParen,
                        )?,
                        right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                    };
                };

                initializations
            } else {
                utils::comma_separated(state, &expression::create, TokenKind::SemiColon)?
            };

            ForIteratorStatement::Standalone {
                initializations,
                initializations_semicolon: utils::skip_semicolon(state)?,
                conditions: utils::comma_separated(
                    state,
                    &expression::create,
                    TokenKind::SemiColon,
                )?,
                conditions_semicolon: utils::skip_semicolon(state)?,
                r#loop: utils::comma_separated(state, &expression::create, TokenKind::LeftBrace)?,
            }
        },
        block: block::block_statement(state)?,
    })
}

pub fn do_while_statement(state: &mut State) -> ParseResult<DoWhileStatement> {
    Ok(DoWhileStatement {
        comments: state.iterator.comments(),
        r#do: utils::skip_keyword(state, TokenKind::Do)?,
        block: block::block_statement(state)?,
        r#while: utils::skip_keyword(state, TokenKind::While)?,
        conditions: utils::comma_separated(state, &expression::create, TokenKind::SemiColon)?,
        semicolon: utils::skip_semicolon(state)?,
    })
}

pub fn while_statement(state: &mut State) -> ParseResult<WhileStatement> {
    Ok(WhileStatement {
        comments: state.iterator.comments(),
        r#while: utils::skip_keyword(state, TokenKind::While)?,
        conditions: utils::comma_separated(state, &expression::create, TokenKind::LeftBrace)?,
        block: block::block_statement(state)?,
    })
}

pub fn continue_statement(state: &mut State) -> ParseResult<ContinueStatement> {
    Ok(ContinueStatement {
        comments: state.iterator.comments(),
        r#continue: utils::skip_keyword(state, TokenKind::Continue)?,
        level: maybe_loop_level(state),
        semicolon: utils::skip_semicolon(state)?,
    })
}

pub fn break_statement(state: &mut State) -> ParseResult<BreakStatement> {
    Ok(BreakStatement {
        comments: state.iterator.comments(),
        r#break: utils::skip_keyword(state, TokenKind::Break)?,
        level: maybe_loop_level(state),
        semicolon: utils::skip_semicolon(state)?,
    })
}

fn maybe_loop_level(state: &mut State) -> Option<LiteralInteger> {
    if let Token {
        kind: TokenKind::LiteralInteger,
        position,
        value,
    } = state.iterator.current()
    {
        state.iterator.next();

        return Some(LiteralInteger {
            comments: state.iterator.comments(),
            value: value.clone(),
            position: *position,
        });
    }

    None
}
