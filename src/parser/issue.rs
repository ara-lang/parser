#![macro_use]

use ara_reporting::annotation::Annotation;
use ara_reporting::issue::Issue;

use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::state::State as ParserState;
use crate::tree::definition::function::AbstractConstructorDefinition;
use crate::tree::definition::function::AbstractMethodDefinition;
use crate::tree::definition::function::ConcreteConstructorDefinition;
use crate::tree::definition::function::ConcreteMethodDefinition;
use crate::tree::definition::function::ConstructorParameterDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#try::TryStatement;
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

    /// Multiple visibility modifiers ( code = 8 )
    ///
    /// Example:
    ///
    /// ```ara
    /// final class Foo {
    ///    public private function bar() {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove one of the visibility modifiers
    MultipleVisibilityModifiers = 8,

    /// Duplicate modifier ( code = 9 )
    ///
    /// Example:
    ///
    /// ```ara
    /// final final class Foo {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove one of the modifiers
    DuplicateModifier = 9,

    /// Reserved keyword cannot be used for type name ( code = 12 )
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
    ReservedKeywordCannotBeUsedForTypeName = 12,

    /// Reserved keyword cannot be used for constant name ( code = 13 )
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
    ReservedKeywordCannotBeUsedForConstantName = 13,

    /// Missing item expression after attribute(s) ( code = 14 )
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
    MissingItemExpressionAfterAttributes = 14,

    /// Final class cannot be abstract ( code = 15 )
    ///
    /// Example:
    ///
    /// ```ara
    /// final abstract class Foo {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the `abstract` modifier
    /// - Remove the `final` modifier
    FinalClassCannotBeAbstract = 15,

    /// Final class member cannot be abstract ( code = 16 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///     final abstract public function bar(): void {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the `abstract` modifier
    /// - Remove the `final` modifier
    FinalClassMemberCannotBeAbstract = 16,

    /// Private constant cannot be final ( code = 17 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///     private final const BAR = 1;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the `final` modifier
    /// - Remove the `private` modifier
    PrivateConstantCannotBeFinal = 17,

    /// Modifier cannot be used on classes ( code = 18 )
    ///
    /// Example:
    ///
    /// ```ara
    /// final class Foo {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnClass = 18,

    /// Modifier cannot be used on class methods ( code = 19 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///     public readonly function bar(): void {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnClassMethod = 19,

    /// Modifier cannot be used on interface methods ( code = 20 )
    ///
    /// Example:
    ///
    /// ```ara
    /// interface Foo {
    ///     private function bar(): void;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnInterfaceMethod = 20,

    /// Modifier cannot be used on enum methods ( code = 21 )
    ///
    /// Example:
    ///
    /// ```ara
    /// enum Foo {
    ///     public abstract function bar(): void {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnEnumMethod = 21,

    /// Modifier cannot be used on properties ( code = 22 )
    ///
    /// Example:
    ///
    ///
    /// ```ara
    /// class Foo {
    ///     public abstract string $bar = "";
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnProperty = 22,

    /// Modifier cannot be used on promoted properties ( code = 23 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///     public function __construct(
    ///         private static string $bar = "",
    ///     ) {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnPromotedProperty = 23,

    /// Modifier cannot be used on constants ( code = 24 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///     public abstract const BAR = 1;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnConstant = 24,

    /// Modifier cannot be used on interface constants ( code = 25 )
    ///
    /// Example:
    ///
    /// ```ara
    /// interface Foo {
    ///     private const BAR = 1;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the modifier
    ModifierCannotBeUsedOnInterfaceConstant = 25,

    /// Match expression cannot have multiple default arms ( code = 26 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(string $input): string {
    ///     match $input {
    ///         "foo" => "bar",
    ///         default => "baz",
    ///         default => "qux",
    ///     }
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove one of the default arms
    MatchExpressionCannotHaveMultipleDefaultArms = 26,

    /// Promoted property cannot be variadic ( code = 27 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///     public function __construct(
    ///         private string ...$bar,
    ///     ) {}
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the variadic declaration ( `...` )
    /// - Demote the property
    PromotedPropertyCannotBeVariadic = 27,

    /// Catch block must have a catch or finally block ( code = 28 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(): void {
    ///   try {
    ///     // ...
    ///   }
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Add a catch or finally block
    /// - Remove the try block
    TryStatementMustHaveCatchOrFinally = 28,

    /// Abstract method cannot be declared on a non-abstract class ( code = 29 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///    abstract function bar(): void;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the `abstract` modifier
    /// - Make the class abstract
    CannotDeclareAbstractMethodOnNonAbstractClass = 29,

    /// Unexpected empty statement ( code = 30 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(): void {
    ///    ;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the empty statement ( `;` )
    UnexpectedEmptyStatement = 30,

    /// Unexpected token
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo() -> void {
    /// }
    /// ```
    UnexpectedToken = 31,

    /// Invalid constant expression ( code = 32 )
    ///
    /// Example:
    ///
    /// ```ara
    /// const FOO = function(): void { };
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a valid constant expression
    InvalidConstantExpression = 32,

    /// Invalid constant initialization expression ( code = 33 )
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
    InvalidConstantInitializationExpression = 33,

    /// Invalid constant initialization expression ( code = 34 )
    ///
    /// Example:
    ///
    /// ```ara
    /// const FOO = new Bar();
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the class instantiation expression(s)
    InvalidInitializationInConstantExpression = 34,
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

pub(crate) fn multiple_visibility_modifiers(
    state: &ParserState,
    first: (usize, String),
    second: (usize, String),
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::MultipleVisibilityModifiers,
        "multiple visibility modifiers are not allowed",
    )
    .with_source(origin, second.0, second.0 + second.1.len())
    .with_annotation(Annotation::primary(
        origin,
        first.0,
        first.0 + first.1.len(),
    ))
}

