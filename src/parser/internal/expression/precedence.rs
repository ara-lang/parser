use crate::lexer::token::TokenKind;

pub enum Associativity {
    Non,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Precedence {
    Lowest,
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
    pub fn infix(kind: &TokenKind) -> Self {
        use TokenKind::*;

        match kind {
            Pow => Self::Pow,
            Instanceof | Is | As => Self::TypeCheck,
            In => Self::ArrayContains,
            Asterisk | Slash | Percent => Self::MulDivMod,
            Plus | Minus => Self::AddSub,
            LeftShift | RightShift => Self::BitShift,
            Dot => Self::Concat,
            LessThan | LessThanEquals | GreaterThan | GreaterThanEquals => Self::LtGt,
            DoubleEquals | BangEquals | TripleEquals | BangDoubleEquals | Spaceship => {
                Self::Equality
            }
            Ampersand => Self::BitwiseAnd,
            Caret => Self::BitwiseXor,
            Pipe => Self::BitwiseOr,
            BooleanAnd => Self::And,
            BooleanOr => Self::Or,
            DoubleQuestion => Self::NullCoalesce,
            Question | QuestionColon => Self::Ternary,
            Equals | PlusEquals | MinusEquals | AsteriskEquals | PowEquals | SlashEquals
            | DotEquals | AndEquals | DoubleQuestionEquals | PercentEquals | AmpersandEquals
            | PipeEquals | CaretEquals | LeftShiftEquals | RightShiftEquals => Self::Assignment,
            Yield => Self::Yield,
            _ => unimplemented!("precedence for op {:?}", kind),
        }
    }

    pub fn postfix(kind: &TokenKind) -> Self {
        use TokenKind::*;

        match kind {
            DoubleQuestion => Self::NullCoalesce,
            Increment | Decrement => Self::IncDec,
            LeftParen | Generic | LeftBracket => Self::CallDim,
            Arrow | QuestionArrow | DoubleColon => Self::ObjectAccess,
            _ => unimplemented!("postfix precedence for op {:?}", kind),
        }
    }

    pub fn associativity(&self) -> Option<Associativity> {
        Some(match self {
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
