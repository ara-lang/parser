use crate::lexer::token::TokenKind;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::r#use::UseDefinition;
use crate::tree::definition::r#use::UseDefinitionSymbolAlias;

pub fn use_definition(state: &mut State) -> ParseResult<UseDefinition> {
    let r#use = utils::skip(state, TokenKind::Use)?;

    let current = state.iterator.current();

    match current.kind {
        TokenKind::Function => Ok(UseDefinition::Function {
            r#use,
            function: utils::skip(state, TokenKind::Function)?,
            name: identifier::fully_qualified_type_identifier(state)?,
            alias: if state.iterator.current().kind == TokenKind::As {
                Some(UseDefinitionSymbolAlias {
                    r#as: utils::skip(state, TokenKind::As)?,
                    alias: identifier::classname_identifier(state)?,
                })
            } else {
                None
            },
            semicolon: utils::skip_semicolon(state)?,
        }),
        TokenKind::Const => Ok(UseDefinition::Constant {
            r#use,
            r#const: utils::skip(state, TokenKind::Const)?,
            name: identifier::fully_qualified_type_identifier(state)?,
            alias: if state.iterator.current().kind == TokenKind::As {
                Some(UseDefinitionSymbolAlias {
                    r#as: utils::skip(state, TokenKind::As)?,
                    alias: identifier::classname_identifier(state)?,
                })
            } else {
                None
            },
            semicolon: utils::skip_semicolon(state)?,
        }),
        _ => Ok(UseDefinition::Default {
            r#use,
            name: identifier::fully_qualified_type_identifier(state)?,
            alias: if state.iterator.current().kind == TokenKind::As {
                Some(UseDefinitionSymbolAlias {
                    r#as: utils::skip(state, TokenKind::As)?,
                    alias: identifier::classname_identifier(state)?,
                })
            } else {
                None
            },
            semicolon: utils::skip_semicolon(state)?,
        }),
    }
}
