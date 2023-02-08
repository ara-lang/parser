use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::modifier;
use crate::parser::internal::definition::parameter;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::expression;
use crate::parser::internal::statement::block;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::function::FunctionLikeReturnTypeDefinition;
use crate::tree::expression::function::AnonymousFunctionExpression;
use crate::tree::expression::function::AnonymousFunctionUseClauseExpression;
use crate::tree::expression::function::AnonymousFunctionUseClauseVariableExpression;
use crate::tree::expression::function::ArrowFunctionExpression;

pub fn anonymous_function_expression(
    state: &mut State,
) -> ParseResult<AnonymousFunctionExpression> {
    let comments = state.iterator.comments();
    let attributes = state.get_attributes();
    let modifiers = modifier::collect(state)?;

    let function = utils::skip_keyword(state, TokenKind::Function)?;
    let parameters = parameter::function_like_parameter_list_definition(state)?;

    let current = state.iterator.current();
    let uses = if current.kind == TokenKind::Use {
        Some(AnonymousFunctionUseClauseExpression {
            comments: state.iterator.comments(),
            r#use: utils::skip_keyword(state, TokenKind::Use)?,
            left_parenthesis: utils::skip_left_parenthesis(state)?,
            variables: utils::comma_separated::<AnonymousFunctionUseClauseVariableExpression>(
                state,
                &|state| {
                    let use_comments = state.iterator.comments();
                    let var = variable::parse(state)?;

                    Ok(AnonymousFunctionUseClauseVariableExpression {
                        comments: use_comments,
                        variable: var,
                    })
                },
                TokenKind::RightParen,
            )?,
            right_parenthesis: utils::skip_right_parenthesis(state)?,
        })
    } else {
        None
    };

    Ok(AnonymousFunctionExpression {
        comments,
        attributes,
        modifiers,
        function,
        parameters,
        use_clause: uses,
        return_type: FunctionLikeReturnTypeDefinition {
            colon: utils::skip_colon(state)?,
            type_definition: r#type::type_definition(state)?,
        },
        body: block::block_statement(state)?,
    })
}

pub fn arrow_function_expression(state: &mut State) -> ParseResult<ArrowFunctionExpression> {
    let comments = state.iterator.comments();
    let attributes = state.get_attributes();
    let modifiers = modifier::collect(state)?;

    Ok(ArrowFunctionExpression {
        comments,
        attributes,
        modifiers,
        r#fn: utils::skip_keyword(state, TokenKind::Fn)?,
        parameters: parameter::function_like_parameter_list_definition(state)?,
        return_type: FunctionLikeReturnTypeDefinition {
            colon: utils::skip_colon(state)?,
            type_definition: r#type::type_definition(state)?,
        },
        double_arrow: utils::skip(state, TokenKind::DoubleArrow)?,
        body: Box::new(expression::create(state)?),
    })
}
