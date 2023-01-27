use crate::lexer::token::TokenKind;
use crate::parser::result::ParseResult;
use crate::parser::state::State;

pub enum Associativity {
    Non,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Precedence {
    Lowest,
    Range,
    Yield,
    YieldFrom,
    IncDec,
    Assignment,
    Ternary,
    NullCoalesce,
    Or,
    And,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equality,
    LtGt,
    Concat,
    BitShift,
    AddSub,
    MulDivMod,
    Bang,
    TypeCheck,
    ArrayContains,
    Prefix,
    Pow,
    Clone,
    CallDim,
    ObjectAccess,
    New,
}

impl Precedence {
    pub fn infix(state: &mut State, kind: &TokenKind) -> ParseResult<Self> {
        use TokenKind::*;

        match kind {
            Pow => Ok(Self::Pow),
            Instanceof | Is | As | Into => Ok(Self::TypeCheck),
            In => Ok(Self::ArrayContains),
            Asterisk | Slash | Percent => Ok(Self::MulDivMod),
            Plus | Minus => Ok(Self::AddSub),
            LeftShift | RightShift => Ok(Self::BitShift),
            Dot => Ok(Self::Concat),
            LessThan | LessThanEquals | GreaterThan | GreaterThanEquals => Ok(Self::LtGt),
            DoubleEquals | BangEquals | TripleEquals | BangDoubleEquals | Spaceship => {
                Ok(Self::Equality)
            }
            Ampersand => Ok(Self::BitwiseAnd),
            Caret => Ok(Self::BitwiseXor),
            Pipe => Ok(Self::BitwiseOr),
            BooleanAnd => Ok(Self::And),
            BooleanOr => Ok(Self::Or),
            DoubleQuestion => Ok(Self::NullCoalesce),
            Question | QuestionColon => Ok(Self::Ternary),
            Equals | PlusEquals | MinusEquals | AsteriskEquals | PowEquals | SlashEquals
            | DotEquals | AndEquals | DoubleQuestionEquals | PercentEquals | AmpersandEquals
            | PipeEquals | CaretEquals | LeftShiftEquals | RightShiftEquals => Ok(Self::Assignment),
            Yield => Ok(Self::Yield),
            DoubleDot => Ok(Self::Range),
            _ => crate::parser_bail!(
                state,
                unreachable_code(format!("unexpected precedence for operator {kind:?}"))
            ),
        }
    }

    pub fn postfix(state: &mut State, kind: &TokenKind) -> ParseResult<Self> {
        use TokenKind::*;

        match kind {
            DoubleQuestion => Ok(Self::NullCoalesce),
            Increment | Decrement => Ok(Self::IncDec),
            LeftParen | Generic | LeftBracket => Ok(Self::CallDim),
            Arrow | QuestionArrow | DoubleColon => Ok(Self::ObjectAccess),
            _ => crate::parser_bail!(
                state,
                unreachable_code(format!("unexpected precedence for operator {kind:?}"))
            ),
        }
    }

    pub fn associativity(&self) -> Option<Associativity> {
        Some(match &self {
            Self::TypeCheck
            | Self::ArrayContains
            | Self::MulDivMod
            | Self::AddSub
            | Self::BitShift
            | Self::Concat
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::And
            | Self::Or => Associativity::Left,
            Self::Pow | Self::NullCoalesce | Self::Assignment => Associativity::Right,
            Self::Ternary | Self::Equality | Self::LtGt => Associativity::Non,
            _ => return None,
        })
    }
}
