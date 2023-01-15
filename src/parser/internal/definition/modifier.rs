use crate::lexer::token::TokenKind;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::modifier::ClassModifierDefinition;
use crate::tree::definition::modifier::ClassModifierDefinitionGroup;
use crate::tree::definition::modifier::ConstantModifierDefinition;
use crate::tree::definition::modifier::ConstantModifierDefinitionGroup;
use crate::tree::definition::modifier::MethodModifierDefinition;
use crate::tree::definition::modifier::MethodModifierDefinitionGroup;
use crate::tree::definition::modifier::PromotedPropertyModifierDefinition;
use crate::tree::definition::modifier::PromotedPropertyModifierDefinitionGroup;
use crate::tree::definition::modifier::PropertyModifierDefinition;
use crate::tree::definition::modifier::PropertyModifierDefinitionGroup;

#[inline(always)]
pub fn class_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<ClassModifierDefinitionGroup> {
    let mut final_modifier: Option<ClassModifierDefinition> = None;
    let mut abstract_modifier: Option<ClassModifierDefinition> = None;

    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Readonly => {
                modifiers.push(ClassModifierDefinition::Readonly(position));
            }
            TokenKind::Final => {
                let modifier = ClassModifierDefinition::Final(position);
                modifiers.push(modifier.clone());

                if final_modifier.is_none() {
                    final_modifier = Some(modifier);
                }
            }
            TokenKind::Abstract => {
                let modifier = ClassModifierDefinition::Abstract(position);
                modifiers.push(modifier.clone());

                if abstract_modifier.is_none() {
                    abstract_modifier = Some(modifier);
                }
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_class(token.to_string(), position)
                );
            }
        }
    }

    Ok(ClassModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn method_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<MethodModifierDefinitionGroup> {
    let mut final_modifier: Option<MethodModifierDefinition> = None;
    let mut abstract_modifier: Option<MethodModifierDefinition> = None;

    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(MethodModifierDefinition::Private(position));
            }
            TokenKind::Protected => {
                modifiers.push(MethodModifierDefinition::Protected(position));
            }
            TokenKind::Public => {
                modifiers.push(MethodModifierDefinition::Public(position));
            }
            TokenKind::Static => {
                modifiers.push(MethodModifierDefinition::Static(position));
            }
            TokenKind::Final => {
                let modifier = MethodModifierDefinition::Final(position);
                modifiers.push(modifier.clone());

                if final_modifier.is_none() {
                    final_modifier = Some(modifier);
                }
            }
            TokenKind::Abstract => {
                let modifier = MethodModifierDefinition::Abstract(position);
                modifiers.push(modifier.clone());

                if abstract_modifier.is_none() {
                    abstract_modifier = Some(modifier);
                }
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_class_method(token.to_string(), position)
                );
            }
        }
    }

    if let (Some(abstract_modifier), Some(final_modifier)) = (&abstract_modifier, &final_modifier) {
        crate::parser_report!(
            state,
            final_class_member_cannot_be_abstract(final_modifier, abstract_modifier)
        );
    }

    Ok(MethodModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn interface_method_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<MethodModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Public => {
                modifiers.push(MethodModifierDefinition::Public(position));
            }
            TokenKind::Static => {
                modifiers.push(MethodModifierDefinition::Static(position));
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_interface_method(token.to_string(), position)
                );
            }
        }
    }

    Ok(MethodModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn enum_method_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<MethodModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(MethodModifierDefinition::Private(position));
            }
            TokenKind::Protected => {
                modifiers.push(MethodModifierDefinition::Protected(position));
            }
            TokenKind::Public => {
                modifiers.push(MethodModifierDefinition::Public(position));
            }
            TokenKind::Static => {
                modifiers.push(MethodModifierDefinition::Static(position));
            }
            TokenKind::Final => {
                modifiers.push(MethodModifierDefinition::Final(position));
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_enum_method(token.to_string(), position)
                );
            }
        }
    }

    Ok(MethodModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn property_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<PropertyModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(PropertyModifierDefinition::Private(position));
            }
            TokenKind::Protected => {
                modifiers.push(PropertyModifierDefinition::Protected(position));
            }
            TokenKind::Public => {
                modifiers.push(PropertyModifierDefinition::Public(position));
            }
            TokenKind::Static => {
                modifiers.push(PropertyModifierDefinition::Static(position));
            }
            TokenKind::Readonly => {
                modifiers.push(PropertyModifierDefinition::Readonly(position));
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_property(token.to_string(), position)
                );
            }
        }
    }

    Ok(PropertyModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn promoted_property_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<PromotedPropertyModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(PromotedPropertyModifierDefinition::Private(position));
            }
            TokenKind::Protected => {
                modifiers.push(PromotedPropertyModifierDefinition::Protected(position));
            }
            TokenKind::Public => {
                modifiers.push(PromotedPropertyModifierDefinition::Public(position));
            }
            TokenKind::Readonly => {
                modifiers.push(PromotedPropertyModifierDefinition::Readonly(position));
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_promoted_property(token.to_string(), position)
                );
            }
        }
    }

    Ok(PromotedPropertyModifierDefinitionGroup { modifiers })
}

