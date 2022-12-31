use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::parser::issue;
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
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<ClassModifierDefinitionGroup> {
    let mut final_modifier: Option<ClassModifierDefinition> = None;
    let mut abstract_modifier: Option<ClassModifierDefinition> = None;

    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Readonly => {
                modifiers.push(ClassModifierDefinition::Readonly(span));
            }
            TokenKind::Final => {
                let modifier = ClassModifierDefinition::Final(span);
                modifiers.push(modifier.clone());

                if final_modifier.is_none() {
                    final_modifier = Some(modifier);
                }
            }
            TokenKind::Abstract => {
                let modifier = ClassModifierDefinition::Final(span);
                modifiers.push(modifier.clone());

                if abstract_modifier.is_none() {
                    abstract_modifier = Some(modifier);
                }
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_class(token.to_string(), span)
                );
            }
        }
    }

    if let (Some(abstract_modifier), Some(final_modifier)) = (&abstract_modifier, &final_modifier) {
        issue::report!(
            state,
            final_class_cannot_be_abstract(final_modifier, abstract_modifier)
        );
    }

    Ok(ClassModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn method_modifier_definition_group(
    state: &mut State,
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<MethodModifierDefinitionGroup> {
    let mut final_modifier: Option<MethodModifierDefinition> = None;
    let mut abstract_modifier: Option<MethodModifierDefinition> = None;

    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(MethodModifierDefinition::Private(span));
            }
            TokenKind::Protected => {
                modifiers.push(MethodModifierDefinition::Protected(span));
            }
            TokenKind::Public => {
                modifiers.push(MethodModifierDefinition::Public(span));
            }
            TokenKind::Static => {
                modifiers.push(MethodModifierDefinition::Static(span));
            }
            TokenKind::Final => {
                let modifier = MethodModifierDefinition::Final(span);
                modifiers.push(modifier.clone());

                if final_modifier.is_none() {
                    final_modifier = Some(modifier);
                }
            }
            TokenKind::Abstract => {
                let modifier = MethodModifierDefinition::Final(span);
                modifiers.push(modifier.clone());

                if abstract_modifier.is_none() {
                    abstract_modifier = Some(modifier);
                }
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_class_method(token.to_string(), span)
                );
            }
        }
    }

    if let (Some(abstract_modifier), Some(final_modifier)) = (&abstract_modifier, &final_modifier) {
        issue::report!(
            state,
            final_class_member_cannot_be_abstract(final_modifier, abstract_modifier)
        );
    }

    Ok(MethodModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn interface_method_modifier_definition_group(
    state: &mut State,
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<MethodModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Public => {
                modifiers.push(MethodModifierDefinition::Public(span));
            }
            TokenKind::Static => {
                modifiers.push(MethodModifierDefinition::Static(span));
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_interface_method(token.to_string(), span)
                );
            }
        }
    }

    Ok(MethodModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn enum_method_modifier_definition_group(
    state: &mut State,
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<MethodModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(MethodModifierDefinition::Private(span));
            }
            TokenKind::Protected => {
                modifiers.push(MethodModifierDefinition::Protected(span));
            }
            TokenKind::Public => {
                modifiers.push(MethodModifierDefinition::Public(span));
            }
            TokenKind::Static => {
                modifiers.push(MethodModifierDefinition::Static(span));
            }
            TokenKind::Final => {
                modifiers.push(MethodModifierDefinition::Final(span));
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_enum_method(token.to_string(), span)
                );
            }
        }
    }

    Ok(MethodModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn property_modifier_definition_group(
    state: &mut State,
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<PropertyModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(PropertyModifierDefinition::Private(span));
            }
            TokenKind::Protected => {
                modifiers.push(PropertyModifierDefinition::Protected(span));
            }
            TokenKind::Public => {
                modifiers.push(PropertyModifierDefinition::Public(span));
            }
            TokenKind::Static => {
                modifiers.push(PropertyModifierDefinition::Static(span));
            }
            TokenKind::Readonly => {
                modifiers.push(PropertyModifierDefinition::Readonly(span));
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_property(token.to_string(), span)
                );
            }
        }
    }

    Ok(PropertyModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn promoted_property_modifier_definition_group(
    state: &mut State,
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<PromotedPropertyModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Private => {
                modifiers.push(PromotedPropertyModifierDefinition::Private(span));
            }
            TokenKind::Protected => {
                modifiers.push(PromotedPropertyModifierDefinition::Protected(span));
            }
            TokenKind::Public => {
                modifiers.push(PromotedPropertyModifierDefinition::Public(span));
            }
            TokenKind::Readonly => {
                modifiers.push(PromotedPropertyModifierDefinition::Readonly(span));
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_promoted_property(token.to_string(), span)
                );
            }
        }
    }

    Ok(PromotedPropertyModifierDefinitionGroup { modifiers })
}

