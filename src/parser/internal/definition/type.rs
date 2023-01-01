use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::template;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::r#type::TypeAliasDefinition;
use crate::tree::definition::r#type::TypeDefinition;

pub fn type_alias_definition(state: &mut State) -> ParseResult<TypeAliasDefinition> {
    Ok(TypeAliasDefinition {
        r#type: utils::skip(state, TokenKind::Type)?,
        name: identifier::type_identifier(state)?,
        equals: utils::skip(state, TokenKind::Equals)?,
        data_type: type_definition(state)?,
        semicolon: utils::skip_semicolon(state)?,
    })
}

pub fn type_definition(state: &mut State) -> ParseResult<TypeDefinition> {
    let current = state.iterator.current();
    if current.kind == TokenKind::Question {
        return nullable(state);
    }

    // (A|B|..)&C.. or (A&B&..)|C..
    if current.kind == TokenKind::LeftParen && state.iterator.lookahead(1).kind != TokenKind::Fn {
        return parenthesized(state);
    }

    let ty = single(state)?;

    let current = state.iterator.current();
    if current.kind == TokenKind::Pipe {
        return union(state, ty, false);
    }

    if current.kind == TokenKind::Ampersand
        && !matches!(
            state.iterator.lookahead(1).kind,
            TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
        )
    {
        return intersection(state, ty, false);
    }

    Ok(ty)
}
fn parenthesized(state: &mut State) -> ParseResult<TypeDefinition> {
    // (A|B|..)&C.. or (A&B&..)|C.. or (A,B,C..)
    let left_parenthesis = utils::skip(state, TokenKind::LeftParen)?;
    let initial_type_definition = single(state)?;
    let current = state.iterator.current();
    match current.kind {
        TokenKind::Pipe => {
            let union = TypeDefinition::Parenthesized {
                left_parenthesis,
                type_definition: Box::new(union(state, initial_type_definition, true)?),
                right_parenthesis: utils::skip_right_parenthesis(state)?,
            };

            intersection(state, union, false)
        }
        TokenKind::Ampersand => {
            let intersection = TypeDefinition::Parenthesized {
                left_parenthesis,
                type_definition: Box::new(intersection(state, initial_type_definition, true)?),
                right_parenthesis: utils::skip_right_parenthesis(state)?,
            };

            union(state, intersection, false)
        }
        TokenKind::Comma => {
            let comma = utils::skip(state, TokenKind::Comma)?;

            let mut previous_type_definitions = [initial_type_definition];
            let mut previous_commas = [comma];

            let mut type_definitions = utils::at_least_one_comma_separated(
                state,
                &type_definition,
                TokenKind::RightParen,
            )?;

            type_definitions.inner = [
                previous_type_definitions.as_mut_slice(),
                type_definitions.inner.as_mut_slice(),
            ]
            .concat();

            type_definitions.commas = [
                previous_commas.as_mut_slice(),
                type_definitions.commas.as_mut_slice(),
            ]
            .concat();

            Ok(TypeDefinition::Tuple {
                left_parenthesis,
                type_definitions,
                right_parenthesis: utils::skip_right_parenthesis(state)?,
            })
        }
        _ => {
            crate::parser_bail!(state, unexpected_token(vec!["|", "&", ","], current));
        }
    }
}

fn single(state: &mut State) -> ParseResult<TypeDefinition> {
    let current = state.iterator.current();

    let position = current.position;
    let name = &current.value[..];
    let lowered_name = name.to_ascii_lowercase();
    let value = lowered_name.as_slice();

    match &current.kind {
        TokenKind::LeftParen => {
            state.iterator.next();

            let r#fn = utils::skip(state, TokenKind::Fn)?;

            Ok(TypeDefinition::Function {
                outer_left_parenthesis: current.position,
                r#fn,
                left_parenthesis: utils::skip(state, TokenKind::LeftParen)?,
                parameter_type_definitions: utils::comma_separated(
                    state,
                    &|state| {
                        let type_definition = type_definition(state)?;
                        if type_definition.is_bottom() {
                            crate::parser_report!(
                                state,
                                bottom_type_cannot_be_used_in_fn_type_parameter(
                                    &type_definition,
                                    r#fn
                                )
                            );
                        }

                        Ok(type_definition)
                    },
                    TokenKind::RightParen,
                )?,
                right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
                colon: utils::skip(state, TokenKind::Colon)?,
                return_type_definition: Box::new(type_definition(state)?),
                outer_right_parenthesis: utils::skip(state, TokenKind::RightParen)?,
            })
        }
        TokenKind::Null => {
            state.iterator.next();

            Ok(TypeDefinition::Null(position))
        }
        TokenKind::True => {
            state.iterator.next();

            Ok(TypeDefinition::True(position))
        }
        TokenKind::False => {
            state.iterator.next();

            Ok(TypeDefinition::False(position))
        }
        TokenKind::Vec => {
            state.iterator.next();

            let templates = template::type_template_group_definition(state)?;

            Ok(TypeDefinition::Vec(position, templates))
        }
        TokenKind::Dict => {
            state.iterator.next();

            let templates = template::type_template_group_definition(state)?;

            Ok(TypeDefinition::Dict(position, templates))
        }
        _ if value == b"iterable" => {
            state.iterator.next();

            let templates = template::type_template_group_definition(state)?;

            Ok(TypeDefinition::Iterable(position, templates))
        }
        _ if value == b"void" => {
            state.iterator.next();

            Ok(TypeDefinition::Void(position))
        }
        _ if value == b"never" => {
            state.iterator.next();

            Ok(TypeDefinition::Never(position))
        }
        _ if value == b"float" => {
            state.iterator.next();

            Ok(TypeDefinition::Float(position))
        }
        _ if value == b"bool" => {
            state.iterator.next();

            Ok(TypeDefinition::Boolean(position))
        }
        _ if value == b"int" => {
            state.iterator.next();

            Ok(TypeDefinition::Integer(position))
        }
        _ if value == b"string" => {
            state.iterator.next();

            Ok(TypeDefinition::String(position))
        }
        _ if value == b"object" => {
            state.iterator.next();

            Ok(TypeDefinition::Object(position))
        }
        _ if value == b"mixed" => {
            state.iterator.next();

            Ok(TypeDefinition::Mixed(position))
        }
        _ if value == b"nonnull" => {
            state.iterator.next();

            Ok(TypeDefinition::NonNull(position))
        }
        _ if value == b"resource" => {
            state.iterator.next();

            Ok(TypeDefinition::Resource(position))
        }
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier
        | TokenKind::Enum
        | TokenKind::From
        | TokenKind::Self_
        | TokenKind::Static
        | TokenKind::Parent => {
            identifier::fully_qualified_templated_identifier_including_self(state)
                .map(TypeDefinition::Identifier)
        }
        c if identifier::is_reserved_identifier(c) => {
            identifier::fully_qualified_templated_identifier_including_self(state)
                .map(TypeDefinition::Identifier)
        }
        _ => {
            crate::parser_bail!(state, unexpected_token(vec!["a type"], current));
        }
    }
}

