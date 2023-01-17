use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::expression;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::modifier::ModifierGroupDefinition;
use crate::tree::definition::property::PropertyDefinition;
use crate::tree::definition::property::PropertyEntryDefinition;

pub fn property_definition(
    state: &mut State,
    modifiers: ModifierGroupDefinition,
) -> ParseResult<PropertyDefinition> {
    let type_definition = r#type::type_definition(state)?;
    let variable = variable::parse(state)?;

    let attributes = state.get_attributes();
    let current = state.iterator.current();

    let entry = if current.kind == TokenKind::Equals {
        PropertyEntryDefinition::Initialized {
            variable,
            equals: utils::skip(state, TokenKind::Equals)?,
            value: expression::create(state)?,
        }
    } else {
        PropertyEntryDefinition::Uninitialized { variable }
    };

    Ok(PropertyDefinition {
        type_definition,
        modifiers,
        attributes,
        entry,
        semicolon: utils::skip(state, TokenKind::SemiColon)?,
    })
}