pub fn constant_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<ConstantModifierDefinitionGroup> {
    let mut final_modifier = None;
    let mut private_modifier = None;

    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Private => {
                let modifier = ConstantModifierDefinition::Private(position);
                modifiers.push(modifier.clone());
                private_modifier = Some(modifier);
            }
            TokenKind::Protected => {
                modifiers.push(ConstantModifierDefinition::Protected(position));
            }
            TokenKind::Public => {
                modifiers.push(ConstantModifierDefinition::Public(position));
            }
            TokenKind::Final => {
                let modifier = ConstantModifierDefinition::Final(position);
                modifiers.push(modifier.clone());
                final_modifier = Some(modifier);
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_constant(token.to_string(), position)
                );
            }
        }
    }

    if let (Some(private_modifier), Some(final_modifier)) = (&private_modifier, &final_modifier) {
        crate::parser_report!(
            state,
            private_constant_cannot_be_final(final_modifier, private_modifier)
        );
    }

    Ok(ConstantModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn interface_constant_modifier_definition_group(
    state: &mut State,
    input: Vec<(usize, TokenKind)>,
) -> ParseResult<ConstantModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (position, token) in input {
        match token {
            TokenKind::Public => {
                modifiers.push(ConstantModifierDefinition::Public(position));
            }
            TokenKind::Final => {
                modifiers.push(ConstantModifierDefinition::Final(position));
            }
            _ => {
                crate::parser_report!(
                    state,
                    modifier_cannot_be_used_on_interface_constant(token.to_string(), position)
                );
            }
        }
    }

    Ok(ConstantModifierDefinitionGroup { modifiers })
}

pub fn collect(state: &mut State) -> ParseResult<Vec<(usize, TokenKind)>> {
    let mut collected: Vec<(usize, TokenKind)> = vec![];

    let collectable_tokens = vec![
        TokenKind::Private,
        TokenKind::Protected,
        TokenKind::Public,
        TokenKind::Final,
        TokenKind::Abstract,
        TokenKind::Static,
        TokenKind::Readonly,
    ];

    let mut current = state.iterator.current().clone();
    let mut current_kind = current.kind;
    let mut current_position = current.position;

    while collectable_tokens.contains(&current_kind) {
        if let Some((position, _)) = collected.iter().find(|(_, kind)| kind == &current_kind) {
            crate::parser_report!(
                state,
                duplicate_modifier(current_kind.to_string(), *position, current_position)
            );
        }

        // guard against multiple visibility modifiers, we don't care where these modifiers are used.
        if matches!(
            current_kind,
            TokenKind::Public | TokenKind::Protected | TokenKind::Private
        ) {
            if let Some((position, visibility)) = collected.iter().find(|(_, kind)| {
                matches!(
                    kind,
                    TokenKind::Public | TokenKind::Protected | TokenKind::Private
                )
            }) {
                crate::parser_report!(
                    state,
                    multiple_visibility_modifiers(
                        (*position, visibility.to_string()),
                        (current_position, current_kind.to_string()),
                    )
                );
            }
        }

        collected.push((current_position, current_kind));

        state.iterator.next();

        current = state.iterator.current().clone();
        current_kind = current.kind;
        current_position = current.position;
    }

    Ok(collected)
}
