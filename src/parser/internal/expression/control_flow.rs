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
    let r#match = utils::skip_keyword(state, TokenKind::Match)?;

    let expression = if state.iterator.current().kind == TokenKind::LeftBrace {
        None
    } else {
        Some(Box::new(expression::create(state)?))
    };

    Ok(MatchExpression {
        comments: state.iterator.comments(),
        r#match,
        expression,
        body: MatchBodyExpression {
            left_brace: utils::skip_left_brace(state)?,
            arms: {
                let mut items = Vec::new();
                let mut commas = Vec::new();

                while state.iterator.current().kind != TokenKind::RightBrace {
                    let current = state.iterator.current();
                    let condition = if current.kind == TokenKind::Default {
                        MatchArmConditionExpression::Default(utils::skip_keyword(
                            state,
                            TokenKind::Default,
                        )?)
                    } else {
                        MatchArmConditionExpression::Expressions(utils::comma_separated(
                            state,
                            &expression::create,
                            TokenKind::DoubleArrow,
                        )?)
                    };

                    let arm = MatchArmExpression {
                        condition,
                        arrow: utils::skip_double_arrow(state)?,
                        expression: expression::create(state)?,
                    };

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