fn nullable(state: &mut State) -> ParseResult<TypeDefinition> {
    let current = state.iterator.current();

    state.iterator.next();

    let ty = single(state)?;

    if ty.is_standalone() {
        crate::parser_report!(
            state,
            standalone_type_cannot_be_nullable(&ty, current.position)
        );
    }

    Ok(TypeDefinition::Nullable(current.position, Box::new(ty)))
}

fn union(
    state: &mut State,
    type_definition: TypeDefinition,
    within_dnf: bool,
) -> ParseResult<TypeDefinition> {
    let mut last_pipe = utils::skip(state, TokenKind::Pipe)?;

    if type_definition.is_standalone() {
        crate::parser_report!(
            state,
            standalone_type_cannot_be_used_in_union(&type_definition, last_pipe)
        );
    }

    let mut type_definitions = vec![type_definition];

    loop {
        let current = state.iterator.current();
        let type_definition = if current.kind == TokenKind::LeftParen
            && state.iterator.lookahead(1).kind != TokenKind::Fn
        {
            if within_dnf {
                // don't allow nesting.
                crate::parser_report!(
                    state,
                    disjunctive_normal_form_types_cannot_be_nested(current.position)
                );
            }

            state.iterator.next();

            let other = single(state)?;
            let type_definition = intersection(state, other, true)?;

            utils::skip_right_parenthesis(state)?;

            type_definition
        } else {
            let type_definition = single(state)?;
            if type_definition.is_standalone() {
                crate::parser_report!(
                    state,
                    standalone_type_cannot_be_used_in_union(&type_definition, last_pipe)
                );
            }

            type_definition
        };

        type_definitions.push(type_definition);

        if state.iterator.current().kind == TokenKind::Pipe {
            last_pipe = utils::skip(state, TokenKind::Pipe)?;
        } else {
            break;
        }
    }

    Ok(TypeDefinition::Union(type_definitions))
}

fn intersection(
    state: &mut State,
    type_definition: TypeDefinition,
    within_dnf: bool,
) -> ParseResult<TypeDefinition> {
    let mut last_ampersand = utils::skip(state, TokenKind::Ampersand)?;

    if type_definition.is_standalone() {
        crate::parser_report!(
            state,
            standalone_type_cannot_be_used_in_intersection(&type_definition, last_ampersand)
        );
    }

    if type_definition.is_scalar() {
        crate::parser_report!(
            state,
            scalar_type_cannot_be_used_in_intersection(&type_definition, last_ampersand)
        );
    }

    let mut type_definitions = vec![type_definition];

    loop {
        let current = state.iterator.current();
        let type_definition = if current.kind == TokenKind::LeftParen
            && state.iterator.lookahead(1).kind != TokenKind::Fn
        {
            if within_dnf {
                // don't allow nesting.
                crate::parser_report!(
                    state,
                    disjunctive_normal_form_types_cannot_be_nested(current.position)
                );
            }

            state.iterator.next();

            let other = single(state)?;
            let type_definition = union(state, other, true)?;

            utils::skip_right_parenthesis(state)?;

            type_definition
        } else {
            let type_definition = single(state)?;
            if type_definition.is_standalone() {
                crate::parser_report!(
                    state,
                    standalone_type_cannot_be_used_in_intersection(
                        &type_definition,
                        last_ampersand,
                    )
                );
            }

            if type_definition.is_scalar() {
                crate::parser_report!(
                    state,
                    scalar_type_cannot_be_used_in_intersection(&type_definition, last_ampersand)
                );
            }

            type_definition
        };

        type_definitions.push(type_definition);

        if state.iterator.current().kind == TokenKind::Ampersand
            && !matches!(
                state.iterator.lookahead(1).kind,
                TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
            )
        {
            last_ampersand = utils::skip(state, TokenKind::Ampersand)?;
        } else {
            break;
        }
    }

    Ok(TypeDefinition::Intersection(type_definitions))
}
