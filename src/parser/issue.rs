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
use crate::tree::definition::modifier::PropertyModifierDefinition;
use crate::tree::definition::property::PropertyEntryDefinition;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#try::TryStatement;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ParserIssueCode {
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

    /// Bottom type cannot be used for parameter ( code = 10 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(never $bar): void {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    BottomTypeCannotBeUsedForParameter = 10,

    /// Bottom type cannot be used for property ( code = 11 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///   public never $bar;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    BottomTypeCannotBeUsedForProperty = 11,

    /// Standalone type cannot be used in union ( code = 12 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(int|void $bar): void {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    StandaloneTypeCannotBeUsedInUnion = 12,

    /// Standalone type cannot be used in intersection ( code = 13 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(int&void $bar): void {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    StandaloneTypeCannotBeUsedInIntersection = 13,

    /// Standalone type cannot be nullable ( code = 14 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(?mixed $bar): void {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    /// - Remove the `?`
    StandaloneTypeCannotBeNullable = 14,

    /// Scalar type cannot be used in intersection ( code = 15 )
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(int&string $bar): void {}
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    /// - Change the intersection to a union
    ScalarTypeCannotBeUsedInIntersection = 15,

    /// Readonly property cannot be static ( code = 17 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///  public readonly static int $bar;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the `static` modifier
    /// - Remove the `readonly` modifier
    ReadonlyPropertyCannotBeStatic = 17,

    /// Readonly property cannot have a default value ( code = 18 )
    ///
    /// Example:
    ///
    /// ```ara
    /// class Foo {
    ///     public readonly int $bar = 1;
    /// }
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Remove the default value
    /// - Remove the `readonly` modifier
    ReadonlyPropertyCannotHaveDefaultValue = 18,

    /// Bottom type cannot be used in tuple type parameter ( code = 19 )
    ///
    /// Example:
    ///
    /// ```ara
    /// type Foo = (void, string);
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Use a different type
    BottomTypeCannotBeUsedInTuple = 19,

    /// Disjunctive normal form types cannot be nested ( code = 20 )
    ///
    /// Example:
    ///
    /// ```ara
    /// type Foo = (A&(B|C))|(D&E);
    /// ```
    ///
    /// Possible solution(s):
    ///
    /// - Refactor the type to not be nested, e.g. `(A&B)|(A&C)|(D&E)`
    DisjunctiveNormalFormTypesCannotBeNested = 20,

    /// Reserved keyword cannot be used for type name ( code = 21 )
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
    ReservedKeywordCannotBeUsedForTypeName = 21,

    /// Reserved keyword cannot be used for constant name ( code = 22 )
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
    ReservedKeywordCannotBeUsedForConstantName = 22,

    /// Type cannot be used in current context ( code = 23 )
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
    TypeCannotBeUsedInCurrentContext = 23,

    /// Missing item expression after attribute(s) ( code = 24 )
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
    MissingItemExpressionAfterAttributes = 24,

    /// Final class cannot be abstract ( code = 25 )
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
    FinalClassCannotBeAbstract = 25,

    /// Final class member cannot be abstract ( code = 26 )
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
    FinalClassMemberCannotBeAbstract = 26,

    /// Private constant cannot be final ( code = 27 )
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
    PrivateConstantCannotBeFinal = 27,

    /// Modifier cannot be used on classes ( code = 28 )
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
    ModifierCannotBeUsedOnClass = 28,

    /// Modifier cannot be used on class methods ( code = 29 )
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
    ModifierCannotBeUsedOnClassMethod = 29,

    /// Modifier cannot be used on interface methods ( code = 30 )
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
    ModifierCannotBeUsedOnInterfaceMethod = 30,

    /// Modifier cannot be used on enum methods ( code = 31 )
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
    ModifierCannotBeUsedOnEnumMethod = 31,

    /// Modifier cannot be used on properties ( code = 32 )
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
    ModifierCannotBeUsedOnProperty = 32,

    /// Modifier cannot be used on promoted properties ( code = 33 )
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
    ModifierCannotBeUsedOnPromotedProperty = 33,

    /// Modifier cannot be used on constants ( code = 34 )
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
    ModifierCannotBeUsedOnConstant = 34,

    /// Modifier cannot be used on interface constants ( code = 35 )
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
    ModifierCannotBeUsedOnInterfaceConstant = 35,

    /// Match expression cannot have multiple default arms ( code = 36 )
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
    MatchExpressionCannotHaveMultipleDefaultArms = 36,

    /// Promoted property cannot be variadic ( code = 37 )
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
    PromotedPropertyCannotBeVariadic = 37,

    /// Enum backing type must be either `int` or `string` ( code = 38 )
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
    InvalidEnumBackingType = 38,

    /// Catch block must have a catch or finally block ( code = 39 )
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
    TryStatementMustHaveCatchOrFinally = 39,

    /// Abstract method cannot be declared on a non-abstract class ( code = 40 )
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
    CannotDeclareAbstractMethodOnNonAbstractClass = 40,

    /// Unexpected empty statement ( code = 41 )
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
    UnexpectedEmptyStatement = 41,

    /// Unexpected token
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo() -> void {
    /// }
    /// ```
    UnexpectedToken = 42,

    /// Invalid constant expression ( code = 43 )
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
    InvalidConstantExpression = 43,

    /// Invalid constant initialization expression ( code = 44 )
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
    InvalidConstantInitializationExpression = 44,

    /// Invalid constant initialization expression ( code = 45 )
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
    InvalidInitializationInConstantExpression = 45,

    /// Invalid empty type template ( code = 46 )
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
    ExpectedAtLeastOneTypeInTemplateGroup = 46,
}