pub(crate) fn duplicate_modifier(
    state: &ParserState,
    modifier: String,
    first: usize,
    second: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::DuplicateModifier,
        format!("multiple `{}` modifiers are not allowed", modifier),
    )
    .with_source(origin, second, second + modifier.len())
    .with_annotation(Annotation::primary(origin, first, first + modifier.len()))
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

pub(crate) fn final_class_cannot_be_abstract(
    state: &ParserState,
    r#final: &dyn Node,
    r#abstract: &dyn Node,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::FinalClassCannotBeAbstract,
        "final class cannot be abstract",
    )
    .with_source(
        origin,
        r#abstract.initial_position(),
        r#abstract.final_position(),
    )
    .with_annotation(Annotation::primary(
        origin,
        r#final.initial_position(),
        r#final.final_position(),
    ))
    .with_note("a final class cannot be abstract because it cannot be extended by other classes.")
}

pub(crate) fn final_class_member_cannot_be_abstract(
    state: &ParserState,
    r#final: &dyn Node,
    r#abstract: &dyn Node,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::FinalClassMemberCannotBeAbstract,
        "final class member cannot be abstract",
    )
    .with_source(
        origin,
        r#abstract.initial_position(),
        r#abstract.final_position(),
    )
    .with_annotation(Annotation::primary(
        origin,
        r#final.initial_position(),
        r#final.final_position(),
    ))
    .with_note(
        "a final class member cannot be abstract because it cannot be overridden by other classes.",
    )
}

pub(crate) fn private_constant_cannot_be_final(
    state: &ParserState,
    r#private: &dyn Node,
    r#final: &dyn Node,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::PrivateConstantCannotBeFinal,
        "private constant cannot be final",
    )
    .with_source(origin, r#final.initial_position(), r#final.final_position())
    .with_annotation(Annotation::primary(
        origin,
        r#private.initial_position(),
        r#private.final_position(),
    ))
    .with_note(
        "a private constant cannot be final because it cannot be overridden by other classes.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_class(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnClass,
        format!("modifier `{}` cannot be used on a class", modifier),
    )
    .with_source(state.source.name(), position, position + modifier.len())
    .with_note("only the `final`, `abstract`, and `readonly` modifiers can be used on a class.")
}

pub(crate) fn modifier_cannot_be_used_on_class_method(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnClassMethod,
        format!("modifier `{}` cannot be used on a class method", modifier),
    )
    .with_source(
        state.source.name(),
        position,
        position + modifier.len(),
    )
    .with_note(
        "only the `final`, `abstract`, `static`, `private`, `protected`, and `public` modifiers can be used on a class method.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_interface_method(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnInterfaceMethod,
        format!(
            "modifier `{}` cannot be used on an interface method",
            modifier
        ),
    )
    .with_source(state.source.name(), position, position + modifier.len())
    .with_note("only the `static`, and `public` modifiers can be used on an interface method.")
}

pub(crate) fn modifier_cannot_be_used_on_enum_method(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnEnumMethod,
        format!("modifier `{}` cannot be used on an enum method", modifier),
    )
    .with_source(
        state.source.name(),
        position,
        position + modifier.len(),
    )
    .with_note("only the `final`, `static`, and `public`, `protected`, `private` modifiers can be used on an enum method.")
}

