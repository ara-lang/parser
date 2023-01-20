use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::constant;
use crate::parser::internal::definition::function;
use crate::parser::internal::definition::modifier;
use crate::parser::internal::expression;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::r#enum::BackedEnumBodyDefinition;
use crate::tree::definition::r#enum::BackedEnumCaseDefinition;
use crate::tree::definition::r#enum::BackedEnumDefinition;
use crate::tree::definition::r#enum::BackedEnumMemberDefinition;
use crate::tree::definition::r#enum::BackedEnumTypeDefinition;
use crate::tree::definition::r#enum::EnumDefinition;
use crate::tree::definition::r#enum::EnumImplementsDefinition;
use crate::tree::definition::r#enum::UnitEnumBodyDefinition;
use crate::tree::definition::r#enum::UnitEnumCaseDefinition;
use crate::tree::definition::r#enum::UnitEnumDefinition;
use crate::tree::definition::r#enum::UnitEnumMemberDefinition;
use crate::tree::identifier::Identifier;

pub fn enum_definition(state: &mut State) -> ParseResult<EnumDefinition> {
    let comments = state.iterator.comments();
    let attributes = state.get_attributes();
    let r#enum = utils::skip_keyword(state, TokenKind::Enum)?;
    let name = identifier::classname_identifier(state)?;

    let backed_type: Option<BackedEnumTypeDefinition> =
        if state.iterator.current().kind == TokenKind::Colon {
            let position = utils::skip_colon(state)?;

            let identifier = identifier::identifier(state)?;
            Some(match &identifier.value[..] {
                b"string" => BackedEnumTypeDefinition::String(position, identifier),
                b"int" => BackedEnumTypeDefinition::Int(position, identifier),
                _ => {
                    crate::parser_report!(state, invalid_enum_backing_type(&identifier));

                    // don't panic, just return a dummy value
                    BackedEnumTypeDefinition::String(position, identifier)
                }
            })
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

        Some(EnumImplementsDefinition {
            implements,
            interfaces,
        })
    } else {
        None
    };

    if let Some(backed_type) = backed_type {
        let body = BackedEnumBodyDefinition {
            left_brace: utils::skip_left_brace(state)?,
            members: {
                let mut members = Vec::new();
                while state.iterator.current().kind != TokenKind::RightBrace {
                    if let Some(member) = backed_enum_definition_member(state, &name)? {
                        members.push(member);
                    }
                }

                members
            },
            right_brace: utils::skip_right_brace(state)?,
        };

        Ok(EnumDefinition::Backed(BackedEnumDefinition {
            comments,
            attributes,
            r#enum,
            name,
            implements,
            backed_type,
            body,
        }))
    } else {
        let body = UnitEnumBodyDefinition {
            left_brace: utils::skip_left_brace(state)?,
            members: {
                let mut members = Vec::new();
                while state.iterator.current().kind != TokenKind::RightBrace {
                    if let Some(member) = unit_enum_definition_member(state, &name)? {
                        members.push(member);
                    }
                }
                members
            },
            right_brace: utils::skip_right_brace(state)?,
        };

        Ok(EnumDefinition::Unit(UnitEnumDefinition {
            comments,
            attributes,
            r#enum,
            name,
            implements,
            body,
        }))
    }
}

fn unit_enum_definition_member(
    state: &mut State,
    enum_name: &Identifier,
) -> ParseResult<Option<UnitEnumMemberDefinition>> {
    attribute::gather(state)?;

    let current = state.iterator.current();
    if current.kind == TokenKind::Case {
        let attributes = state.get_attributes();

        let case = utils::skip_keyword(state, TokenKind::Case)?;

        let name = identifier::identifier_maybe_reserved(state)?;

        let current = state.iterator.current();
        if current.kind == TokenKind::Equals {
            // parse the value, but don't do anything with it.
            let _ = utils::skip(state, TokenKind::Equals)?;
            let _ = expression::create(state)?;
            let semicolon = utils::skip_semicolon(state)?;

            crate::parser_report!(
                state,
                unit_enum_case_cannot_have_value(enum_name, &name, semicolon)
            );

            return Ok(None);
        }

        return Ok(Some(UnitEnumMemberDefinition::Case(
            UnitEnumCaseDefinition {
                attributes,
                case,
                name,
                semicolon: utils::skip_semicolon(state)?,
            },
        )));
    }

    let modifiers = modifier::collect(state)?;

    if state.iterator.current().kind == TokenKind::Const {
        return constant::classish_constant_definition(state, modifiers)
            .map(UnitEnumMemberDefinition::Constant)
            .map(Some);
    }

    Ok(Some(UnitEnumMemberDefinition::Method(
        function::method_definition(state, modifiers)?,
    )))
}

fn backed_enum_definition_member(
    state: &mut State,
    enum_name: &Identifier,
) -> ParseResult<Option<BackedEnumMemberDefinition>> {
    attribute::gather(state)?;

    let current = state.iterator.current();
    if current.kind == TokenKind::Case {
        let attributes = state.get_attributes();

        let case = utils::skip_keyword(state, TokenKind::Case)?;
        let name = identifier::identifier_maybe_reserved(state)?;

        let current = state.iterator.current();
        if current.kind == TokenKind::SemiColon {
            // parse the semicolon, but don't do anything with it.
            let semicolon = utils::skip_semicolon(state)?;

            crate::parser_report!(
                state,
                backed_enum_case_must_have_value(enum_name, &name, semicolon)
            );

            return Ok(None);
        }

        return Ok(Some(BackedEnumMemberDefinition::Case(
            BackedEnumCaseDefinition {
                attributes,
                case,
                name,
                equals: utils::skip(state, TokenKind::Equals)?,
                value: expression::create(state)?,
                semicolon: utils::skip_semicolon(state)?,
            },
        )));
    }

    let modifiers = modifier::collect(state)?;

    if state.iterator.current().kind == TokenKind::Const {
        return constant::classish_constant_definition(state, modifiers)
            .map(BackedEnumMemberDefinition::Constant)
            .map(Some);
    }

    Ok(Some(BackedEnumMemberDefinition::Method(
        function::method_definition(state, modifiers)?,
    )))
}
