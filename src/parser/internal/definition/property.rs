use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::expression;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::modifier::PropertyModifierDefinitionGroup;
use crate::tree::definition::property::PropertyDefinition;
use crate::tree::definition::property::PropertyEntryDefinition;

pub fn property_definition(
    state: &mut State,
    modifiers: PropertyModifierDefinitionGroup,
) -> ParseResult<PropertyDefinition> {
    let type_definition = r#type::type_definition(state)?;
    let variable = variable::parse(state)?;

    let attributes = state.get_attributes();
    let current = state.iterator.current();

    let entry = if current.kind == TokenKind::Equals {
        PropertyEntryDefinition::Initialized {
            variable,
            equals: utils::skip(state, TokenKind::Equals)?,
            value: {
                let expression = expression::create(state)?;

                if !expression.is_constant(false) {
                    if expression.is_constant(true) {
                        crate::parser_report!(
                            state,
                            invalid_initialization_in_constant_expression(&expression)
                        );
                    } else {
                        crate::parser_report!(state, invalid_constant_expression(&expression));
                    }
                }

                expression
            },
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
