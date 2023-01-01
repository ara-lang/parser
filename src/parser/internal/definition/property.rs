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
use crate::tree::identifier::Identifier;

pub fn property_definition(
    state: &mut State,
    class_name: Option<&Identifier>,
    modifiers: PropertyModifierDefinitionGroup,
) -> ParseResult<PropertyDefinition> {
    let type_definition = r#type::type_definition(state)?;
    let variable = variable::parse(state)?;

    if type_definition.is_bottom() {
        crate::parser_report!(
            state,
            bottom_type_cannot_be_used_for_property(class_name, &type_definition, &variable)
        );
    }

    if let (Some(readonly), Some(r#static)) = (modifiers.get_readonly(), modifiers.get_static()) {
        crate::parser_report!(
            state,
            readonly_property_cannot_be_static(class_name, &variable, readonly, r#static)
        );
    }

    let attributes = state.get_attributes();
    let current = state.iterator.current();

    let entry = if current.kind == TokenKind::Equals {
        let entry = PropertyEntryDefinition::Initialized {
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
        };

        if let Some(modifier) = modifiers.get_readonly() {
            crate::parser_report!(
                state,
                readonly_property_cannot_have_default_value(class_name, &entry, modifier,)
            );
        }

        entry
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