pub(crate) fn php_opening_tag_not_supported(state: &ParserState, token: &Token) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::PHPOpeningTagNotSupported,
        format!("PHP opening tag `{}` is not supported", token.value),
        origin,
        token.position,
        token.position + token.value.len(),
    )
    .with_help("try removing the opening tag")
}

pub(crate) fn php_closing_tag_not_supported(state: &ParserState, token: &Token) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::PHPClosingTagNotSupported,
        format!("PHP closing tag `{}` is not supported", token.value),
        origin,
        token.position,
        token.position + token.value.len(),
    )
    .with_help("try removing the closing tag")
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
        origin,
        case.initial_position(),
        semicolon + 1,
    )
    .with_annotation(Annotation::new(
        origin,
        r#enum.initial_position(),
        r#enum.final_position(),
    ))
    .with_annotation(Annotation::new(
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
        origin,
        case.initial_position(),
        semicolon + 1,
    )
    .with_annotation(Annotation::new(
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
        origin,
        constructor.initial_position(),
        constructor.final_position(),
    )
    .with_annotation(Annotation::new(
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
        origin,
        method.initial_position(),
        method.final_position(),
    )
    .with_annotation(Annotation::new(
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
        origin,
        current.position,
        current.position + current.value.len(),
    )
    .with_help("try adding an item definition after the attribute(s).")
    .with_note("an item definition can be a class, an interface, an enum, or a function.");

    for attribute in &state.attributes {
        issue = issue.with_annotation(Annotation::new(
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
        origin,
        second.0,
        second.0 + second.1.len(),
    )
    .with_annotation(Annotation::new(origin, first.0, first.0 + first.1.len()))
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
        origin,
        second,
        second + modifier.len(),
    )
    .with_annotation(Annotation::new(origin, first, first + modifier.len()))
}

pub(crate) fn bottom_type_cannot_be_used_for_parameter(
    state: &ParserState,
    type_definition: &TypeDefinition,
    parameter: &Variable,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::BottomTypeCannotBeUsedForParameter,
        format!(
            "bottom type `{}` cannot be used for parameter `{}`",
            &type_definition, &parameter,
        ),
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        parameter.initial_position(),
        parameter.final_position(),
    ))
    .with_note("bottom types are types that have no values, such as `never` and `void`.")
    .with_help("try using a different type for the parameter.")
}

pub(crate) fn bottom_type_cannot_be_used_for_property(
    state: &ParserState,
    class_name: Option<&Identifier>,
    type_definition: &TypeDefinition,
    property: &Variable,
) -> Issue {
    let origin = state.source.name();

    let mut issue = Issue::error(
        ParserIssueCode::BottomTypeCannotBeUsedForProperty,
        format!(
            "bottom type `{}` cannot be used for property `{}::{}`",
            &type_definition,
            class_name
                .map(|c| state.named(c))
                .unwrap_or_else(|| "anonymous@class".to_string()),
            &property,
        ),
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        property.initial_position(),
        property.final_position(),
    ))
    .with_note("bottom types are types that have no values, such as `never` and `void`.")
    .with_help("try using a different type for the property.");

    if let Some(class_name) = class_name {
        issue = issue.with_annotation(Annotation::new(
            origin,
            class_name.initial_position(),
            class_name.final_position(),
        ));
    }

    issue
}

pub(crate) fn standalone_type_cannot_be_used_in_union(
    state: &ParserState,
    type_definition: &TypeDefinition,
    pipe: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::StandaloneTypeCannotBeUsedInUnion,
        format!(
            "standalone type `{}` cannot be used in a union",
            &type_definition,
        ),
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_annotation(Annotation::new(origin, pipe, pipe + 1))
    .with_note("a standalone type is either `mixed`, `void`, `never`, `resource`, `nonnull` or a nullable type.")
    .with_help("try using a different type for the union.")
}

pub(crate) fn standalone_type_cannot_be_used_in_intersection(
    state: &ParserState,
    type_definition: &TypeDefinition,
    ampersand: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::StandaloneTypeCannotBeUsedInIntersection,
        format!(
            "standalone type `{}` cannot be used in an intersection",
            &type_definition,
        ),
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        ampersand,
        ampersand + 1,
    ))
    .with_note("a standalone type is either `mixed`, `void`, `never`, `resource`, `nonnull` or a nullable type.")
    .with_help("try using a different type for the intersection.")
}

