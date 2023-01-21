use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::expression::precedence::Associativity;
use crate::parser::internal::expression::precedence::Precedence;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::internal::variable;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::array::TupleExpression;
use crate::tree::expression::construct::ExitConstructExpression;
use crate::tree::expression::literal::Literal;
use crate::tree::expression::literal::LiteralFalse;
use crate::tree::expression::literal::LiteralFloat;
use crate::tree::expression::literal::LiteralInteger;
use crate::tree::expression::literal::LiteralNull;
use crate::tree::expression::literal::LiteralString;
use crate::tree::expression::literal::LiteralTrue;
use crate::tree::expression::magic_constant::MagicConstant;
use crate::tree::expression::operator::ArithmeticOperationExpression;
use crate::tree::expression::operator::ArrayOperationExpression;
use crate::tree::expression::operator::AsyncOperationExpression;
use crate::tree::expression::operator::BitwiseOperationExpression;
use crate::tree::expression::operator::ClassOperationExpression;
use crate::tree::expression::operator::ClassOperationInitializationClassExpression;
use crate::tree::expression::operator::ExceptionOperationExpression;
use crate::tree::expression::operator::FunctionalOperationExpression;
use crate::tree::expression::operator::GeneratorOperationExpression;
use crate::tree::expression::operator::LogicalOperationExpression;
use crate::tree::expression::operator::ObjectOperationExpression;
use crate::tree::expression::operator::RangeOperationExpression;
use crate::tree::expression::Expression;
use crate::tree::expression::ParenthesizedExpression;
use crate::tree::identifier::Identifier;

pub mod argument;
pub mod array;
pub mod class;
pub mod control_flow;
pub mod function;
pub mod generic;
pub mod infix;
pub mod postfix;
pub mod precedence;

pub fn create(state: &mut State) -> ParseResult<Expression> {
    for_precedence(state, Precedence::Lowest)
}

pub fn null_coalesce_precedence(state: &mut State) -> ParseResult<Expression> {
    for_precedence(state, Precedence::NullCoalesce)
}

pub fn for_precedence(state: &mut State, precedence: Precedence) -> ParseResult<Expression> {
    let mut left = left(state, &precedence)?;

    loop {
        let current = state.iterator.current();
        let kind = &current.kind;

        if matches!(current.kind, TokenKind::SemiColon | TokenKind::Eof) {
            break;
        }

        if postfix::is_postfix(kind) {
            let left_precedence = Precedence::postfix(state, kind)?;

            if left_precedence < precedence {
                break;
            }

            left = postfix::postfix(state, left, kind)?;

            continue;
        }

        if infix::is_infix(state, kind) {
            let right_precedence = Precedence::infix(state, kind)?;

            if right_precedence < precedence {
                break;
            }

            if right_precedence == precedence
                && matches!(right_precedence.associativity(), Some(Associativity::Left))
            {
                break;
            }

            if right_precedence == precedence
                && matches!(right_precedence.associativity(), Some(Associativity::Non))
            {
                let expected: Vec<String> = vec![];
                crate::parser_bail!(state, unexpected_token(expected, current));
            }

            left = infix::infix(state, left, kind, right_precedence)?;

            continue;
        }

        break;
    }

    Ok(left)
}

pub fn left(state: &mut State, precedence: &Precedence) -> ParseResult<Expression> {
    if state.iterator.is_eof() {
        crate::parser_bail!(
            state,
            unexpected_token(vec!["an expression"], state.iterator.current())
        );
    }

    attributes(state, precedence)
}

macro_rules! expressions {
    (
        using($state:ident):

        $(
            #[before($else:ident), $(precedence($precedence:expr),)? current($(|)? $( $current:pat_param )|+) $(, peek($(|)? $( $peek:pat_param )|+))?]
            $expr:ident($out:tt)
        )+
    ) => {
        $(
            pub fn $expr($state: &mut State, precedence: &Precedence) -> ParseResult<Expression> {
                $(
                    if &$precedence < precedence {
                        return $else($state, precedence);
                    }
                )?

                match &$state.iterator.current().kind {
                    $( $current )|+ $( if matches!(&$state.iterator.lookahead(1).kind, $( $peek )|+ ))? => $out,
                    _ => $else($state, precedence),
                }
            }
        )+
    };
}

