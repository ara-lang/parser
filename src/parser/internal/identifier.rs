use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::template;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::identifier::Identifier;
use crate::tree::identifier::TemplatedIdentifier;

/// Expect an identifier such as `Foo`, `Foo\Bar` for a namespace name.
///
/// The identifier can be either a simple identifier or a qualified identifier.
/// The identifier can be a reserved keyword.
#[inline(always)]
pub fn namespace_identifier(state: &mut State) -> ParseResult<Identifier> {
    let current = state.iterator.current();

    match &current.kind {
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            let position = current.position;

            state.iterator.next();

            Ok(Identifier {
                position,
                value: current.value.clone(),
            })
        }
        _ if is_reserved_identifier(&current.kind) => {
            let name = current.to_string().into();
            let position = current.position;
            state.iterator.next();

            Ok(Identifier {
                position,
                value: name,
            })
        }
        _ => {
            crate::parser_bail!(
                state,
                unexpected_token(vec!["an identifier".to_owned()], current)
            )
        }
    }
}

/// Expect an unqualified identifier such as Foo or Bar for a `class`, `interface`, or an `enum` name.
///
/// The identifier can only be a simple identifier.
/// The identifier may be one of the following reserved keywords:
///
/// - `enum`
/// - `from`
/// - `vec`
/// - `dict`
/// - `where`
/// - `type`
/// - `in`
/// - `into`
/// - `using`
/// - `async`
/// - `await`
/// - `concurrently`
pub fn classname_identifier(state: &mut State) -> ParseResult<Identifier> {
    let current = state.iterator.current();
    match &current.kind {
        TokenKind::Identifier => {
            let position = current.position;

            state.iterator.next();

            Ok(Identifier {
                position,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum
        | TokenKind::From
        | TokenKind::Where
        | TokenKind::Type
        | TokenKind::In
        | TokenKind::Into
        | TokenKind::Using
        | TokenKind::Dict
        | TokenKind::Vec
        | TokenKind::Async
        | TokenKind::Await
        | TokenKind::Concurrently => {
            let position = current.position;
            let name = current.to_string().into();

            state.iterator.next();

            Ok(Identifier {
                position,
                value: name,
            })
        }
        t if is_reserved_identifier(t) => {
            state.iterator.next();

            let identifier = Identifier {
                position: current.position,
                value: current.to_string().into(),
            };

            crate::parser_report!(
                state,
                reserved_keyword_cannot_be_used_for_type_name(&identifier)
            );

            Ok(identifier)
        }
        _ => {
            crate::parser_bail!(
                state,
                unexpected_token(vec!["an identifier".to_owned()], current)
            )
        }
    }
}

/// Expect an unqualified identifier such as FOO or BAR for a `constant` name.
///
/// The identifier can only be a simple identifier.
/// The identifier may be one of the following reserved keywords:
///
/// - `enum`
/// - `from`
/// - `where`
/// - `type`
#[inline(always)]
pub fn constant_identifier(state: &mut State) -> ParseResult<Identifier> {
    let current = state.iterator.current();
    match &current.kind {
        TokenKind::Identifier => {
            let position = current.position;

            state.iterator.next();

            Ok(Identifier {
                position,
                value: current.value.clone(),
            })
        }
        TokenKind::Class => {
            let position = current.position;
            let name = current.to_string().into();

            state.iterator.next();

            let identifier = Identifier {
                position,
                value: name,
            };

            crate::parser_report!(
                state,
                reserved_keyword_cannot_be_used_for_constant_name(&identifier)
            );

            Ok(identifier)
        }
        t if is_reserved_identifier(t) => {
            state.iterator.next();

            let identifier = Identifier {
                position: current.position,
                value: current.to_string().into(),
            };

            Ok(identifier)
        }
        _ => {
            crate::parser_bail!(
                state,
                unexpected_token(vec!["an identifier".to_owned()], current)
            )
        }
    }
}

/// Expect an unqualified identifier such as FOO or BAR for a `type` name.
///
/// The identifier can only be a simple identifier.
/// The identifier may be one of the following reserved keywords:
///
/// - `enum`
/// - `from`
/// - `where`
/// - `type`
/// - `in`
/// - `into`
/// - `using`
///
/// Unlike `constant` name, `type` name can be templated ( e.g `Callback<U, V>` )
#[inline(always)]
pub fn type_identifier(state: &mut State) -> ParseResult<TemplatedIdentifier> {
    let current = state.iterator.current();
    let name = match &current.kind {
        TokenKind::Identifier => {
            state.iterator.next();

            Identifier {
                position: current.position,
                value: current.value.clone(),
            }
        }
        TokenKind::Enum
        | TokenKind::From
        | TokenKind::Where
        | TokenKind::Type
        | TokenKind::In
        | TokenKind::Into
        | TokenKind::Using => {
            state.iterator.next();

            Identifier {
                position: current.position,
                value: current.value.clone(),
            }
        }
        t if is_reserved_identifier(t) => {
            state.iterator.next();

            let identifier = Identifier {
                position: current.position,
                value: current.to_string().into(),
            };

            crate::parser_report!(
                state,
                reserved_keyword_cannot_be_used_for_type_name(&identifier)
            );

            identifier
        }
        _ => {
            crate::parser_bail!(
                state,
                unexpected_token(vec!["an identifier".to_owned()], current)
            )
        }
    };

    let templates = if state.iterator.current().kind == TokenKind::LessThan {
        Some(template::type_template_group_definition(state)?)
    } else {
        None
    };

    Ok(TemplatedIdentifier { name, templates })
}

pub fn fully_qualified_templated_identifier(state: &mut State) -> ParseResult<TemplatedIdentifier> {
    let name = fully_qualified_type_identifier(state)?;
    let templates = if state.iterator.current().kind == TokenKind::LessThan {
        Some(template::type_template_group_definition(state)?)
    } else {
        None
    };

    Ok(TemplatedIdentifier { name, templates })
}

pub fn fully_qualified_templated_identifier_including_self(
    state: &mut State,
) -> ParseResult<TemplatedIdentifier> {
    let name = fully_qualified_type_identifier_including_self(state)?;
    let templates = if state.iterator.current().kind == TokenKind::LessThan {
        Some(template::type_template_group_definition(state)?)
    } else {
        None
    };

    Ok(TemplatedIdentifier { name, templates })
}

/// Expect an unqualified identifier such as Foo or Bar.
pub fn identifier(state: &mut State) -> ParseResult<Identifier> {
    let current = state.iterator.current();
    if let TokenKind::Identifier = &current.kind {
        let position = current.position;

        state.iterator.next();

        Ok(Identifier {
            position,
            value: current.value.clone(),
        })
    } else {
        crate::parser_bail!(
            state,
            unexpected_token(vec!["an identifier".to_owned()], current)
        );
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn fully_qualified_type_identifier(state: &mut State) -> ParseResult<Identifier> {
    let current = state.iterator.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let position = current.position;

            state.iterator.next();

            Ok(Identifier {
                position,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum
        | TokenKind::From
        | TokenKind::Where
        | TokenKind::Type
        | TokenKind::In
        | TokenKind::Into
        | TokenKind::Using
        | TokenKind::Dict
        | TokenKind::Vec
        | TokenKind::Async
        | TokenKind::Await
        | TokenKind::Concurrently => {
            let position = current.position;
            let name = current.to_string().into();

            state.iterator.next();

            Ok(Identifier {
                position,
                value: name,
            })
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.iterator.next();

            let identifier = Identifier {
                position: current.position,
                value: current.to_string().into(),
            };

            crate::parser_report!(state, type_cannot_be_used_in_current_context(&identifier));

            Ok(identifier)
        }
        t if is_reserved_identifier(t) => {
            state.iterator.next();

            let identifier = Identifier {
                position: current.position,
                value: current.to_string().into(),
            };

            crate::parser_report!(
                state,
                reserved_keyword_cannot_be_used_for_type_name(&identifier)
            );

            Ok(identifier)
        }
        _ => crate::parser_bail!(
            state,
            unexpected_token(vec!["an identifier".to_owned()], current)
        ),
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn fully_qualified_type_identifier_including_self(
    state: &mut State,
) -> ParseResult<Identifier> {
    let current = state.iterator.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let position = current.position;

            state.iterator.next();

            Ok(Identifier {
                position,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum
        | TokenKind::From
        | TokenKind::Self_
        | TokenKind::Static
        | TokenKind::Parent
        | TokenKind::Type
        | TokenKind::In
        | TokenKind::Into
        | TokenKind::Using
        | TokenKind::Where
        | TokenKind::Dict
        | TokenKind::Vec
        | TokenKind::Async
        | TokenKind::Await
        | TokenKind::Concurrently => {
            let position = current.position;
            let name = current.to_string().into();

            state.iterator.next();

            Ok(Identifier {
                position,
                value: name,
            })
        }
        t if is_reserved_identifier(t) => {
            state.iterator.next();

            let identifier = Identifier {
                position: current.position,
                value: current.to_string().into(),
            };

            crate::parser_report!(
                state,
                reserved_keyword_cannot_be_used_for_type_name(&identifier)
            );

            Ok(identifier)
        }
        _ => crate::parser_bail!(
            state,
            unexpected_token(vec!["an identifier".to_owned()], current)
        ),
    }
}

pub fn identifier_maybe_reserved(state: &mut State) -> ParseResult<Identifier> {
    let current = state.iterator.current();

    if is_reserved_identifier(&current.kind) {
        let name = current.to_string().into();
        let position = current.position;
        state.iterator.next();

        Ok(Identifier {
            position,
            value: name,
        })
    } else {
        identifier(state)
    }
}

pub fn identifier_maybe_soft_reserved(state: &mut State) -> ParseResult<Identifier> {
    let current = state.iterator.current();

    if is_soft_reserved_identifier(&current.kind) {
        let name = current.to_string().into();
        let position = current.position;
        state.iterator.next();

        Ok(Identifier {
            position,
            value: name,
        })
    } else {
        identifier(state)
    }
}

pub fn is_identifier_maybe_reserved(kind: &TokenKind) -> bool {
    if let TokenKind::Identifier = kind {
        return true;
    }

    is_reserved_identifier(kind)
}

pub fn is_soft_reserved_identifier(kind: &TokenKind) -> bool {
    matches!(kind, |TokenKind::Parent| TokenKind::Self_
        | TokenKind::True
        | TokenKind::False
        | TokenKind::Type
        | TokenKind::In
        | TokenKind::Into
        | TokenKind::Using
        | TokenKind::Is
        | TokenKind::List
        | TokenKind::Null
        | TokenKind::Concurrently
        | TokenKind::Async
        | TokenKind::Await
        | TokenKind::Where
        | TokenKind::Enum
        | TokenKind::From
        | TokenKind::Readonly)
}

pub fn is_reserved_identifier(kind: &TokenKind) -> bool {
    if is_soft_reserved_identifier(kind) {
        return true;
    }

    matches!(
        kind,
        TokenKind::Static
            | TokenKind::Abstract
            | TokenKind::Final
            | TokenKind::For
            | TokenKind::Private
            | TokenKind::Protected
            | TokenKind::Public
            | TokenKind::Dict
            | TokenKind::Vec
            | TokenKind::LogicalOr
            | TokenKind::LogicalXor
            | TokenKind::LogicalAnd
            | TokenKind::Instanceof
            | TokenKind::New
            | TokenKind::Clone
            | TokenKind::Exit
            | TokenKind::If
            | TokenKind::ElseIf
            | TokenKind::Else
            | TokenKind::EndIf
            | TokenKind::Echo
            | TokenKind::Do
            | TokenKind::While
            | TokenKind::EndWhile
            | TokenKind::EndFor
            | TokenKind::Foreach
            | TokenKind::EndForeach
            | TokenKind::Declare
            | TokenKind::EndDeclare
            | TokenKind::As
            | TokenKind::Try
            | TokenKind::Catch
            | TokenKind::Finally
            | TokenKind::Throw
            | TokenKind::Use
            | TokenKind::Insteadof
            | TokenKind::Global
            | TokenKind::Var
            | TokenKind::Unset
            | TokenKind::Isset
            | TokenKind::Empty
            | TokenKind::Continue
            | TokenKind::Goto
            | TokenKind::Function
            | TokenKind::Const
            | TokenKind::Return
            | TokenKind::Print
            | TokenKind::Yield
            | TokenKind::List
            | TokenKind::Switch
            | TokenKind::EndSwitch
            | TokenKind::Case
            | TokenKind::Default
            | TokenKind::Break
            | TokenKind::Array
            | TokenKind::Callable
            | TokenKind::Extends
            | TokenKind::Implements
            | TokenKind::Namespace
            | TokenKind::Trait
            | TokenKind::Interface
            | TokenKind::Class
            | TokenKind::ClassConstant
            | TokenKind::TraitConstant
            | TokenKind::FunctionConstant
            | TokenKind::MethodConstant
            | TokenKind::LineConstant
            | TokenKind::FileConstant
            | TokenKind::DirConstant
            | TokenKind::NamespaceConstant
            | TokenKind::HaltCompiler
            | TokenKind::Fn
            | TokenKind::Match
            | TokenKind::Include
            | TokenKind::IncludeOnce
            | TokenKind::Eval
            | TokenKind::Require
            | TokenKind::RequireOnce
            | TokenKind::Die
    )
}
