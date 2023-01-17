use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::constant::classish_constant_definition;
use crate::parser::internal::definition::function::method_definition;
use crate::parser::internal::definition::function::MethodDefinitionReference;
use crate::parser::internal::definition::function::MethodDefinitionType;
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

    let current = state.iterator.current();
    let extends = if current.kind == TokenKind::Extends {
        let extends = utils::skip_keyword(state, TokenKind::Extends)?;
        let parent = identifier::fully_qualified_templated_identifier(state)?;

        Some(ClassDefinitionExtends { extends, parent })
    } else {
        None
    };

    let current = state.iterator.current();
    let implements = if current.kind == TokenKind::Implements {
        let implements = utils::skip_keyword(state, TokenKind::Implements)?;
        let interfaces = utils::at_least_one_comma_separated(
            state,
            &identifier::fully_qualified_templated_identifier,
            TokenKind::LeftBrace,
        )?;

        Some(ClassDefinitionImplements {
            implements,
            interfaces,
        })
    } else {
        None
    };

    let body = ClassDefinitionBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.iterator.current().kind != TokenKind::RightBrace {
                members.push(class_definition_member(state)?);
            }

            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(ClassDefinition {
        comments,
        class,
        name,
        templates,
        modifiers,
        extends,
        implements,
        attributes,
        body,
    })
}

fn class_definition_member(state: &mut State) -> ParseResult<ClassDefinitionMember> {
    attribute::gather(state)?;

    let modifiers = modifier::collect(state)?;

    if state.iterator.current().kind == TokenKind::Const {
        return classish_constant_definition(state, modifiers).map(ClassDefinitionMember::Constant);
    }

    if state.iterator.current().kind == TokenKind::Function {
        let method = method_definition(state, MethodDefinitionType::Either, modifiers)?;

        return match method {
            MethodDefinitionReference::Abstract(method) => {
                Ok(ClassDefinitionMember::AbstractMethod(method))
            }
            MethodDefinitionReference::Concrete(method) => {
                Ok(ClassDefinitionMember::ConcreteMethod(method))
            }
            MethodDefinitionReference::AbstractConstructor(ctor) => {
                Ok(ClassDefinitionMember::AbstractConstructor(ctor))
            }
            MethodDefinitionReference::ConcreteConstructor(ctor) => {
                Ok(ClassDefinitionMember::ConcreteConstructor(ctor))
            }
        };
    }

    property::property_definition(state, modifiers).map(ClassDefinitionMember::Property)
}
