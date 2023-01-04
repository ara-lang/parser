use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::expression::argument;
use crate::parser::internal::expression::generic;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::argument::ArgumentPlaceholderExpression;
use crate::tree::expression::operator::ArithmeticOperationExpression;
use crate::tree::expression::operator::ArrayOperationExpression;
use crate::tree::expression::operator::ClassOperationExpression;
use crate::tree::expression::operator::CoalesceOperationExpression;
use crate::tree::expression::operator::FunctionOperationExpression;
use crate::tree::expression::operator::ObjectOperationExpression;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;

pub fn postfix(state: &mut State, left: Expression, kind: &TokenKind) -> ParseResult<Expression> {
    Ok(match kind {
        TokenKind::DoubleQuestion => {
            let double_question = state.iterator.current().position;
            state.iterator.next();

            let comments = state.iterator.comments();

            let rhs = expression::null_coalesce_precedence(state)?;

            Expression::CoalesceOperation(CoalesceOperationExpression::Coalesce {
                comments,
                left: Box::new(left),
                double_question,
                right: Box::new(rhs),
            })
        }
        TokenKind::Generic | TokenKind::LeftParen => {
            let generics = if kind == &TokenKind::Generic {
                Some(generic::generic_group(state)?)
            } else {
                None
            };

            let comments = state.iterator.comments();
            // `(...)` closure creation
            if state.iterator.lookahead(1).kind == TokenKind::Ellipsis
                && state.iterator.lookahead(2).kind == TokenKind::RightParen
            {
                Expression::FunctionOperation(FunctionOperationExpression::ClosureCreation {
                    comments,
                    function: Box::new(left),
                    generics,
                    placeholder: ArgumentPlaceholderExpression {
                        left_parenthesis: utils::skip(state, TokenKind::LeftParen)?,
                        ellipsis: utils::skip(state, TokenKind::Ellipsis)?,
                        right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                        comments: state.iterator.comments(),
                    },
                })
            } else {
                let arguments = argument::argument_list_expression(state)?;

                Expression::FunctionOperation(FunctionOperationExpression::Call {
                    comments,
                    function: Box::new(left),
                    generics,
                    arguments,
                })
            }
        }
        TokenKind::LeftBracket => {
            let comments = state.iterator.comments();
            let left_bracket = utils::skip(state, TokenKind::LeftBracket)?;

            let current = state.iterator.current();
            if current.kind == TokenKind::RightBracket {
                state.iterator.next();
                let right_bracket = current.position;

                Expression::ArrayOperation(ArrayOperationExpression::Push {
                    comments,
                    array: Box::new(left),
                    left_bracket,
                    right_bracket,
                })
            } else {
                let index = expression::create(state)?;

                Expression::ArrayOperation(ArrayOperationExpression::Access {
                    comments,
                    array: Box::new(left),
                    left_bracket,
                    index: Box::new(index),
                    right_bracket: utils::skip(state, TokenKind::RightBracket)?,
                })
            }
        }
        TokenKind::DoubleColon => {
            let position = utils::skip_double_colon(state)?;

            let current = state.iterator.current();

            match current.kind {
                TokenKind::Variable => {
                    Expression::ClassOperation(ClassOperationExpression::StaticPropertyFetch {
                        comments: state.iterator.comments(),
                        class: Box::new(left),
                        double_colon: position,
                        property: variable::parse(state)?,
                    })
                }
                _ if identifier::is_identifier_maybe_reserved(&state.iterator.current().kind) => {
                    let identifier = identifier::identifier_maybe_reserved(state)?;
                    let comments = state.iterator.comments();

                    let current = state.iterator.current();
                    if current.kind == TokenKind::LeftParen || current.kind == TokenKind::Generic {
                        let generics = if current.kind == TokenKind::Generic {
                            Some(generic::generic_group(state)?)
                        } else {
                            None
                        };

                        if state.iterator.lookahead(1).kind == TokenKind::Ellipsis
                            && state.iterator.lookahead(2).kind == TokenKind::RightParen
                        {
                            Expression::ClassOperation(
                                ClassOperationExpression::StaticMethodClosureCreation {
                                    comments,
                                    class: Box::new(left),
                                    double_colon: position,
                                    method: identifier,
                                    generics,
                                    placeholder: ArgumentPlaceholderExpression {
                                        left_parenthesis: utils::skip(state, TokenKind::LeftParen)?,
                                        ellipsis: utils::skip(state, TokenKind::Ellipsis)?,
                                        right_parenthesis: utils::skip(
                                            state,
                                            TokenKind::RightParen,
                                        )?,
                                        comments: state.iterator.comments(),
                                    },
                                },
                            )
                        } else {
                            Expression::ClassOperation(ClassOperationExpression::StaticMethodCall {
                                comments,
                                class: Box::new(left),
                                double_colon: position,
                                method: identifier,
                                generics,
                                arguments: argument::argument_list_expression(state)?,
                            })
                        }
                    } else {
                        Expression::ClassOperation(ClassOperationExpression::ConstantFetch {
                            comments,
                            class: Box::new(left),
                            double_colon: position,
                            constant: identifier,
                        })
                    }
                }
                TokenKind::Class => {
                    state.iterator.next();

                    Expression::ClassOperation(ClassOperationExpression::ConstantFetch {
                        comments: state.iterator.comments(),
                        class: Box::new(left),
                        double_colon: position,
                        constant: Identifier {
                            position: current.position,
                            value: current.value.clone(),
                        },
                    })
                }
                _ => {
                    crate::parser_bail!(
                        state,
                        unexpected_token(vec!["a variable", "an identifier", "class"], current)
                    );
                }
            }
        }
        TokenKind::Arrow | TokenKind::QuestionArrow => {
            let position = state.iterator.current().position;
            state.iterator.next();

            let identifier = identifier::identifier_maybe_reserved(state)?;
            let comments = state.iterator.comments();

            let current = state.iterator.current();
            if current.kind == TokenKind::LeftParen || current.kind == TokenKind::Generic {
                let generics = if current.kind == TokenKind::Generic {
                    Some(generic::generic_group(state)?)
                } else {
                    None
                };

                if kind == &TokenKind::QuestionArrow {
                    let arguments = argument::argument_list_expression(state)?;

                    Expression::ObjectOperation(ObjectOperationExpression::NullsafeMethodCall {
                        comments,
                        object: Box::new(left),
                        method: identifier,
                        generics,
                        question_arrow: position,
                        arguments,
                    })
                } else {
                    // `(...)` closure creation
                    if state.iterator.lookahead(1).kind == TokenKind::Ellipsis
                        && state.iterator.lookahead(2).kind == TokenKind::RightParen
                    {
                        Expression::ObjectOperation(
                            ObjectOperationExpression::MethodClosureCreation {
                                comments,
                                object: Box::new(left),
                                method: identifier,
                                generics,
                                arrow: position,
                                placeholder: ArgumentPlaceholderExpression {
                                    left_parenthesis: utils::skip(state, TokenKind::LeftParen)?,
                                    ellipsis: utils::skip(state, TokenKind::Ellipsis)?,
                                    right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                                    comments: state.iterator.comments(),
                                },
                            },
                        )
                    } else {
                        let arguments = argument::argument_list_expression(state)?;

                        Expression::ObjectOperation(ObjectOperationExpression::MethodCall {
                            comments,
                            object: Box::new(left),
                            method: identifier,
                            generics,
                            arrow: position,
                            arguments,
                        })
                    }
                }
            } else if kind == &TokenKind::QuestionArrow {
                Expression::ObjectOperation(ObjectOperationExpression::NullsafePropertyFetch {
                    comments,
                    object: Box::new(left),
                    question_arrow: position,
                    property: identifier,
                })
            } else {
                Expression::ObjectOperation(ObjectOperationExpression::PropertyFetch {
                    comments,
                    object: Box::new(left),
                    arrow: position,
                    property: identifier,
                })
            }
        }
        TokenKind::Increment => {
            let position = state.iterator.current().position;
            state.iterator.next();

            Expression::ArithmeticOperation(ArithmeticOperationExpression::PostIncrement {
                left: Box::new(left),
                increment: position,
            })
        }
        TokenKind::Decrement => {
            let position = state.iterator.current().position;
            state.iterator.next();

            Expression::ArithmeticOperation(ArithmeticOperationExpression::PostDecrement {
                left: Box::new(left),
                decrement: position,
            })
        }
        _ => unreachable!(),
    })
}

#[inline(always)]
pub fn is_postfix(t: &TokenKind) -> bool {
    matches!(
        t,
        TokenKind::Increment
            | TokenKind::Decrement
            | TokenKind::LeftParen
            | TokenKind::LeftBracket
            | TokenKind::Arrow
            | TokenKind::QuestionArrow
            | TokenKind::DoubleColon
            | TokenKind::Generic
            | TokenKind::DoubleQuestion
    )
}