expressions! {
    using(state):

    #[before(functional_expression), current(TokenKind::Attribute)]
    attributes({
        attribute::gather(state)?;

        let current = state.iterator.current();

        match &current.kind {
            TokenKind::Static if state.iterator.lookahead(1).kind == TokenKind::Function => {
                Ok(Expression::AnonymousFunction(function::anonymous_function_expression(state)?))
            }
            TokenKind::Static if state.iterator.lookahead(1).kind == TokenKind::Fn => {
                Ok(Expression::ArrowFunction(function::arrow_function_expression(state)?))
            }
            TokenKind::Function => {
                Ok(Expression::AnonymousFunction(function::anonymous_function_expression(state)?))
            }
            TokenKind::Fn => {
                Ok(Expression::ArrowFunction(function::arrow_function_expression(state)?))
            }
            _ => {
                crate::parser_report!(state, missing_item_expression_after_attributes);

                crate::parser_bail!(
                    state,
                    unexpected_token(vec!["static", "fn", "function"], current)
                );
            }
        }
    })

    #[before(static_arrow_function), current(TokenKind::Dollar), peek(TokenKind::Generic | TokenKind::LeftParen)]
    functional_expression({
        let dollar = state.iterator.current().position;
        state.iterator.next();
        Ok(Expression::FunctionalOperation(FunctionalOperationExpression::Expression {
            comments: state.iterator.comments(),
            dollar,
            generics: if state.iterator.current().kind == TokenKind::Generic {
                Some(generic::generic_group(state)?)
            } else {
                None
            },
            left_parenthesis: utils::skip_left_parenthesis(state)?,
            expression: Box::new(create(state)?),
            right_parenthesis: utils::skip_right_parenthesis(state)?,
        }))
    })

    #[before(static_anonymous_function), current(TokenKind::Static), peek(TokenKind::Fn)]
    static_arrow_function({
        Ok(Expression::ArrowFunction(function::arrow_function_expression(state)?))
    })

    #[before(arrow_function), current(TokenKind::Static), peek(TokenKind::Function)]
    static_anonymous_function({
        Ok(Expression::AnonymousFunction(function::anonymous_function_expression(state)?))
    })

    #[before(anonymous_function), current(TokenKind::Fn)]
    arrow_function({
        Ok(Expression::ArrowFunction(function::arrow_function_expression(state)?))
    })

    #[before(exit), current(TokenKind::Function)]
    anonymous_function({
        Ok(Expression::AnonymousFunction(function::anonymous_function_expression(state)?))
    })

    #[before(reserved_identifier_function_call), current(TokenKind::Exit)]
    exit({
        let exit = utils::skip_keyword(state, TokenKind::Exit)?;
        if state.iterator.current().kind == TokenKind::LeftParen {
            Ok(Expression::ExitConstruct(ExitConstructExpression::ExitWith {
                comments: state.iterator.comments(),
                exit,
                left_parenthesis: utils::skip_left_parenthesis(state)?,
                value: if state.iterator.current().kind == TokenKind::RightParen {
                    None
                } else {
                    Some(Box::new(create(state)?))
                },
                right_parenthesis: utils::skip_right_parenthesis(state)?,
            }))
        } else {
            Ok(Expression::ExitConstruct(ExitConstructExpression::Exit {
                comments: state.iterator.comments(),
                exit,
            }))
        }
    })

    #[before(reserved_identifier_static_call), precedence(Precedence::CallDim), current(
        | TokenKind::True       | TokenKind::False | TokenKind::Null
        | TokenKind::Readonly   | TokenKind::Self_ | TokenKind::Parent
        | TokenKind::Enum       | TokenKind::From  | TokenKind::Where
        | TokenKind::Type       | TokenKind::Dict  | TokenKind::Vec
        | TokenKind::Async      | TokenKind::Await | TokenKind::Concurrently
        | TokenKind::Is         | TokenKind::In    | TokenKind::Into
        | TokenKind::Using
    ), peek(TokenKind::LeftParen | TokenKind::Generic)]
    reserved_identifier_function_call({
        let ident = identifier::identifier_maybe_soft_reserved(state)?;
        let lhs = Expression::Identifier(ident);
        let op = &state.iterator.current().kind;

        postfix::postfix(state, lhs, op)
    })

    #[before(isset), current(
        | TokenKind::Enum   | TokenKind::From   | TokenKind::Where
        | TokenKind::Type   | TokenKind::Vec    | TokenKind::Dict
        | TokenKind::Async  | TokenKind::Await  | TokenKind::Concurrently
        | TokenKind::Is     | TokenKind::In     | TokenKind::Into
        | TokenKind::Using
    ), peek(TokenKind::DoubleColon)]
    reserved_identifier_static_call({
        let ident = identifier::classname_identifier(state)?;
        let lhs = Expression::Identifier(ident);

        postfix::postfix(state, lhs, &TokenKind::DoubleColon)
    })

    #[before(unset), current(TokenKind::Isset)]
    isset({
        Ok(Expression::ArrayOperation(ArrayOperationExpression::Isset {
            comments: state.iterator.comments(),
            isset: utils::skip_keyword(state, TokenKind::Isset)?,
            item: Box::new(for_precedence(state, Precedence::Lowest)?)
        }))
    })

    #[before(anonymous_class), current(TokenKind::Unset)]
    unset({
        Ok(Expression::ArrayOperation(ArrayOperationExpression::Unset {
            comments: state.iterator.comments(),
            unset: utils::skip_keyword(state, TokenKind::Unset)?,
            item: Box::new(for_precedence(state, Precedence::Lowest)?)
        }))
    })

    #[before(new), current(TokenKind::New), peek(TokenKind::Class | TokenKind::Attribute)]
    anonymous_class({
        Ok(Expression::ClassOperation(
            class::anonymous_initialization_class_operation_expression(state)?,
        ))
    })

    #[before(throw), current(TokenKind::New)]
    new({
        Ok(Expression::ClassOperation(
            ClassOperationExpression::Initialization {
                comments: state.iterator.comments(),
                new: utils::skip_keyword(state, TokenKind::New)?,
                class: match state.iterator.current().kind {
                    TokenKind::Variable => ClassOperationInitializationClassExpression::Variable(
                        variable::parse(state)?
                    ),
                    _ => ClassOperationInitializationClassExpression::Identifier(
                        identifier::fully_qualified_type_identifier_including_self(state)?
                    )
                },
                generics: if state.iterator.current().kind == TokenKind::Generic {
                    Some(generic::generic_group(state)?)
                } else {
                    None
                },
                arguments: argument::argument_list_expression(state)?
            }
        ))
    })

    #[before(r#async), current(TokenKind::Throw)]
    throw({
        Ok(Expression::ExceptionOperation(ExceptionOperationExpression::Throw {
            comments: state.iterator.comments(),
            throw: utils::skip_keyword(state, TokenKind::Throw)?,
            value: Box::new(for_precedence(state, Precedence::Lowest)?)
        }))
    })

    #[before(r#await), current(TokenKind::Async)]
    r#async({
        Ok(Expression::AsyncOperation(AsyncOperationExpression::Async {
            comments: state.iterator.comments(),
            r#async: utils::skip_keyword(state, TokenKind::Async)?,
            expression: Box::new(for_precedence(state, Precedence::Lowest)?)
        }))
    })

    #[before(concurrently), current(TokenKind::Await)]
    r#await({
        Ok(Expression::AsyncOperation(AsyncOperationExpression::Await {
            comments: state.iterator.comments(),
            r#await: utils::skip_keyword(state, TokenKind::Await)?,
            expression: Box::new(for_precedence(state, Precedence::Lowest)?)
        }))
    })

    #[before(r#yield), current(TokenKind::Concurrently)]
    concurrently({
        Ok(Expression::AsyncOperation(AsyncOperationExpression::Concurrently {
            comments: state.iterator.comments(),
            concurrently: utils::skip_keyword(state, TokenKind::Concurrently)?,
            left_brace: utils::skip_left_brace(state)?,
            expressions: utils::comma_separated(state, &create, TokenKind::RightBrace)?,
            right_brace: utils::skip_right_brace(state)?,
        }))
    })

    #[before(clone), current(TokenKind::Yield)]
    r#yield({
        let r#yield = utils::skip_keyword(state, TokenKind::Yield)?;
        let comments = state.iterator.comments();
        let current = state.iterator.current();
        if current.kind == TokenKind::SemiColon || current.kind == TokenKind::RightParen {
            Ok(Expression::GeneratorOperation(GeneratorOperationExpression::Yield {
                comments,
                r#yield,
            }))
        } else if current.kind == TokenKind::From  {
            Ok(Expression::GeneratorOperation(GeneratorOperationExpression::YieldFrom {
                comments,
                r#yield,
                from: utils::skip_keyword(state, TokenKind::From)?,
                value: Box::new(for_precedence(
                    state,
                    Precedence::YieldFrom,
                )?)
            }))
        } else {
            let mut value = Box::new(for_precedence(
                state,
                Precedence::Yield
            )?);

            let current = state.iterator.current();
            if current.kind == TokenKind::DoubleArrow {
                state.iterator.next();

                let mut key = Box::new(for_precedence(state, Precedence::Yield)?);

                std::mem::swap(&mut key, &mut value);

                Ok(Expression::GeneratorOperation(GeneratorOperationExpression::YieldKeyValue {
                    comments,
                    r#yield,
                    key,
                    double_arrow: current.position,
                    value,
                }))
            } else {
                Ok(Expression::GeneratorOperation(GeneratorOperationExpression::YieldValue {
                    comments,
                    r#yield,
                    value,
                }))
            }
        }
    })

    #[before(r#true), current(TokenKind::Clone)]
    clone({
        Ok(Expression::ObjectOperation(ObjectOperationExpression::Clone {
            comments: state.iterator.comments(),
            clone: utils::skip_keyword(state, TokenKind::Clone)?,
            object: Box::new(for_precedence(state, Precedence::Clone)?),
        }))
    })

    #[before(r#false), current(TokenKind::True)]
    r#true({
        Ok(Expression::Literal(Literal::True(
            LiteralTrue {
                comments: state.iterator.comments(),
                r#true: utils::skip_keyword(state, TokenKind::True)?,
            }
        )))
    })

    #[before(null), current(TokenKind::False)]
    r#false({
        Ok(Expression::Literal(Literal::False(
            LiteralFalse {
                comments: state.iterator.comments(),
                r#false: utils::skip_keyword(state, TokenKind::False)?,
            }
        )))
    })

    #[before(literal_integer), current(TokenKind::Null)]
    null({
        Ok(Expression::Literal(Literal::Null(
            LiteralNull {
                comments: state.iterator.comments(),
                null: utils::skip_keyword(state, TokenKind::Null)?,
            }
        )))
    })

    #[before(literal_float), current(TokenKind::LiteralInteger)]
    literal_integer({
        let current = state.iterator.current();
        state.iterator.next();

        Ok(Expression::Literal(Literal::Integer(
            LiteralInteger {
                comments: state.iterator.comments(),
                position: current.position,
                value: current.value.clone()
            }
        )))
    })

    #[before(literal_string), current(TokenKind::LiteralFloat)]
    literal_float({
        let current = state.iterator.current();
        state.iterator.next();

        Ok(Expression::Literal(
            Literal::Float(LiteralFloat {
                comments: state.iterator.comments(),
                position: current.position,
                value: current.value.clone()
            })
        ))
    })

    #[before(dict), current(TokenKind::LiteralString)]
    literal_string({
        let current = state.iterator.current();
        state.iterator.next();

        Ok(Expression::Literal(
            Literal::String(LiteralString {
                comments: state.iterator.comments(),
                position: current.position,
                value: current.value.clone()
            })
        ))
    })

    #[before(vec), current(TokenKind::Dict), peek(TokenKind::LeftBracket)]
    dict({
        Ok(Expression::Dict(array::dict_expression(state)?))
    })

    #[before(identifier), current(TokenKind::Vec), peek(TokenKind::LeftBracket)]
    vec({
        Ok(Expression::Vec(array::vec_expression(state)?))
    })

    #[before(reserved_identifier), current(TokenKind::Identifier | TokenKind::QualifiedIdentifier | TokenKind::FullyQualifiedIdentifier)]
    identifier({
        Ok(Expression::Identifier(identifier::fully_qualified_type_identifier(state)?))
    })

    #[before(left_parenthesis), current(
        | TokenKind::Self_  | TokenKind::Parent | TokenKind::Static
        | TokenKind::From   | TokenKind::Enum   | TokenKind::Where
        | TokenKind::Type   | TokenKind::Vec    | TokenKind::Dict
        | TokenKind::Async  | TokenKind::Await  | TokenKind::Concurrently
        | TokenKind::Is     | TokenKind::In     | TokenKind::Into
        | TokenKind::Using
    )]
    reserved_identifier({
        let current = state.iterator.current();

        state.iterator.next();

        Ok(Expression::Identifier(Identifier {
            position: current.position,
            value: current.value.clone(),
        }))
    })

    #[before(r#match), current(TokenKind::LeftParen)]
    left_parenthesis({
        let comments = state.iterator.comments();
        let left_parenthesis = state.iterator.current().position;
        state.iterator.next();
        let expression = create(state)?;

        let current = state.iterator.current();
        if current.kind == TokenKind::RightParen {
            state.iterator.next();

            Ok(Expression::Parenthesized(ParenthesizedExpression {
                comments,
                left_parenthesis,
                expression: Box::new(expression),
                right_parenthesis: current.position,
            }))
        } else {
            let comma = utils::skip(state, TokenKind::Comma)?;

            let elements = {
                let expressions = [expression];
                let commas = [comma];

                let mut elements = utils::at_least_one_comma_separated(state, &create, TokenKind::RightParen)?;

                elements.inner = [expressions.as_slice(), elements.inner.as_slice()].concat();
                elements.commas = [commas.as_slice(), elements.commas.as_slice()].concat();

                elements
            };

            let right_parenthesis = utils::skip_right_parenthesis(state)?;

            Ok(Expression::Tuple(TupleExpression {
                comments,
                left_parenthesis,
                elements,
                right_parenthesis,
            }))
        }
    })

    #[before(directory_magic_constant), current(TokenKind::Match)]
    r#match({
        Ok(Expression::Match(control_flow::match_expression(state)?))
    })

    #[before(file_magic_constant), current(TokenKind::DirConstant)]
    directory_magic_constant({
        let current = state.iterator.current();
        let position = current.position;
        let value = current.value.clone();

        state.iterator.next();

        Ok(Expression::MagicConstant(MagicConstant::Directory {
            position,
            value,
        }))
    })

    #[before(line_magic_constant), current(TokenKind::FileConstant)]
    file_magic_constant({
        let current = state.iterator.current();
        let position = current.position;
        let value = current.value.clone();

        state.iterator.next();

        Ok(Expression::MagicConstant(MagicConstant::File {
            position,
            value,
        }))
    })

    #[before(function_magic_constant), current(TokenKind::LineConstant)]
    line_magic_constant({
        let current = state.iterator.current();
        let position = current.position;
        let value = current.value.clone();

        state.iterator.next();

        Ok(Expression::MagicConstant(MagicConstant::Line {
            position,
            value,
        }))
    })

    #[before(class_magic_constant), current(TokenKind::FunctionConstant)]
    function_magic_constant({
        let current = state.iterator.current();
        let position = current.position;
        let value = current.value.clone();

        state.iterator.next();

        Ok(Expression::MagicConstant(MagicConstant::Function {
            position,
            value,
        }))
    })

    #[before(method_magic_constant), current(TokenKind::ClassConstant)]
    class_magic_constant({
        let current = state.iterator.current();
        let position = current.position;
        let value = current.value.clone();

        state.iterator.next();

        Ok(Expression::MagicConstant(MagicConstant::Class {
            position,
            value,
        }))
    })

    #[before(namespace_magic_constant), current(TokenKind::MethodConstant)]
    method_magic_constant({
        let current = state.iterator.current();
        let position = current.position;
        let value = current.value.clone();

        state.iterator.next();

        Ok(Expression::MagicConstant(MagicConstant::Method {
            position,
            value,
        }))
    })

    #[before(numeric_prefix), current(TokenKind::NamespaceConstant)]
    namespace_magic_constant({
        let current = state.iterator.current();
        let position = current.position;
        let value = current.value.clone();

        state.iterator.next();

        Ok(Expression::MagicConstant(MagicConstant::Namespace {
            position,
            value,
        }))
    })

    #[before(bang_prefix), current(TokenKind::Decrement | TokenKind::Increment | TokenKind::Minus | TokenKind::Plus)]
    numeric_prefix({
        let current = state.iterator.current();

        let position = current.position;
        let op = current.kind.clone();

        state.iterator.next();

        let expr = match op {
            TokenKind::Minus => Expression::ArithmeticOperation(ArithmeticOperationExpression::Negative {
                comments: state.iterator.comments(),
                minus: position,
                right: Box::new(for_precedence(state, Precedence::Prefix)?),
            }),
            TokenKind::Plus => Expression::ArithmeticOperation(ArithmeticOperationExpression::Positive {
                comments: state.iterator.comments(),
                plus: position,
                right: Box::new(for_precedence(state, Precedence::Prefix)?)
            }),
            TokenKind::Decrement => Expression::ArithmeticOperation(ArithmeticOperationExpression::PreDecrement {
                comments: state.iterator.comments(),
                decrement: position,
                right: Box::new(for_precedence(state, Precedence::Prefix)?)
            }),
            TokenKind::Increment => Expression::ArithmeticOperation(ArithmeticOperationExpression::PreIncrement {
                comments: state.iterator.comments(),
                increment: position,
                right: Box::new(for_precedence(state, Precedence::Prefix)?)
            }),
            _ => crate::parser_bail!(state, unreachable_code("unexpected operator"))
        };

        Ok(expr)
    })

    #[before(bitwise_prefix), current(TokenKind::Bang)]
    bang_prefix({
        Ok(Expression::LogicalOperation(LogicalOperationExpression::Not {
            comments: state.iterator.comments(),
            bang: utils::skip(state, TokenKind::Bang)?,
            right: Box::new(for_precedence(state, Precedence::Bang)?)
        }))
    })

    #[before(variable), current(TokenKind::BitwiseNot)]
    bitwise_prefix({
        Ok(Expression::BitwiseOperation(BitwiseOperationExpression::Not {
            comments: state.iterator.comments(),
            not: utils::skip(state, TokenKind::BitwiseNot)?,
            right: Box::new(for_precedence(state, Precedence::Prefix)?)
        }))
    })

    #[before(range_to), current(TokenKind::Variable)]
    variable({
        Ok(Expression::Variable(variable::parse(state)?))
    })

    #[before(unexpected_token), current(TokenKind::DoubleDot)]
    range_to({
        let comments = state.iterator.comments();
        let current = state.iterator.current();
        let position = current.position;
        state.iterator.next();

        let current = state.iterator.current();
        if current.kind == TokenKind::Equals {
            let equals = current.position;
            state.iterator.next();
            let expr = for_precedence(state, Precedence::Range)?;

            Ok(Expression::RangeOperation(RangeOperationExpression::ToInclusive {
                comments,
                double_dot: position,
                equals,
                to: Box::new(expr),
            }))
        } else if !is_range_terminator(&current.kind) {
            let expr = for_precedence(state, Precedence::Range)?;
            Ok(Expression::RangeOperation(RangeOperationExpression::To {
                comments,
                double_dot: position,
                to: Box::new(expr),
            }))
        } else {
            Ok(Expression::RangeOperation(RangeOperationExpression::Full {
                comments,
                double_dot: position,
            }))
        }
    })
}

fn unexpected_token(state: &mut State, _precedence: &Precedence) -> ParseResult<Expression> {
    crate::parser_bail!(
        state,
        unexpected_token(vec!["an expression"], state.iterator.current())
    );
}

/// Return true if the current token terminates an expression.
pub fn is_range_terminator(current: &TokenKind) -> bool {
    matches!(
        current,
        TokenKind::SemiColon
            | TokenKind::RightParen
            | TokenKind::RightBrace
            | TokenKind::LeftBrace
            | TokenKind::RightBracket
            | TokenKind::Comma
            | TokenKind::Colon
            | TokenKind::If
            | TokenKind::As
    )
}
