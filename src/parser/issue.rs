#![macro_use]

use ara_reporting::annotation::Annotation;
use ara_reporting::issue::Issue;

use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::state::State as ParserState;
use crate::tree::definition::function::ConcreteConstructorDefinition;
use crate::tree::definition::function::ConcreteMethodDefinition;
use crate::tree::identifier::Identifier;
use crate::tree::Node;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ParserIssueCode {
    /// An unreachable code was encountered.
    UnreachableCode = 0,

    /// PHP opening tag is not supported ( code = 1 )
    ///
    /// Example:
    ///
    /// ```ara
    /// <?php
    ///
    /// class Foo {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the opening tag ( `<?php` )
    PHPOpeningTagNotSupported = 1,

    /// PHP closing tag is not supported ( code = 2 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {}
    ///
    /// ?>
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the closing tag ( `?>` )
    PHPClosingTagNotSupported = 2,

    /// Unit enum case cannot have a value ( code = 3 )
    ///
    /// Example:
    ///
    /// ```ara
    /// enum Foo {
    ///    Bar = 1;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the value
    /// - Change the enum to a backed enum
    UnitEnumCaseCannotHaveValue = 3,

    /// Backed enum case must have a value ( code = 4 )
    ///
    /// Example:
    ///
    /// ```ara
    /// enum Foo: int {
    ///   Bar;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Add a value
    /// - Change the enum to a unit enum
    BackedEnumCaseMustHaveValue = 4,

    /// Enum cannot have a constructor ( code = 5 )
    ///
    /// Example:
    ///
    /// ```ara
    /// enum Foo {
    ///     public function __construct() {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the constructor
    /// - Change the enum to a class
    EnumCannotHaveConstructor = 5,

    /// Enum cannot have a magic method ( code = 6 )
    ///
    /// Example:
    ///
    /// ```ara
    /// enum Foo {
    ///    public function __toString() {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the magic method
    EnumCannotHaveMagicMethod = 6,

    /// Missing item definition after attributes ( code = 7 )
    ///
    /// Example:
    ///
    /// ```ara
    /// #[Foo, Bar]
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Add an item definition after the attributes
    /// - Remove the attributes
    MissingItemDefinitionAfterAttributes = 7,

    /// Reserved keyword cannot be used for type name ( code = 8 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class if {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different name
    ReservedKeywordCannotBeUsedForTypeName = 8,

    /// Reserved keyword cannot be used for constant name ( code = 9 )
    ///
    /// Example:
    ///
    /// ```ara
    /// const if = 1;
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different name
    ReservedKeywordCannotBeUsedForConstantName = 9,

    /// Type cannot be used in current context ( code = 10 )
    ///
    /// Example:
    ///
    /// ```ara
    /// use self;
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    TypeCannotBeUsedInCurrentContext = 10,

    /// Missing item expression after attribute(s) ( code = 11 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(): void {
    ///     #[Foo]
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Add an item expression after the attribute(s)
    /// - Remove the attribute(s)
    MissingItemExpressionAfterAttributes = 11,

    /// Enum backing type must be either `int` or `string` ( code = 12 )
    ///
    /// Example:
    ///
    /// ```ara
    /// enum: float {
    ///
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Change the backing type to `int` or `string`
    InvalidEnumBackingType = 12,

    /// Unexpected token ( code = 13 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo() -> void {
    /// }
    /// ```
    UnexpectedToken = 13,

    /// Invalid constant initialization expression ( code = 14 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(
    ///     Closure<(), void> $a = (
    ///         function(): void { }
    ///     )
    /// ): void {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a valid constant initialization expression
    InvalidConstantInitializationExpression = 14,

    /// Invalid empty type template ( code = 15 )
    ///
    /// Example:
    ///
    /// ```ara
    /// type Foo = Bar<>;
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the empty type template
    /// - Add a type template
    ExpectedAtLeastOneTypeInTemplateGroup = 15,
}

pub(crate) fn unreachable_code<M: Into<String>>(state: &ParserState, message: M) -> Issue {
    let token = state.iterator.current();

    Issue::bug(ParserIssueCode::UnreachableCode, message).with_source(
        state.source.name(),
        token.position,
        token.position + token.value.len(),
    )
}

