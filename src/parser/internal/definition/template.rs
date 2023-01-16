use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::template::TemplateDefinition;
use crate::tree::definition::template::TemplateDefinitionTypeConstraint;
use crate::tree::definition::template::TemplateDefinitionVariance;
use crate::tree::definition::template::TemplateGroupDefinition;
use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::utils::CommaSeparated;

pub fn template_group_definition(state: &mut State) -> ParseResult<TemplateGroupDefinition> {
    Ok(TemplateGroupDefinition {
        comments: state.iterator.comments(),
        less_than: utils::skip(state, TokenKind::LessThan)?,
        members: {
            let mut inner = vec![];
            let mut commas = vec![];

            let mut current = state.iterator.current();
            while current.kind != TokenKind::GreaterThan && current.kind != TokenKind::RightShift {
                inner.push({
                    let current = state.iterator.current();
                    let variance = match &current.kind {
                        TokenKind::Plus => {
                            state.iterator.next();

                            TemplateDefinitionVariance::Covariance(current.position)
                        }
                        _ => TemplateDefinitionVariance::Invaraint,
                    };

                    let name = identifier::classname_identifier(state)?;

                    let current = state.iterator.current();
                    let constraint = match &current.kind {
                        TokenKind::As => TemplateDefinitionTypeConstraint::SubType(
                            utils::skip_keyword(state, TokenKind::As)?,
                            r#type::type_definition(state)?,
                        ),
                        _ => TemplateDefinitionTypeConstraint::None,
                    };

                    TemplateDefinition {
                        name,
                        variance,
                        constraint,
                    }
                });

                current = state.iterator.current();
                if current.kind != TokenKind::Comma {
                    break;
                }

                commas.push(current.position);

                state.iterator.next();
                current = state.iterator.current();
            }

            CommaSeparated { inner, commas }
        },
        greater_than: {
            let current = state.iterator.current();

            if let Some(token) = state.ignored_shift_at {
                utils::skip(state, TokenKind::RightShift)?;
                state.ignored_shift_at = None;
                token.position + 1
            } else if current.kind == TokenKind::RightShift {
                state.ignored_shift_at = Some(current);

                current.position
            } else {
                utils::skip(state, TokenKind::GreaterThan)?
            }
        },
    })
}

pub fn type_template_group_definition(
    state: &mut State,
) -> ParseResult<TypeTemplateGroupDefinition> {
    let comments = state.iterator.comments();
    let less_than = utils::skip(state, TokenKind::LessThan)?;

    let members = {
        let mut inner = vec![];
        let mut commas = vec![];

        let mut current = state.iterator.current();
        while current.kind != TokenKind::GreaterThan && current.kind != TokenKind::RightShift {
            inner.push(r#type::type_definition(state)?);

            current = state.iterator.current();
            if current.kind != TokenKind::Comma {
                break;
            }

            commas.push(current.position);

            state.iterator.next();
            current = state.iterator.current();
        }

        CommaSeparated { inner, commas }
    };

    let greater_than = {
        let current = state.iterator.current();

        if let Some(token) = state.ignored_shift_at {
            utils::skip(state, TokenKind::RightShift)?;
            state.ignored_shift_at = None;
            token.position + 1
        } else if current.kind == TokenKind::RightShift {
            state.ignored_shift_at = Some(current);

            current.position
        } else {
            utils::skip(state, TokenKind::GreaterThan)?
        }
    };

    if members.inner.is_empty() {
        crate::parser_report!(
            state,
            expected_at_least_one_type_in_template_group(less_than, greater_than)
        );
    }

    Ok(TypeTemplateGroupDefinition {
        comments,
        less_than,
        members,
        greater_than,
    })
}
