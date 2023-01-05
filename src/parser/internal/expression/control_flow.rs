use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::control_flow::MatchArmConditionExpression;
use crate::tree::expression::control_flow::MatchArmExpression;
use crate::tree::expression::control_flow::MatchBodyExpression;
use crate::tree::expression::control_flow::MatchExpression;
use crate::tree::utils::CommaSeparated;

pub fn match_expression(state: &mut State) -> ParseResult<MatchExpression> {
    Ok(MatchExpression {
        comments: state.iterator.comments(),
        r#match: utils::skip_keyword(state, TokenKind::Match)?,
        expression: Box::new(expression::create(state)?),
        body: MatchBodyExpression {
            left_brace: utils::skip_left_brace(state)?,
            arms: {
                let mut items = Vec::new();
                let mut commas = Vec::new();
                let mut default_arm = None;

                while state.iterator.current().kind != TokenKind::RightBrace {
                    let current = state.iterator.current();
                    let (condition, default) = if current.kind == TokenKind::Default {
                        (
                            MatchArmConditionExpression::Default(utils::skip_keyword(
                                state,
                                TokenKind::Default,
                            )?),
                            true,
                        )
                    } else {
                        (
                            MatchArmConditionExpression::Expressions(utils::comma_separated(
                                state,
                                &expression::create,
                                TokenKind::DoubleArrow,
                            )?),
                            false,
                        )
                    };

                    let arm = MatchArmExpression {
                        condition,
                        arrow: utils::skip_double_arrow(state)?,
                        expression: expression::create(state)?,
                    };

                    if default {
                        if let Some(default_arm) = &default_arm {
                            crate::parser_report!(
                                state,
                                match_expression_cannot_have_multiple_default_arms(
                                    default_arm,
                                    &arm,
                                )
                            );
                        } else {
                            default_arm = Some(arm.clone());
                        }
                    }

                    items.push(arm);

                    let current = state.iterator.current();
                    if current.kind == TokenKind::Comma {
                        state.iterator.next();
                        commas.push(current.position);
                    } else {
                        break;
                    }
                }

                CommaSeparated {
                    inner: items,
                    commas,
                }
            },
            right_brace: utils::skip_right_brace(state)?,
        },
    })
}
