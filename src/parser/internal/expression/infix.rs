use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::r#type;
use crate::parser::internal::expression;
use crate::parser::internal::expression::precedence::Precedence;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::expression::operator::ArithmeticOperationExpression;
use crate::tree::expression::operator::AssignmentOperationExpression;
use crate::tree::expression::operator::BitwiseOperationExpression;
use crate::tree::expression::operator::ComparisonOperationExpression;
use crate::tree::expression::operator::LogicalOperationExpression;
use crate::tree::expression::operator::StringOperationExpression;
use crate::tree::expression::operator::TernaryOperationExpression;
use crate::tree::expression::operator::TypeOperationExpression;
use crate::tree::expression::Expression;

pub fn infix(
    state: &mut State,
    left: Expression,
    kind: &TokenKind,
    right_precedence: Precedence,
) -> ParseResult<Expression> {
    let comments = state.iterator.comments();

    let span = state.iterator.current().span;
    state.iterator.next();
    let op = state.iterator.current();

    Ok(match kind {
        TokenKind::Question => {
            // this happens due to a comment, or whitespaces between the ? and the :
            if op.kind == TokenKind::Colon {
                state.iterator.next();

                Expression::TernaryOperation(TernaryOperationExpression::ImplicitShortTernary {
                    comments,
                    condition: Box::new(left),
                    question: span,
                    colon: op.span,
                    if_false: Box::new(expression::create(state)?),
                })
            } else {
                Expression::TernaryOperation(TernaryOperationExpression::Ternary {
                    comments,
                    condition: Box::new(left),
                    question: span,
                    if_true: Box::new(expression::create(state)?),
                    colon: utils::skip_colon(state)?,
                    if_false: Box::new(expression::create(state)?),
                })
            }
        }
        TokenKind::QuestionColon => {
            Expression::TernaryOperation(TernaryOperationExpression::ShortTernary {
                comments,
                condition: Box::new(left),
                question_colon: span,
                if_false: Box::new(expression::create(state)?),
            })
        }
        TokenKind::Is => Expression::TypeOperation(TypeOperationExpression::Is {
            comments,
            left: Box::new(left),
            is: span,
            right: r#type::type_definition(state)?,
        }),
        TokenKind::As => Expression::TypeOperation(TypeOperationExpression::As {
            comments,
            left: Box::new(left),
            r#as: span,
            right: r#type::type_definition(state)?,
        }),
        TokenKind::Instanceof => Expression::TypeOperation(TypeOperationExpression::Instanceof {
            comments,
            left: Box::new(left),
            r#instanceof: span,
            right: identifier::fully_qualified_type_identifier_including_self(state)?,
        }),
        _ => {
            let left = Box::new(left);
            let right = Box::new(expression::for_precedence(state, right_precedence)?);

            match kind {
                TokenKind::Plus => {
                    Expression::ArithmeticOperation(ArithmeticOperationExpression::Addition {
                        comments,
                        left,
                        plus: span,
                        right,
                    })
                }
                TokenKind::Minus => {
                    Expression::ArithmeticOperation(ArithmeticOperationExpression::Subtraction {
                        comments,
                        left,
                        minus: span,
                        right,
                    })
                }
                TokenKind::Asterisk => {
                    Expression::ArithmeticOperation(ArithmeticOperationExpression::Multiplication {
                        comments,
                        left,
                        asterisk: span,
                        right,
                    })
                }
                TokenKind::Slash => {
                    Expression::ArithmeticOperation(ArithmeticOperationExpression::Division {
                        comments,
                        left,
                        slash: span,
                        right,
                    })
                }
                TokenKind::Percent => {
                    Expression::ArithmeticOperation(ArithmeticOperationExpression::Modulo {
                        comments,
                        left,
                        percent: span,
                        right,
                    })
                }
                TokenKind::Pow => {
                    Expression::ArithmeticOperation(ArithmeticOperationExpression::Exponentiation {
                        comments,
                        left,
                        pow: span,
                        right,
                    })
                }
                TokenKind::Equals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Assignment {
                        comments,
                        left,
                        equals: span,
                        right,
                    })
                }
                TokenKind::PlusEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Addition {
                        comments,
                        left,
                        plus_equals: span,
                        right,
                    })
                }
                TokenKind::MinusEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Subtraction {
                        comments,
                        left,
                        minus_equals: span,
                        right,
                    })
                }
                TokenKind::AsteriskEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Multiplication {
                        comments,
                        left,
                        asterisk_equals: span,
                        right,
                    })
                }
                TokenKind::SlashEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Division {
                        comments,
                        left,
                        slash_equals: span,
                        right,
                    })
                }
                TokenKind::PercentEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Modulo {
                        comments,
                        left,
                        percent_equals: span,
                        right,
                    })
                }
                TokenKind::PowEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Exponentiation {
                        comments,
                        left,
                        pow_equals: span,
                        right,
                    })
                }
                TokenKind::AmpersandEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::BitwiseAnd {
                        comments,
                        left,
                        ampersand_equals: span,
                        right,
                    })
                }
                TokenKind::PipeEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::BitwiseOr {
                        comments,
                        left,
                        pipe_equals: span,
                        right,
                    })
                }
                TokenKind::CaretEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::BitwiseXor {
                        comments,
                        left,
                        caret_equals: span,
                        right,
                    })
                }
                TokenKind::LeftShiftEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::LeftShift {
                        comments,
                        left,
                        left_shift_equals: span,
                        right,
                    })
                }
                TokenKind::RightShiftEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::RightShift {
                        comments,
                        left,
                        right_shift_equals: span,
                        right,
                    })
                }
                TokenKind::DoubleQuestionEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Coalesce {
                        comments,
                        left,
                        coalesce_equals: span,
                        right,
                    })
                }
                TokenKind::DotEquals => {
                    Expression::AssignmentOperation(AssignmentOperationExpression::Concat {
                        comments,
                        left,
                        dot_equals: span,
                        right,
                    })
                }
                TokenKind::Ampersand => {
                    Expression::BitwiseOperation(BitwiseOperationExpression::And {
                        comments,
                        left,
                        and: span,
                        right,
                    })
                }
                TokenKind::Pipe => Expression::BitwiseOperation(BitwiseOperationExpression::Or {
                    comments,
                    left,
                    or: span,
                    right,
                }),
                TokenKind::Caret => Expression::BitwiseOperation(BitwiseOperationExpression::Xor {
                    comments,
                    left,
                    xor: span,
                    right,
                }),
                TokenKind::LeftShift => {
                    Expression::BitwiseOperation(BitwiseOperationExpression::LeftShift {
                        comments,
                        left,
                        left_shift: span,
                        right,
                    })
                }
                TokenKind::RightShift => {
                    Expression::BitwiseOperation(BitwiseOperationExpression::RightShift {
                        comments,
                        left,
                        right_shift: span,
                        right,
                    })
                }
                TokenKind::DoubleEquals => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::Equal {
                        comments,
                        left,
                        double_equals: span,
                        right,
                    })
                }
                TokenKind::TripleEquals => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::Identical {
                        comments,
                        left,
                        triple_equals: span,
                        right,
                    })
                }
                TokenKind::BangEquals => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::NotEqual {
                        comments,
                        left,
                        bang_equals: span,
                        right,
                    })
                }
                TokenKind::AngledLeftRight => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::AngledNotEqual {
                        comments,
                        left,
                        angled_left_right: span,
                        right,
                    })
                }
                TokenKind::BangDoubleEquals => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::NotIdentical {
                        comments,
                        left,
                        bang_double_equals: span,
                        right,
                    })
                }
                TokenKind::LessThan => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::LessThan {
                        comments,
                        left,
                        less_than: span,
                        right,
                    })
                }
                TokenKind::GreaterThan => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::GreaterThan {
                        comments,
                        left,
                        greater_than: span,
                        right,
                    })
                }
                TokenKind::LessThanEquals => Expression::ComparisonOperation(
                    ComparisonOperationExpression::LessThanOrEqual {
                        comments,
                        left,
                        less_than_equals: span,
                        right,
                    },
                ),
                TokenKind::GreaterThanEquals => Expression::ComparisonOperation(
                    ComparisonOperationExpression::GreaterThanOrEqual {
                        comments,
                        left,
                        greater_than_equals: span,
                        right,
                    },
                ),
                TokenKind::Spaceship => {
                    Expression::ComparisonOperation(ComparisonOperationExpression::Spaceship {
                        comments,
                        left,
                        spaceship: span,
                        right,
                    })
                }
                TokenKind::BooleanAnd => {
                    Expression::LogicalOperation(LogicalOperationExpression::And {
                        comments,
                        left,
                        double_ampersand: span,
                        right,
                    })
                }
                TokenKind::BooleanOr => {
                    Expression::LogicalOperation(LogicalOperationExpression::Or {
                        comments,
                        left,
                        double_pipe: span,
                        right,
                    })
                }
                TokenKind::LogicalAnd => {
                    Expression::LogicalOperation(LogicalOperationExpression::LogicalAnd {
                        comments,
                        left,
                        and: span,
                        right,
                    })
                }
                TokenKind::LogicalOr => {
                    Expression::LogicalOperation(LogicalOperationExpression::LogicalOr {
                        comments,
                        left,
                        or: span,
                        right,
                    })
                }
                TokenKind::LogicalXor => {
                    Expression::LogicalOperation(LogicalOperationExpression::LogicalXor {
                        comments,
                        left,
                        xor: span,
                        right,
                    })
                }
                TokenKind::Dot => Expression::StringOperation(StringOperationExpression::Concat {
                    comments,
                    left,
                    dot: span,
                    right,
                }),
                _ => unreachable!(),
            }
        }
    })
}

