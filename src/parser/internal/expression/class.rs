use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::class;
use crate::parser::internal::expression::argument;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::class::AnonymousClassExpression;
use crate::tree::expression::operator::ClassOperationExpression;

pub fn anonymous_initialization_class_operation_expression(
    state: &mut State,
) -> ParseResult<ClassOperationExpression> {
    Ok(ClassOperationExpression::AnonymousInitialization {
        comments: state.iterator.comments(),
        new: utils::skip_keyword(state, TokenKind::New)?,
        class: anonymous_class_expression(state)?,
    })
}

pub fn anonymous_class_expression(state: &mut State) -> ParseResult<AnonymousClassExpression> {
    attribute::gather(state)?;

    let attributes = state.get_attributes();
    let class = utils::skip_keyword(state, TokenKind::Class)?;
    let comments = state.iterator.comments();
    let arguments = argument::argument_list_expression(state)?;

    Ok(AnonymousClassExpression {
        comments,
        attributes,
        class,
        arguments,
        extends: class::class_definition_extends(state)?,
        implements: class::class_definition_implements(state)?,
        body: class::class_definition_body(state)?,
    })
}