pub(crate) fn standalone_type_cannot_be_nullable(
    state: &ParserState,
    type_definition: &TypeDefinition,
    question_mark: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::StandaloneTypeCannotBeNullable,
        format!("standalone type `{}` cannot be nullable", &type_definition,),
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        question_mark,
        question_mark + 1,
    ))
    .with_note("a standalone type is either `mixed`, `void`, `never`, `resource`, `nonnull` or a nullable type.")
    .with_help("try removing `?`.")
}

pub(crate) fn scalar_type_cannot_be_used_in_intersection(
    state: &ParserState,
    type_definition: &TypeDefinition,
    ampersand: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ScalarTypeCannotBeUsedInIntersection,
        format!(
            "scalar type `{}` cannot be used in an intersection",
            &type_definition,
        ),
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_annotation(Annotation::new(origin, ampersand, ampersand + 1))
    .with_note("a scalar type is either `int`, `float`, `string`, or `bool`.")
    .with_help("try using a different type for the intersection.")
}

pub(crate) fn readonly_property_cannot_be_static(
    state: &ParserState,
    class_name: Option<&Identifier>,
    property: &Variable,
    readonly: &PropertyModifierDefinition,
    r#static: &PropertyModifierDefinition,
) -> Issue {
    let origin = state.source.name();

    let mut issue = Issue::error(
        ParserIssueCode::ReadonlyPropertyCannotBeStatic,
        format!(
            "readonly property `{}::{}` cannot be static",
            class_name
                .map(|c| state.named(c))
                .unwrap_or_else(|| "anonymous@class".to_string()),
            &property,
        ),
        origin,
        r#static.initial_position(),
        r#static.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        readonly.initial_position(),
        readonly.final_position(),
    ))
    .with_annotation(Annotation::new(
        origin,
        property.initial_position(),
        property.final_position(),
    ))
    .with_note("a property cannot be both readonly and static.")
    .with_help("try removing `static`.");

    if let Some(class_name) = class_name {
        issue = issue.with_annotation(Annotation::new(
            origin,
            class_name.initial_position(),
            class_name.final_position(),
        ));
    }

    issue
}

pub(crate) fn readonly_property_cannot_have_default_value(
    state: &ParserState,
    class_name: Option<&Identifier>,
    entry: &PropertyEntryDefinition,
    readonly: &PropertyModifierDefinition,
) -> Issue {
    let origin = state.source.name();

    let mut issue = Issue::error(
        ParserIssueCode::ReadonlyPropertyCannotHaveDefaultValue,
        format!(
            "readonly property `{}::{}` cannot have a default value",
            class_name
                .map(|c| state.named(c))
                .unwrap_or_else(|| "anonymous@class".to_string()),
            &entry.variable(),
        ),
        origin,
        entry.initial_position(),
        entry.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        readonly.initial_position(),
        readonly.final_position(),
    ))
    .with_note(
        "a readonly property cannot have a default value because it cannot be changed after initialization.",
    )
    .with_help("try removing the default value.");

    if let Some(class_name) = class_name {
        issue = issue.with_annotation(Annotation::new(
            origin,
            class_name.initial_position(),
            class_name.final_position(),
        ));
    }

    issue
}

pub(crate) fn bottom_type_cannot_be_used_in_tuple(
    state: &ParserState,
    type_definition: &TypeDefinition,
    left_parenthesis: usize,
    right_parenthesis: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::BottomTypeCannotBeUsedInTuple,
        format!(
            "bottom type `{}` cannot be used in a tuple type",
            &type_definition,
        ),
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        left_parenthesis,
        right_parenthesis + 1,
    ))
    .with_note("bottom types are types that have no values, such as `never` and `void`.")
    .with_help("try using a different type for the tuple.")
}