pub fn constant_modifier_definition_group(
    state: &mut State,
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<ConstantModifierDefinitionGroup> {
    let mut final_modifier = None;
    let mut private_modifier = None;

    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Private => {
                let modifier = ConstantModifierDefinition::Private(span);
                modifiers.push(modifier.clone());
                private_modifier = Some(modifier);
            }
            TokenKind::Protected => {
                modifiers.push(ConstantModifierDefinition::Protected(span));
            }
            TokenKind::Public => {
                modifiers.push(ConstantModifierDefinition::Public(span));
            }
            TokenKind::Final => {
                let modifier = ConstantModifierDefinition::Final(span);
                modifiers.push(modifier.clone());
                final_modifier = Some(modifier);
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_constant(token.to_string(), span)
                );
            }
        }
    }

    if let (Some(private_modifier), Some(final_modifier)) = (&private_modifier, &final_modifier) {
        issue::report!(
            state,
            private_constant_cannot_be_final(final_modifier, private_modifier)
        );
    }

    Ok(ConstantModifierDefinitionGroup { modifiers })
}

#[inline(always)]
pub fn interface_constant_modifier_definition_group(
    state: &mut State,
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<ConstantModifierDefinitionGroup> {
    let mut modifiers = vec![];
    for (span, token) in input {
        match token {
            TokenKind::Public => {
                modifiers.push(ConstantModifierDefinition::Public(span));
            }
            TokenKind::Final => {
                modifiers.push(ConstantModifierDefinition::Final(span));
            }
            _ => {
                issue::report!(
                    state,
                    modifier_cannot_be_used_on_interface_constant(token.to_string(), span)
                );
            }
        }
    }

    Ok(ConstantModifierDefinitionGroup { modifiers })
}

pub fn collect(state: &mut State) -> ParseResult<Vec<(Span, TokenKind)>> {
    let mut collected: Vec<(Span, TokenKind)> = vec![];

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
    let mut current_span = current.span;

    while collectable_tokens.contains(&current_kind) {
        if let Some((span, _)) = collected.iter().find(|(_, kind)| kind == &current_kind) {
            issue::report!(
                state,
                duplicate_modifier(current_kind.to_string(), *span, current_span)
            );
        }

        // guard against multiple visibility modifiers, we don't care where these modifiers are used.
        if matches!(
            current_kind,
            TokenKind::Public | TokenKind::Protected | TokenKind::Private
        ) {
            if let Some((span, visibility)) = collected.iter().find(|(_, kind)| {
                matches!(
                    kind,
                    TokenKind::Public | TokenKind::Protected | TokenKind::Private
                )
            }) {
                issue::report!(
                    state,
                    multiple_visibility_modifiers(
                        (*span, visibility.to_string()),
                        (current_span, current_kind.to_string()),
                    )
                );
            }
        }

        collected.push((current_span, current_kind));

        state.iterator.next();

        current = state.iterator.current().clone();
        current_kind = current.kind;
        current_span = current.span;
    }

    Ok(collected)
}
