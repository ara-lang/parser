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
use crate::tree::identifier::Identifier;

pub fn class_definition(state: &mut State) -> ParseResult<ClassDefinition> {
    let attributes = state.get_attributes();

    let modifiers = modifier::collect(state)?;
    let modifiers = modifier::class_modifier_definition_group(state, modifiers)?;
    let comments = state.iterator.comments();
    let class = utils::skip(state, TokenKind::Class)?;
    let name = identifier::classname_identifier(state)?;
    let templates = if state.iterator.current().kind == TokenKind::LessThan {
        Some(template::template_group_definition(state)?)
    } else {
        None
    };

    let current = state.iterator.current();
    let extends = if current.kind == TokenKind::Extends {
        let position = current.position;

        state.iterator.next();
        let parent = identifier::fully_qualified_templated_identifier(state)?;

        Some(ClassDefinitionExtends {
            extends: position,
            parent,
        })
    } else {
        None
    };

    let current = state.iterator.current();
    let implements = if current.kind == TokenKind::Implements {
        let position = current.position;

        state.iterator.next();

        let interfaces = utils::at_least_one_comma_separated(
            state,
            &identifier::fully_qualified_templated_identifier,
            TokenKind::LeftBrace,
        )?;

        Some(ClassDefinitionImplements {
            implements: position,
            interfaces,
        })
    } else {
        None
    };

    let has_abstract = modifiers.has_abstract();

    let body = ClassDefinitionBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.iterator.current().kind != TokenKind::RightBrace {
                members.push(class_definition_member(state, has_abstract, &name)?);
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

fn class_definition_member(
    state: &mut State,
    has_abstract: bool,
    name: &Identifier,
) -> ParseResult<ClassDefinitionMember> {
    attribute::gather(state)?;

    let modifiers = modifier::collect(state)?;

    if state.iterator.current().kind == TokenKind::Const {
        let modifiers = modifier::constant_modifier_definition_group(state, modifiers)?;
        return classish_constant_definition(state, modifiers).map(ClassDefinitionMember::Constant);
    }

    if state.iterator.current().kind == TokenKind::Function {
        let modifiers = modifier::method_modifier_definition_group(state, modifiers)?;

        let method = method_definition(
            state,
            MethodDefinitionType::DependingOnModifiers,
            modifiers,
            Some(name),
        )?;

        return match method {
            MethodDefinitionReference::Abstract(method) => {
                if !has_abstract {
                    crate::parser_report!(
                        state,
                        cannot_declare_abstract_method_on_non_abstract_class(name, &method),
                    );
                }

                Ok(ClassDefinitionMember::AbstractMethod(method))
            }
            MethodDefinitionReference::Concrete(method) => {
                Ok(ClassDefinitionMember::ConcreteMethod(method))
            }
            MethodDefinitionReference::AbstractConstructor(ctor) => {
                if !has_abstract {
                    crate::parser_report!(
                        state,
                        cannot_declare_abstract_ctor_on_non_abstract_class(name, &ctor),
                    );
                }

                Ok(ClassDefinitionMember::AbstractConstructor(ctor))
            }
            MethodDefinitionReference::ConcreteConstructor(ctor) => {
                Ok(ClassDefinitionMember::ConcreteConstructor(ctor))
            }
        };
    }

    // e.g: public static
    let modifiers = modifier::property_modifier_definition_group(state, modifiers)?;

    property::property_definition(state, Some(name), modifiers).map(ClassDefinitionMember::Property)
}