pub(crate) fn modifier_cannot_be_used_on_property(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnProperty,
        format!("modifier `{}` cannot be used on a property", modifier)
    )
    .with_source(
        state.source.name(),
        position,
        position + modifier.len(),
    )
    .with_note(
        "only the `static`, `readonly`, `private`, `protected`, and `public` modifiers can be used on a property.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_promoted_property(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnPromotedProperty,
        format!("modifier `{}` cannot be used on a promoted property", modifier)
    )
    .with_source(
        state.source.name(),
        position,
        position + modifier.len(),
    )
    .with_note(
        "only the `readonly`, `private`, `protected`, and `public` modifiers can be used on a promoted property.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_constant(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnConstant,
        format!("modifier `{}` cannot be used on a constant", modifier)
    )
    .with_source(
        state.source.name(),
        position,
        position + modifier.len(),
    )
    .with_note(
        "only the `final`, `private`, `protected`, and `public` modifiers can be used on a constant.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_interface_constant(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnInterfaceConstant,
        format!(
            "modifier `{}` cannot be used on an interface constant",
            modifier
        ),
    )
    .with_source(state.source.name(), position, position + modifier.len())
    .with_note("only the `final`, and `public` modifiers can be used on an interface constant.")
}

pub(crate) fn match_expression_cannot_have_multiple_default_arms(
    state: &ParserState,
    first: &dyn Node,
    second: &dyn Node,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::MatchExpressionCannotHaveMultipleDefaultArms,
        "match expression cannot have multiple default arms",
    )
    .with_source(origin, second.initial_position(), second.final_position())
    .with_annotation(Annotation::primary(
        origin,
        first.initial_position(),
        first.final_position(),
    ))
    .with_note("a match expression can only have one default arm")
}

pub(crate) fn promoted_property_cannot_be_variadic(
    state: &ParserState,
    class_name: Option<&Identifier>,
    promoted_property: &ConstructorParameterDefinition,
) -> Issue {
    let origin = state.source.name();

    let position = promoted_property.ellipsis.unwrap();

    let mut issue = Issue::error(
        ParserIssueCode::PromotedPropertyCannotBeVariadic,
        format!(
            "promoted property `{}::{}` cannot be declared variadic",
            class_name
                .map(|c| state.named(c))
                .unwrap_or_else(|| "anonymous@class".to_string()),
            promoted_property.variable,
        ),
    )
    .with_source(origin, position, position + 3)
    .with_annotation(Annotation::secondary(
        origin,
        promoted_property.initial_position(),
        promoted_property.final_position(),
    ));

    if let Some(class_name) = class_name {
        issue = issue.with_annotation(Annotation::secondary(
            origin,
            class_name.initial_position(),
            class_name.final_position(),
        ));
    }

    issue
}

pub(crate) fn try_statement_must_have_catch_or_finally(
    state: &ParserState,
    try_statement: &TryStatement,
) -> Issue {
    Issue::error(
        ParserIssueCode::TryStatementMustHaveCatchOrFinally,
        "try statement must have a catch or finally block",
    )
    .with_source(
        state.source.name(),
        try_statement.initial_position(),
        try_statement.final_position(),
    )
}

pub(crate) fn cannot_declare_abstract_method_on_non_abstract_class(
    state: &ParserState,
    class_name: &Identifier,
    method: &AbstractMethodDefinition,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::CannotDeclareAbstractMethodOnNonAbstractClass,
        format!(
            "cannot declare abstract method `{}::{}` on a non-abstract class",
            state.named(class_name),
            method.name
        ),
    )
    .with_source(origin, method.initial_position(), method.final_position())
    .with_annotation(Annotation::secondary(
        origin,
        class_name.initial_position(),
        class_name.final_position(),
    ))
    .with_note("abstract methods can only be declared on abstract classes.")
}

pub(crate) fn cannot_declare_abstract_ctor_on_non_abstract_class(
    state: &ParserState,
    class_name: &Identifier,
    method: &AbstractConstructorDefinition,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::CannotDeclareAbstractMethodOnNonAbstractClass,
        format!(
            "cannot declare abstract constructor on non-abstract class `{}`",
            state.named(class_name),
        ),
    )
    .with_source(origin, method.initial_position(), method.final_position())
    .with_annotation(Annotation::secondary(
        origin,
        class_name.initial_position(),
        class_name.final_position(),
    ))
    .with_note("abstract methods can only be declared on abstract classes.")
}

pub(crate) fn unexpected_empty_statement(state: &ParserState, current: &Token) -> Issue {
    Issue::error(
        ParserIssueCode::UnexpectedEmptyStatement,
        "unexpected empty statement",
    )
    .with_source(
        state.source.name(),
        current.position,
        current.position + current.value.len(),
    )
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

pub(crate) fn invalid_constant_expression(state: &ParserState, expression: &Expression) -> Issue {
    Issue::error(
        ParserIssueCode::InvalidConstantExpression,
        "invalid constant expression",
    )
    .with_source(
        state.source.name(),
        expression.initial_position(),
        expression.final_position(),
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

pub(crate) fn invalid_initialization_in_constant_expression(
    state: &ParserState,
    expression: &Expression,
) -> Issue {
    Issue::error(
        ParserIssueCode::InvalidInitializationInConstantExpression,
        "invalid initialization in constant expression",
    )
    .with_source(
        state.source.name(),
        expression.initial_position(),
        expression.final_position(),
    )
    .with_note("constant expressions cannot contain `new` expressions.")
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