pub(crate) fn disjunctive_normal_form_types_cannot_be_nested(
    state: &ParserState,
    type_definition: &TypeDefinition,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::DisjunctiveNormalFormTypesCannotBeNested,
        "cannot nest disjunctive normal form types",
        origin,
        type_definition.initial_position(),
        type_definition.final_position(),
    )
    .with_note(
        "disjunctive normal form types are types that are separated by `|`, or `&` and are enclosed in `(` and `)`.",
    )
    .with_help("try removing the parenthesis.")
}

pub(crate) fn reserved_keyword_cannot_be_used_for_type_name(
    state: &ParserState,
    identifier: &Identifier,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ReservedKeywordCannotBeUsedForTypeName,
        format!(
            "reserved keyword `{}` cannot be used as a type name",
            identifier,
        ),
        origin,
        identifier.initial_position(),
        identifier.final_position(),
    )
    .with_help("try using a different name for the type.")
}

pub(crate) fn reserved_keyword_cannot_be_used_for_constant_name(
    state: &ParserState,
    identifier: &Identifier,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ReservedKeywordCannotBeUsedForConstantName,
        format!(
            "reserved keyword `{}` cannot be used as a constant name",
            identifier,
        ),
        origin,
        identifier.initial_position(),
        identifier.final_position(),
    )
    .with_help("try using a different name for the constant.")
}

pub(crate) fn type_cannot_be_used_in_current_context(
    state: &ParserState,
    identifier: &Identifier,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::TypeCannotBeUsedInCurrentContext,
        format!(
            "type `{}` cannot be used in the current context",
            identifier,
        ),
        origin,
        identifier.initial_position(),
        identifier.final_position(),
    )
    .with_help("try using a different type.")
}

pub(crate) fn missing_item_expression_after_attributes(state: &ParserState) -> Issue {
    let origin = state.source.name();
    let current = state.iterator.current();

    let mut issue = Issue::error(
        ParserIssueCode::MissingItemExpressionAfterAttributes,
        "missing item expression after attribute(s)",
        origin,
        current.position,
        current.position + current.value.len(),
    )
    .with_help("try adding an item expression after the attribute(s)")
    .with_note("an item expression can be an anonymous function, or an arrow function.");

    for attribute in &state.attributes {
        issue = issue.with_annotation(Annotation::new(
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
        origin,
        r#abstract.initial_position(),
        r#abstract.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        r#final.initial_position(),
        r#final.final_position(),
    ))
    .with_note("a final class cannot be abstract because it cannot be extended by other classes.")
    .with_help("try removing the `abstract` modifier.")
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
        origin,
        r#abstract.initial_position(),
        r#abstract.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        r#final.initial_position(),
        r#final.final_position(),
    ))
    .with_note(
        "a final class member cannot be abstract because it cannot be overridden by other classes.",
    )
    .with_help("try removing the `abstract` modifier.")
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
        origin,
        r#final.initial_position(),
        r#final.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        r#private.initial_position(),
        r#private.final_position(),
    ))
    .with_note(
        "a private constant cannot be final because it cannot be overridden by other classes.",
    )
    .with_help("try removing the `final` modifier.")
}

pub(crate) fn modifier_cannot_be_used_on_class(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnClass,
        format!("modifier `{}` cannot be used on a class", modifier),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
    .with_note("only the `final`, `abstract`, and `readonly` modifiers can be used on a class.")
}

