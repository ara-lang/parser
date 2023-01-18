use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::template;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::r#type::FloatingPointTypeDefinition;
use crate::tree::definition::r#type::SignedIntegerTypeDefinition;
use crate::tree::definition::r#type::TypeAliasDefinition;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::definition::r#type::UnsignedIntegerTypeDefinition;
use crate::tree::expression::literal::Literal;
use crate::tree::expression::literal::LiteralFalse;
use crate::tree::expression::literal::LiteralFloat;
use crate::tree::expression::literal::LiteralInteger;
use crate::tree::expression::literal::LiteralNull;
use crate::tree::expression::literal::LiteralString;
use crate::tree::expression::literal::LiteralTrue;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;

pub fn type_alias_definition(state: &mut State) -> ParseResult<TypeAliasDefinition> {
    Ok(TypeAliasDefinition {
        r#type: utils::skip_keyword(state, TokenKind::Type)?,
        name: identifier::type_identifier(state)?,
        equals: utils::skip(state, TokenKind::Equals)?,
        type_definition: type_definition(state)?,
        semicolon: utils::skip_semicolon(state)?,
    })
}

pub fn type_definition(state: &mut State) -> ParseResult<TypeDefinition> {
    atomic(state)
}

fn atomic(state: &mut State) -> ParseResult<TypeDefinition> {
    let current = state.iterator.current();
    if current.kind == TokenKind::Question {
        return nullable(state);
    }

    let type_definition = if current.kind == TokenKind::LeftParen {
        parenthesized(state)?
    } else {
        single(state)?
    };

    let current = state.iterator.current();
    if current.kind == TokenKind::Pipe {
        return union(state, type_definition);
    }

    if current.kind == TokenKind::Ampersand
        && !matches!(state.iterator.lookahead(1).kind, TokenKind::Ampersand)
    {
        return intersection(state, type_definition);
    }

    Ok(type_definition)
}

fn parenthesized(state: &mut State) -> ParseResult<TypeDefinition> {
    let left_parenthesis = utils::skip(state, TokenKind::LeftParen)?;
    if state.iterator.current().kind == TokenKind::RightParen {
        return Ok(TypeDefinition::Tuple {
            left_parenthesis,
            type_definitions: CommaSeparated {
                inner: Vec::new(),
                commas: Vec::new(),
            },
            right_parenthesis: utils::skip_right_parenthesis(state)?,
        });
    }

    let initial_type_definition = atomic(state)?;
    let current = state.iterator.current();
    match current.kind {
        TokenKind::Comma | TokenKind::RightParen => {
            tuple(state, left_parenthesis, initial_type_definition)
        }
        _ => {
            crate::parser_bail!(state, unexpected_token(vec![",", ")"], current));
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
        TokenKind::Vec => {
            let vec = utils::skip_keyword(state, TokenKind::Vec)?;
            let templates = template::type_template_group_definition(state)?;

            Ok(TypeDefinition::Vec(vec, templates))
        }
        TokenKind::Dict => {
            let dict = utils::skip_keyword(state, TokenKind::Dict)?;
            let templates = template::type_template_group_definition(state)?;

            Ok(TypeDefinition::Dict(dict, templates))
        }
        TokenKind::Null => Ok(TypeDefinition::Literal(Literal::Null(LiteralNull {
            comments: state.iterator.comments(),
            null: utils::skip_keyword(state, TokenKind::Null)?,
        }))),
        TokenKind::True => Ok(TypeDefinition::Literal(Literal::True(LiteralTrue {
            comments: state.iterator.comments(),
            r#true: utils::skip_keyword(state, TokenKind::True)?,
        }))),
        TokenKind::False => Ok(TypeDefinition::Literal(Literal::False(LiteralFalse {
            comments: state.iterator.comments(),
            r#false: utils::skip_keyword(state, TokenKind::False)?,
        }))),
        TokenKind::LiteralInteger => {
            state.iterator.next();

            Ok(TypeDefinition::Literal(Literal::Integer(LiteralInteger {
                comments: state.iterator.comments(),
                value: current.value.clone(),
                position,
            })))
        }
        TokenKind::LiteralFloat => {
            state.iterator.next();

            Ok(TypeDefinition::Literal(Literal::Float(LiteralFloat {
                comments: state.iterator.comments(),
                value: current.value.clone(),
                position,
            })))
        }
        TokenKind::LiteralString => {
            state.iterator.next();

            Ok(TypeDefinition::Literal(Literal::String(LiteralString {
                comments: state.iterator.comments(),
                value: current.value.clone(),
                position,
            })))
        }
        _ if value == b"iterable" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);
            let templates = template::type_template_group_definition(state)?;

            Ok(TypeDefinition::Iterable(keyword, templates))
        }
        _ if value == b"void" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::Void(keyword))
        }
        _ if value == b"never" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::Never(keyword))
        }
        _ if value == b"bool" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::Boolean(keyword))
        }
        _ if value == b"int" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::SignedInteger(
                SignedIntegerTypeDefinition::Default(keyword),
            ))
        }
        _ if value == b"i128" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::SignedInteger(
                SignedIntegerTypeDefinition::I128(keyword),
            ))
        }
        _ if value == b"i64" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::SignedInteger(
                SignedIntegerTypeDefinition::I64(keyword),
            ))
        }
        _ if value == b"i32" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::SignedInteger(
                SignedIntegerTypeDefinition::I32(keyword),
            ))
        }
        _ if value == b"i16" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::SignedInteger(
                SignedIntegerTypeDefinition::I16(keyword),
            ))
        }
        _ if value == b"i8" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::SignedInteger(
                SignedIntegerTypeDefinition::I8(keyword),
            ))
        }
        _ if value == b"uint" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::UnsignedInteger(
                UnsignedIntegerTypeDefinition::Default(keyword),
            ))
        }
        _ if value == b"u32" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::UnsignedInteger(
                UnsignedIntegerTypeDefinition::U32(keyword),
            ))
        }
        _ if value == b"u16" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::UnsignedInteger(
                UnsignedIntegerTypeDefinition::U16(keyword),
            ))
        }
        _ if value == b"u8" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::UnsignedInteger(
                UnsignedIntegerTypeDefinition::U8(keyword),
            ))
        }
        _ if value == b"float" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::FloatingPoint(
                FloatingPointTypeDefinition::Default(keyword),
            ))
        }
        _ if value == b"f64" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::FloatingPoint(
                FloatingPointTypeDefinition::F64(keyword),
            ))
        }
        _ if value == b"f32" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::FloatingPoint(
                FloatingPointTypeDefinition::F32(keyword),
            ))
        }
        _ if value == b"string" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::String(keyword))
        }
        _ if value == b"object" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::Object(keyword))
        }
        _ if value == b"mixed" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::Mixed(keyword))
        }
        _ if value == b"nonnull" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::NonNull(keyword))
        }
        _ if value == b"resource" => {
            state.iterator.next();

            let keyword = Keyword::new(name.into(), current.position);

            Ok(TypeDefinition::Resource(keyword))
        }
        TokenKind::Class => Ok(TypeDefinition::Class(
            utils::skip_keyword(state, TokenKind::Class)?,
            template::type_template_group_definition(state)?,
        )),
        TokenKind::Interface => Ok(TypeDefinition::Interface(
            utils::skip_keyword(state, TokenKind::Interface)?,
            template::type_template_group_definition(state)?,
        )),
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
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

    let type_definition = single(state)?;

    Ok(TypeDefinition::Nullable(
        current.position,
        Box::new(type_definition),
    ))
}

