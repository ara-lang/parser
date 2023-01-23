use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::constant::classish_constant_definition;
use crate::parser::internal::definition::function::method_definition;
use crate::parser::internal::definition::modifier;
use crate::parser::internal::definition::property;
use crate::parser::internal::definition::template;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::class::ClassDefinition;
use crate::tree::definition::class::ClassDefinitionBody;
use crate::tree::definition::class::ClassDefinitionExtends;
use crate::tree::definition::class::ClassDefinitionImplements;
use crate::tree::definition::class::ClassDefinitionMember;

pub fn class_definition(state: &mut State) -> ParseResult<ClassDefinition> {
    let attributes = state.get_attributes();

    let modifiers = modifier::collect(state)?;
    let comments = state.iterator.comments();
    let class = utils::skip_keyword(state, TokenKind::Class)?;
    let name = identifier::classname_identifier(state)?;
    let templates = if state.iterator.current().kind == TokenKind::LessThan {
        Some(template::template_group_definition(state)?)
    } else {
        None
    };

    Ok(ClassDefinition {
        comments,
        class,
        name,
        templates,
        modifiers,
        extends: class_definition_extends(state)?,
        implements: class_definition_implements(state)?,
        attributes,
        body: class_definition_body(state)?,
    })
}

pub fn class_definition_member(state: &mut State) -> ParseResult<ClassDefinitionMember> {
    attribute::gather(state)?;

    let modifiers = modifier::collect(state)?;

    if state.iterator.current().kind == TokenKind::Const {
        return classish_constant_definition(state, modifiers).map(ClassDefinitionMember::Constant);
    }

    if state.iterator.current().kind == TokenKind::Function {
        return method_definition(state, modifiers).map(ClassDefinitionMember::Method);
    }

    property::property_definition(state, modifiers).map(ClassDefinitionMember::Property)
}

pub fn class_definition_extends(state: &mut State) -> ParseResult<Option<ClassDefinitionExtends>> {
    let current = state.iterator.current();
    if current.kind == TokenKind::Extends {
        let extends = utils::skip_keyword(state, TokenKind::Extends)?;
        let parent = identifier::fully_qualified_templated_identifier(state)?;

        Ok(Some(ClassDefinitionExtends { extends, parent }))
    } else {
        Ok(None)
    }
}

pub fn class_definition_implements(
    state: &mut State,
) -> ParseResult<Option<ClassDefinitionImplements>> {
    let current = state.iterator.current();
    if current.kind == TokenKind::Implements {
        let implements = utils::skip_keyword(state, TokenKind::Implements)?;
        let interfaces = utils::at_least_one_comma_separated(
            state,
            &identifier::fully_qualified_templated_identifier,
            TokenKind::LeftBrace,
        )?;

        Ok(Some(ClassDefinitionImplements {
            implements,
            interfaces,
        }))
    } else {
        Ok(None)
    }
}

pub fn class_definition_body(state: &mut State) -> ParseResult<ClassDefinitionBody> {
    Ok(ClassDefinitionBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.iterator.current().kind != TokenKind::RightBrace {
                members.push(class_definition_member(state)?);
            }

            members
        },
        right_brace: utils::skip_right_brace(state)?,
    })
}