pub(crate) fn php_opening_tag_not_supported(state: &ParserState, token: &Token) -> Issue {
    Issue::error(
        ParserIssueCode::PHPOpeningTagNotSupported,
        format!("PHP opening tag `{}` is not supported", token.value),
    )
    .with_source(
        state.source.name(),
        token.position,
        token.position + token.value.len(),
    )
}

pub(crate) fn php_closing_tag_not_supported(state: &ParserState, token: &Token) -> Issue {
    Issue::error(
        ParserIssueCode::PHPClosingTagNotSupported,
        format!("PHP closing tag `{}` is not supported", token.value),
    )
    .with_source(
        state.source.name(),
        token.position,
        token.position + token.value.len(),
    )
}

pub(crate) fn unit_enum_case_cannot_have_value(
    state: &ParserState,
    r#enum: &Identifier,
    case: &Identifier,
    semicolon: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::UnitEnumCaseCannotHaveValue,
        format!(
            "case `{}::{}` of unit enum `{}` cannot have a value",
            r#enum,
            case,
            state.named(&r#enum),
        ),
    )
    .with_source(origin, case.initial_position(), semicolon + 1)
    .with_annotation(Annotation::secondary(
        origin,
        r#enum.initial_position(),
        r#enum.final_position(),
    ))
    .with_annotation(Annotation::secondary(
        origin,
        case.initial_position(),
        case.final_position(),
    ))
}

pub(crate) fn backed_enum_case_must_have_value(
    state: &ParserState,
    r#enum: &Identifier,
    case: &Identifier,
    semicolon: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::BackedEnumCaseMustHaveValue,
        format!(
            "case `{}::{}` of backed enum `{}` must have a value",
            r#enum,
            case,
            state.named(&r#enum),
        ),
    )
    .with_source(origin, case.initial_position(), semicolon + 1)
    .with_annotation(Annotation::secondary(
        origin,
        r#enum.initial_position(),
        r#enum.final_position(),
    ))
}