fn tuple(
    state: &mut State,
    left_parenthesis: usize,
    initial_type_definition: TypeDefinition,
) -> ParseResult<TypeDefinition> {
    if state.iterator.current().kind == TokenKind::RightParen {
        let right_parenthesis = utils::skip_right_parenthesis(state)?;

        if let TypeDefinition::Union(_) = initial_type_definition {
            return Ok(TypeDefinition::Parenthesized {
                left_parenthesis,
                type_definition: Box::new(initial_type_definition),
                right_parenthesis,
            });
        }

        if let TypeDefinition::Intersection(_) = initial_type_definition {
            return Ok(TypeDefinition::Parenthesized {
                left_parenthesis,
                type_definition: Box::new(initial_type_definition),
                right_parenthesis,
            });
        }

        return Ok(TypeDefinition::Tuple {
            left_parenthesis,
            type_definitions: CommaSeparated {
                inner: vec![initial_type_definition],
                commas: Vec::new(),
            },
            right_parenthesis,
        });
    }

    let comma = utils::skip(state, TokenKind::Comma)?;

    let mut previous_type_definitions = [initial_type_definition];
    let mut previous_commas = [comma];

    let mut type_definitions =
        utils::comma_separated(state, &type_definition, TokenKind::RightParen)?;

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

    let right_parenthesis = utils::skip_right_parenthesis(state)?;

    Ok(TypeDefinition::Tuple {
        left_parenthesis,
        type_definitions,
        right_parenthesis,
    })
}

fn union(state: &mut State, type_definition: TypeDefinition) -> ParseResult<TypeDefinition> {
    utils::skip(state, TokenKind::Pipe)?;

    let mut type_definitions = vec![type_definition];

    loop {
        let current = state.iterator.current();
        let type_definition = if current.kind == TokenKind::LeftParen {
            let left_parenthesis = current.position;
            state.iterator.next();
            let other = atomic(state)?;

            if let TypeDefinition::Intersection(_) = other {
                TypeDefinition::Parenthesized {
                    left_parenthesis,
                    type_definition: Box::new(other),
                    right_parenthesis: utils::skip_right_parenthesis(state)?,
                }
            } else {
                tuple(state, left_parenthesis, other)?
            }
        } else {
            single(state)?
        };

        type_definitions.push(type_definition);

        if state.iterator.current().kind == TokenKind::Pipe {
            utils::skip(state, TokenKind::Pipe)?;
        } else {
            break;
        }
    }

    Ok(TypeDefinition::Union(type_definitions))
}

fn intersection(state: &mut State, type_definition: TypeDefinition) -> ParseResult<TypeDefinition> {
    utils::skip(state, TokenKind::Ampersand)?;

    let mut type_definitions = vec![type_definition];

    loop {
        let current = state.iterator.current();
        let type_definition = if current.kind == TokenKind::LeftParen {
            let left_parenthesis = current.position;
            state.iterator.next();
            let other = atomic(state)?;

            if let TypeDefinition::Union(_) = other {
                TypeDefinition::Parenthesized {
                    left_parenthesis,
                    type_definition: Box::new(other),
                    right_parenthesis: utils::skip_right_parenthesis(state)?,
                }
            } else {
                tuple(state, left_parenthesis, other)?
            }
        } else {
            single(state)?
        };

        type_definitions.push(type_definition);

        if state.iterator.current().kind == TokenKind::Ampersand
            && !matches!(state.iterator.lookahead(1).kind, TokenKind::Ampersand)
        {
            utils::skip(state, TokenKind::Ampersand)?;
        } else {
            break;
        }
    }

    Ok(TypeDefinition::Intersection(type_definitions))
}
