use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::constant;
use crate::parser::internal::definition::function::method_definition;
use crate::parser::internal::definition::function::MethodDefinitionReference;
use crate::parser::internal::definition::function::MethodDefinitionType;
use crate::parser::internal::definition::modifier;
use crate::parser::internal::definition::template;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::interface::InterfaceDefinition;
use crate::tree::definition::interface::InterfaceDefinitionBody;
use crate::tree::definition::interface::InterfaceDefinitionExtends;
use crate::tree::definition::interface::InterfaceDefinitionMember;
use crate::tree::identifier::Identifier;

pub fn interface_definition(state: &mut State) -> ParseResult<InterfaceDefinition> {
    let comments = state.iterator.comments();
    let span = utils::skip(state, TokenKind::Interface)?;
    let name = identifier::classname_identifier(state)?;
    let templates = if state.iterator.current().kind == TokenKind::LessThan {
        Some(template::template_group_definition(state)?)
    } else {
        None
    };

    let current = state.iterator.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.iterator.next();

        let parents = utils::at_least_one_comma_separated(
            state,
            &identifier::fully_qualified_templated_identifier,
            TokenKind::LeftBrace,
        )?;

        Some(InterfaceDefinitionExtends {
            extends: span,
            parents,
        })
    } else {
        None
    };

    let attributes = state.get_attributes();

    let body = InterfaceDefinitionBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.iterator.current().kind != TokenKind::RightBrace {
                members.push(interface_definition_member(state, &name)?);
            }

            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(InterfaceDefinition {
        comments,
        attributes,
        interface: span,
        name,
        templates,
        extends,
        body,
    })
}

fn interface_definition_member(
    state: &mut State,
    interface_name: &Identifier,
) -> ParseResult<InterfaceDefinitionMember> {
    attribute::gather(state)?;

    let modifiers = modifier::collect(state)?;

    if state.iterator.current().kind == TokenKind::Const {
        let modifiers = modifier::interface_constant_modifier_definition_group(state, modifiers)?;
        constant::classish_constant_definition(state, modifiers)
            .map(InterfaceDefinitionMember::Constant)
    } else {
        let modifiers = modifier::interface_method_modifier_definition_group(state, modifiers)?;

        let method = method_definition(
            state,
            MethodDefinitionType::Abstract,
            modifiers,
            Some(interface_name),
        )?;

        match method {
            MethodDefinitionReference::Abstract(method) => {
                Ok(InterfaceDefinitionMember::Method(method))
            }
            MethodDefinitionReference::AbstractConstructor(ctor) => {
                Ok(InterfaceDefinitionMember::Constructor(ctor))
            }
            MethodDefinitionReference::ConcreteConstructor(_)
            | MethodDefinitionReference::Concrete(_) => unreachable!(),
        }
    }
}