pub(crate) fn modifier_cannot_be_used_on_class_method(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnClassMethod,
        format!("modifier `{}` cannot be used on a class method", modifier),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
    .with_note(
        "only the `final`, `abstract`, `static`, `private`, `protected`, and `public` modifiers can be used on a class method.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_interface_method(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnInterfaceMethod,
        format!(
            "modifier `{}` cannot be used on an interface method",
            modifier
        ),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
    .with_note("only the `static`, and `public` modifiers can be used on an interface method.")
}

pub(crate) fn modifier_cannot_be_used_on_enum_method(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnEnumMethod,
        format!("modifier `{}` cannot be used on an enum method", modifier),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
    .with_note("only the `final`, `static`, and `public`, `protected`, `private` modifiers can be used on an enum method.")
}

pub(crate) fn modifier_cannot_be_used_on_property(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnProperty,
        format!("modifier `{}` cannot be used on a property", modifier),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
    .with_note(
        "only the `static`, `readonly`, `private`, `protected`, and `public` modifiers can be used on a property.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_promoted_property(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnPromotedProperty,
        format!("modifier `{}` cannot be used on a promoted property", modifier),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
    .with_note(
        "only the `readonly`, `private`, `protected`, and `public` modifiers can be used on a promoted property.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_constant(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnConstant,
        format!("modifier `{}` cannot be used on a constant", modifier),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
    .with_note(
        "only the `final`, `private`, `protected`, and `public` modifiers can be used on a constant.",
    )
}

pub(crate) fn modifier_cannot_be_used_on_interface_constant(
    state: &ParserState,
    modifier: String,
    position: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ModifierCannotBeUsedOnInterfaceConstant,
        format!(
            "modifier `{}` cannot be used on an interface constant",
            modifier
        ),
        origin,
        position,
        position + modifier.len(),
    )
    .with_help("try removing the modifier.")
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
        origin,
        second.initial_position(),
        second.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        first.initial_position(),
        first.final_position(),
    ))
    .with_help("try removing one of the default arms.")
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
        origin,
        position,
        position + 3,
    )
    .with_annotation(Annotation::new(
        origin,
        promoted_property.initial_position(),
        promoted_property.final_position(),
    ))
    .with_help("try removing the variadic declaration.");

    if let Some(class_name) = class_name {
        issue = issue.with_annotation(Annotation::new(
            origin,
            class_name.initial_position(),
            class_name.final_position(),
        ));
    }

    issue
}

pub(crate) fn invalid_enum_backing_type(
    state: &ParserState,
    backing_identifier: &Identifier,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::InvalidEnumBackingType,
        format!("invalid enum backing type `{}`", backing_identifier),
        origin,
        backing_identifier.initial_position(),
        backing_identifier.final_position(),
    )
    .with_help("try using a valid enum backing type.")
    .with_note("the only valid enum backing types are `int`, and `string`.")
}

pub(crate) fn try_statement_must_have_catch_or_finally(
    state: &ParserState,
    try_statement: &TryStatement,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::TryStatementMustHaveCatchOrFinally,
        "try statement must have a catch or finally block",
        origin,
        try_statement.initial_position(),
        try_statement.final_position(),
    )
    .with_help("try adding a catch or finally block.")
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
        origin,
        method.initial_position(),
        method.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        class_name.initial_position(),
        class_name.final_position(),
    ))
    .with_help("try removing the abstract modifier.")
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
        origin,
        method.initial_position(),
        method.final_position(),
    )
    .with_annotation(Annotation::new(
        origin,
        class_name.initial_position(),
        class_name.final_position(),
    ))
    .with_help("try removing the abstract modifier.")
    .with_note("abstract methods can only be declared on abstract classes.")
}

pub(crate) fn unexpected_empty_statement(state: &ParserState, current: &Token) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::UnexpectedEmptyStatement,
        "unexpected empty statement",
        origin,
        current.position,
        current.position + current.value.len(),
    )
    .with_help("try removing the `;`")
}

pub(crate) fn unexpected_token<T: ToString>(
    state: &ParserState,
    expected: Vec<T>,
    found: &Token,
) -> Issue {
    let origin = state.source.name();

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

    Issue::error(
        ParserIssueCode::UnexpectedToken,
        message,
        origin,
        found.position,
        found.position + found.value.len(),
    )
}

pub(crate) fn invalid_constant_expression(state: &ParserState, expression: &Expression) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::InvalidConstantExpression,
        "invalid constant expression",
        origin,
        expression.initial_position(),
        expression.final_position(),
    )
}

pub(crate) fn invalid_constant_initialization_expression(
    state: &ParserState,
    expression: &dyn Node,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::InvalidConstantInitializationExpression,
        "invalid constant initialization expression",
        origin,
        expression.initial_position(),
        expression.final_position(),
    )
}

pub(crate) fn invalid_initialization_in_constant_expression(
    state: &ParserState,
    expression: &Expression,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::InvalidInitializationInConstantExpression,
        "invalid initialization in constant expression",
        origin,
        expression.initial_position(),
        expression.final_position(),
    )
    .with_note("constant expressions cannot contain `new` expressions.")
    .with_help("try removing the `new` expression.")
}

pub(crate) fn expected_at_least_one_type_in_template_group(
    state: &ParserState,
    less_than: usize,
    greater_than: usize,
) -> Issue {
    let origin = state.source.name();

    Issue::error(
        ParserIssueCode::ExpectedAtLeastOneTypeInTemplateGroup,
        "expected at least one type in template group",
        origin,
        less_than,
        greater_than,
    )
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
