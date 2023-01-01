use crate::lexer::token::TokenKind;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::Definition;
use crate::tree::definition::DefinitionTree;

pub mod attribute;
pub mod class;
pub mod constant;
pub mod r#enum;
pub mod function;
pub mod interface;
pub mod modifier;
pub mod namespace;
pub mod parameter;
pub mod property;
pub mod template;
pub mod r#type;
pub mod r#use;

pub fn tree(state: &mut State) -> ParseResult<DefinitionTree> {
    let mut definitions = Vec::new();

    while !state.iterator.is_eof() {
        definitions.push(definition(state)?);
    }

    Ok(DefinitionTree {
        definitions,
        eof: state.iterator.current().position,
    })
}

pub fn definition(state: &mut State) -> ParseResult<Definition> {
    let current = state.iterator.current();
    if matches!(current.kind, TokenKind::OpenTag(_)) {
        state.iterator.next();

        crate::parser_report!(state, php_opening_tag_not_supported(current));

        return definition(state);
    }

    if current.kind == TokenKind::Namespace {
        return Ok(Definition::Namespace(Box::new(
            namespace::namespace_definition(state)?,
        )));
    }

    if current.kind == TokenKind::Use {
        return Ok(Definition::Use(Box::new(r#use::use_definition(state)?)));
    }

    if current.kind == TokenKind::Const {
        return Ok(Definition::Constant(Box::new(
            constant::constant_definition(state)?,
        )));
    }

    if current.kind == TokenKind::Type {
        return Ok(Definition::TypeAlias(Box::new(
            r#type::type_alias_definition(state)?,
        )));
    }

    let has_attributes = attribute::gather(state)?;
    let current = state.iterator.current();

    if current.kind == TokenKind::Enum {
        return Ok(Definition::Enum(Box::new(r#enum::enum_definition(state)?)));
    }

    if current.kind == TokenKind::Interface {
        return Ok(Definition::Interface(Box::new(
            interface::interface_definition(state)?,
        )));
    }

    if current.kind == TokenKind::Function {
        return Ok(Definition::Function(Box::new(
            function::function_definition(state)?,
        )));
    }

    if matches!(
        current.kind,
        TokenKind::Readonly | TokenKind::Final | TokenKind::Abstract | TokenKind::Class
    ) {
        return Ok(Definition::Class(Box::new(class::class_definition(state)?)));
    }

    if has_attributes {
        crate::parser_report!(state, missing_item_definition_after_attributes);
    }

    crate::parser_bail!(state, unexpected_token(vec!["a definition"], current));
}
