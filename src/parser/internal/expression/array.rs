use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::array::DictElementExpression;
use crate::tree::expression::array::DictExpression;
use crate::tree::expression::array::VecElementExpression;
use crate::tree::expression::array::VecExpression;

pub fn vec_expression(state: &mut State) -> ParseResult<VecExpression> {
    Ok(VecExpression {
        comments: state.iterator.comments(),
        vec: utils::skip(state, TokenKind::Vec)?,
        left_bracket: utils::skip(state, TokenKind::LeftBracket)?,
        elements: utils::comma_separated(
            state,
            &|state| {
                Ok(VecElementExpression {
                    value: expression::create(state)?,
                })
            },
            TokenKind::RightBracket,
        )?,
        right_bracket: utils::skip(state, TokenKind::RightBracket)?,
    })
}

pub fn dict_expression(state: &mut State) -> ParseResult<DictExpression> {
    Ok(DictExpression {
        comments: state.iterator.comments(),
        dict: utils::skip(state, TokenKind::Dict)?,
        left_bracket: utils::skip(state, TokenKind::LeftBracket)?,
        elements: utils::comma_separated(
            state,
            &|state| {
                Ok(DictElementExpression {
                    key: expression::create(state)?,
                    double_arrow: utils::skip(state, TokenKind::DoubleArrow)?,
                    value: expression::create(state)?,
                })
            },
            TokenKind::RightBracket,
        )?,
        right_bracket: utils::skip(state, TokenKind::RightBracket)?,
    })
}