#[inline(always)]
pub fn is_infix(state: &mut State, t: &TokenKind) -> bool {
    if t == &TokenKind::As {
        return state.iterator.lookahead(1).kind != TokenKind::Variable;
    }

    matches!(
        t,
        TokenKind::Pow
            | TokenKind::RightShiftEquals
            | TokenKind::LeftShiftEquals
            | TokenKind::CaretEquals
            | TokenKind::AmpersandEquals
            | TokenKind::PipeEquals
            | TokenKind::PercentEquals
            | TokenKind::PowEquals
            | TokenKind::LogicalAnd
            | TokenKind::LogicalOr
            | TokenKind::LogicalXor
            | TokenKind::Spaceship
            | TokenKind::LeftShift
            | TokenKind::RightShift
            | TokenKind::Ampersand
            | TokenKind::Pipe
            | TokenKind::Caret
            | TokenKind::Percent
            | TokenKind::Is
            | TokenKind::Instanceof
            | TokenKind::Asterisk
            | TokenKind::Slash
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Dot
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanEquals
            | TokenKind::GreaterThanEquals
            | TokenKind::DoubleEquals
            | TokenKind::TripleEquals
            | TokenKind::BangEquals
            | TokenKind::BangDoubleEquals
            | TokenKind::AngledLeftRight
            | TokenKind::Question
            | TokenKind::QuestionColon
            | TokenKind::BooleanAnd
            | TokenKind::BooleanOr
            | TokenKind::Equals
            | TokenKind::PlusEquals
            | TokenKind::MinusEquals
            | TokenKind::DotEquals
            | TokenKind::DoubleQuestionEquals
            | TokenKind::AsteriskEquals
            | TokenKind::SlashEquals
    )
}
