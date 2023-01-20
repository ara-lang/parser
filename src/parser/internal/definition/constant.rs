use crate::lexer::token::TokenKind;
use crate::parser::internal::expression;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::constant::ConstantDefinition;
use crate::tree::definition::modifier::ModifierGroupDefinition;

pub fn constant_definition(state: &mut State) -> ParseResult<ConstantDefinition> {
    Ok(ConstantDefinition {
        comments: state.iterator.comments(),
        r#const: utils::skip_keyword(state, TokenKind::Const)?,
        name: identifier::constant_identifier(state)?,
        equals: utils::skip(state, TokenKind::Equals)?,
        value: expression::create(state)?,
        semicolon: utils::skip_semicolon(state)?,
    })
}

pub fn classish_constant_definition(
    state: &mut State,
    modifiers: ModifierGroupDefinition,
) -> ParseResult<ClassishConstantDefinition> {
    Ok(ClassishConstantDefinition {
        comments: state.iterator.comments(),
        attributes: state.get_attributes(),
        modifiers,
        r#const: utils::skip_keyword(state, TokenKind::Const)?,
        name: identifier::constant_identifier(state)?,
        equals: utils::skip(state, TokenKind::Equals)?,
        value: expression::create(state)?,
        semicolon: utils::skip_semicolon(state)?,
    })
}