pub(crate) fn enum_cannot_have_constructor(
    state: &ParserState,
    r#enum: &Identifier,
    constructor: &ConcreteConstructorDefinition,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::EnumCannotHaveConstructor,
        format!("enum `{}` cannot have constructor", state.named(&r#enum),),
    )
    .with_source(
        origin,
        constructor.initial_position(),
        constructor.final_position(),
    )
    .with_annotation(Annotation::secondary(
        origin,
        r#enum.initial_position(),
        r#enum.final_position(),
    ))
}

pub(crate) fn enum_cannot_have_magic_method(
    state: &ParserState,
    r#enum: &Identifier,
    method: &ConcreteMethodDefinition,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::EnumCannotHaveMagicMethod,
        format!("enum `{}` cannot have magic method", state.named(&r#enum),),
    )
    .with_source(origin, method.initial_position(), method.final_position())
    .with_annotation(Annotation::secondary(
        origin,
        r#enum.initial_position(),
        r#enum.final_position(),
    ))
}

pub(crate) fn missing_item_definition_after_attributes(state: &ParserState) -> Issue {
    let origin = state.source.name();
    let current = state.iterator.current();

    let mut issue = Issue::error(
        ParserIssueCode::MissingItemDefinitionAfterAttributes,
        "missing item definition after attribute(s)",
    )
    .with_source(
        origin,
        current.position,
        current.position + current.value.len(),
    )
    .with_note("an item definition can be a class, an interface, an enum, or a function.")
    .with_note("try adding an item definition after the attribute(s).");

    for attribute in &state.attributes {
        issue = issue.with_annotation(Annotation::primary(
            origin,
            attribute.initial_position(),
            attribute.final_position(),
        ));
    }

    issue
}

pub(crate) fn reserved_keyword_cannot_be_used_for_type_name(
    state: &ParserState,
    identifier: &Identifier,
) -> Issue {
    Issue::error(
        ParserIssueCode::ReservedKeywordCannotBeUsedForTypeName,
        format!(
            "reserved keyword `{}` cannot be used as a type name",
            identifier,
        ),
    )
    .with_source(
        state.source.name(),
        identifier.initial_position(),
        identifier.final_position(),
    )
}

pub(crate) fn reserved_keyword_cannot_be_used_for_constant_name(
    state: &ParserState,
    identifier: &Identifier,
) -> Issue {
    Issue::error(
        ParserIssueCode::ReservedKeywordCannotBeUsedForConstantName,
        format!(
            "reserved keyword `{}` cannot be used as a constant name",
            identifier,
        ),
    )
    .with_source(
        state.source.name(),
        identifier.initial_position(),
        identifier.final_position(),
    )
}

pub(crate) fn type_cannot_be_used_in_current_context(
    state: &ParserState,
    identifier: &Identifier,
) -> Issue {
    Issue::error(
        ParserIssueCode::TypeCannotBeUsedInCurrentContext,
        format!(
            "type `{}` cannot be used in the current context",
            identifier,
        ),
    )
    .with_source(
        state.source.name(),
        identifier.initial_position(),
        identifier.final_position(),
    )
}

pub(crate) fn missing_item_expression_after_attributes(state: &ParserState) -> Issue {
    let origin = state.source.name();
    let current = state.iterator.current();

    let mut issue = Issue::error(
        ParserIssueCode::MissingItemExpressionAfterAttributes,
        "missing item expression after attribute(s)",
    )
    .with_source(
        origin,
        current.position,
        current.position + current.value.len(),
    )
    .with_note("an item expression can be an anonymous function, or an arrow function.")
    .with_note("try adding an item expression after the attribute(s)");

    for attribute in &state.attributes {
        issue = issue.with_annotation(Annotation::primary(
            origin,
            attribute.initial_position(),
            attribute.final_position(),
        ));
    }

    issue
}

pub(crate) fn invalid_enum_backing_type(
    state: &ParserState,
    backing_identifier: &Identifier,
) -> Issue {
    Issue::error(
        ParserIssueCode::InvalidEnumBackingType,
        format!("invalid enum backing type `{}`", backing_identifier),
    )
    .with_source(
        state.source.name(),
        backing_identifier.initial_position(),
        backing_identifier.final_position(),
    )
    .with_note("the only valid enum backing types are `int`, and `string`.")
}

pub(crate) fn unexpected_token<T: ToString>(
    state: &ParserState,
    expected: Vec<T>,
    found: &Token,
) -> Issue {
    let found_name = match &found.kind {
        TokenKind::Eof => "end of file".to_string(),
        kind => match kind {
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::FullyQualifiedIdentifier => "identifier".to_string(),
            TokenKind::Variable => "variable".to_string(),
            TokenKind::LiteralInteger
            | TokenKind::LiteralFloat
            | TokenKind::LiteralString
            | TokenKind::Null
            | TokenKind::False
            | TokenKind::True => "literal".to_string(),
            _ => format!("token `{}`", found.value),
        },
    };

    let message = if expected.is_empty() {
        format!("unexpected {}", found_name)
    } else {
        let expected: Vec<String> = expected
            .iter()
            .map(|s| {
                let s: String = s.to_string();

                if s.starts_with("a ") || s.starts_with("an ") {
                    s
                } else {
                    format!("`{}`", s)
                }
            })
            .collect();

        let length = expected.len();
        let expected = if length > 2 {
            let (left, right) = expected.split_at(length - 1);

            format!("{}, or {}", left.join(", "), right[0])
        } else {
            expected.join(", or ")
        };

        format!("unexpected {}, expected {}", found_name, expected)
    };

    Issue::error(ParserIssueCode::UnexpectedToken, message).with_source(
        state.source.name(),
        found.position,
        found.position + found.value.len(),
    )
}

pub(crate) fn invalid_constant_initialization_expression(
    state: &ParserState,
    expression: &dyn Node,
) -> Issue {
    Issue::error(
        ParserIssueCode::InvalidConstantInitializationExpression,
        "invalid constant initialization expression",
    )
    .with_source(
        state.source.name(),
        expression.initial_position(),
        expression.final_position(),
    )
}

pub(crate) fn expected_at_least_one_type_in_template_group(
    state: &ParserState,
    less_than: usize,
    greater_than: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ExpectedAtLeastOneTypeInTemplateGroup,
        "expected at least one type in template group",
    )
    .with_source(state.source.name(), less_than, greater_than)
}

impl ::std::fmt::Display for ParserIssueCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "P{:04}", *self as u8)
    }
}

impl From<ParserIssueCode> for String {
    fn from(code: ParserIssueCode) -> Self {
        format!("{}", code)
    }
}
