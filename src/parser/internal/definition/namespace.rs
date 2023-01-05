use crate::lexer::token::TokenKind;
use crate::parser::internal::definition;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::namespace::NamespaceDefinition;

pub fn namespace_definition(state: &mut State) -> ParseResult<NamespaceDefinition> {
    let namespace = utils::skip_keyword(state, TokenKind::Namespace)?;
    let name = identifier::namespace_identifier(state)?;
    let semicolon = utils::skip_semicolon(state)?;

    state.namespace(name.clone());

    let mut definitions = Vec::new();
    while state.iterator.current().kind != TokenKind::Namespace && !state.iterator.is_eof() {
        definitions.push(definition::definition(state)?);
    }

    Ok(NamespaceDefinition {
        namespace,
        name,
        semicolon,
        definitions,
    })
}
