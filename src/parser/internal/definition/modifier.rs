use crate::lexer::token::TokenKind;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::modifier::{ModifierDefinition, ModifierGroupDefinition};

pub fn collect(state: &mut State) -> ParseResult<ModifierGroupDefinition> {
    let mut modifiers: Vec<ModifierDefinition> = vec![];

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
        modifiers.push(match current_kind {
            TokenKind::Private => {
                ModifierDefinition::Private(utils::skip_keyword(state, TokenKind::Private)?)
            }
            TokenKind::Protected => {
                ModifierDefinition::Protected(utils::skip_keyword(state, TokenKind::Protected)?)
            }
            TokenKind::Public => {
                ModifierDefinition::Public(utils::skip_keyword(state, TokenKind::Public)?)
            }
            TokenKind::Final => {
                ModifierDefinition::Final(utils::skip_keyword(state, TokenKind::Final)?)
            }
            TokenKind::Abstract => {
                ModifierDefinition::Abstract(utils::skip_keyword(state, TokenKind::Abstract)?)
            }
            TokenKind::Static => {
                ModifierDefinition::Static(utils::skip_keyword(state, TokenKind::Static)?)
            }
            TokenKind::Readonly => {
                ModifierDefinition::Readonly(utils::skip_keyword(state, TokenKind::Readonly)?)
            }
            _ => unreachable!(),
        });

        current = state.iterator.current().clone();
        current_kind = current.kind;
        current_position = current.position;
    }

    Ok(ModifierGroupDefinition {
        position: current_position,
        modifiers,
    })
}
