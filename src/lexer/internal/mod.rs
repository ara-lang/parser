use crate::ident_start;
use crate::lexer::byte_string::ByteString;
use crate::lexer::result::SyntaxResult;
use crate::lexer::state::State;
use crate::lexer::token::OpenTagKind;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

pub mod identifier;
pub mod number;
pub mod string;
pub mod variable;

pub fn tokenize(state: &mut State) -> SyntaxResult<Token> {
    let position = state.bytes.position();
    let (kind, value): (TokenKind, ByteString) = match state.bytes.read(3) {
        [b'!', b'=', b'='] => {
            state.bytes.skip(3);

            (TokenKind::BangDoubleEquals, b"!==".into())
        }
        [b'?', b'?', b'='] => {
            state.bytes.skip(3);
            (TokenKind::DoubleQuestionEquals, b"??=".into())
        }
        [b'?', b'-', b'>'] => {
            state.bytes.skip(3);
            (TokenKind::QuestionArrow, b"?->".into())
        }
        [b'=', b'=', b'='] => {
            state.bytes.skip(3);
            (TokenKind::TripleEquals, b"===".into())
        }
        [b'.', b'.', b'.'] => {
            state.bytes.skip(3);
            (TokenKind::Ellipsis, b"...".into())
        }
        [b':', b':', b'<'] => {
            state.bytes.skip(3);
            (TokenKind::Generic, b"::<".into())
        }
        [b'@', ..] => {
            state.bytes.next();
            (TokenKind::At, b"@".into())
        }
        [b'!', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::BangEquals, b"!=".into())
        }
        [b'!', ..] => {
            state.bytes.next();
            (TokenKind::Bang, b"!".into())
        }
        [b'&', b'&', ..] => {
            state.bytes.skip(2);
            (TokenKind::BooleanAnd, b"&&".into())
        }
        [b'&', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::AmpersandEquals, b"&=".into())
        }
        [b'&', ..] => {
            state.bytes.next();
            (TokenKind::Ampersand, b"&".into())
        }
        [b'?', b'>', ..] => {
            state.bytes.skip(2);

            (TokenKind::CloseTag, b"?>".into())
        }
        [b'?', b'?', ..] => {
            state.bytes.skip(2);
            (TokenKind::DoubleQuestion, b"??".into())
        }
        [b'?', b':', ..] => {
            state.bytes.skip(2);
            (TokenKind::QuestionColon, b"?:".into())
        }
        [b'?', ..] => {
            state.bytes.next();
            (TokenKind::Question, b"?".into())
        }
        [b'=', b'>', ..] => {
            state.bytes.skip(2);
            (TokenKind::DoubleArrow, b"=>".into())
        }
        [b'=', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::DoubleEquals, b"==".into())
        }
        [b'=', ..] => {
            state.bytes.next();
            (TokenKind::Equals, b"=".into())
        }
        [b'<', b'?', b'=', ..] => {
            state.bytes.skip(3);
            (TokenKind::OpenTag(OpenTagKind::Echo), b"<?=".into())
        }
        [b'<', b'?', b'p', ..] if state.bytes.at_case_insensitive(b"<?php", 5) => {
            state.bytes.skip(5);
            (TokenKind::OpenTag(OpenTagKind::Full), b"<?php".into())
        }
        [b'<', b'?', ..] => {
            state.bytes.skip(2);
            (TokenKind::OpenTag(OpenTagKind::Short), b"<?".into())
        }
        // Single quoted string.
        [b'\'', ..] => {
            let opening_position = state.bytes.position();
            let opening = state.bytes.read_and_skip(1);
            string::tokenize_single_quote(state, opening, opening_position)?
        }
        [b'b' | b'B', b'\'', ..] => {
            let opening_position = state.bytes.position();
            let opening = state.bytes.read_and_skip(2);
            string::tokenize_single_quote(state, opening, opening_position)?
        }
        [b'"', ..] => {
            let opening_position = state.bytes.position();
            let opening = state.bytes.read_and_skip(1);
            string::tokenize_double_quote(state, opening, opening_position)?
        }
        [b'b' | b'B', b'"', ..] => {
            let opening_position = state.bytes.position();
            let opening = state.bytes.read_and_skip(2);
            string::tokenize_double_quote(state, opening, opening_position)?
        }
        [b'$', ident_start!(), ..] => variable::tokenize(state),
        [b'$', ..] => {
            state.bytes.next();
            (TokenKind::Dollar, b"$".into())
        }
        [b'.', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::DotEquals, b".=".into())
        }
        [b'0'..=b'9', ..] => number::tokenize(state)?,
        [b'.', b'0'..=b'9', ..] => number::tokenize(state)?,
        [b'.', ..] => {
            state.bytes.next();
            (TokenKind::Dot, b".".into())
        }
        [b'\\', ident_start!(), ..] => {
            state.bytes.next();

            match tokenize(state)? {
                Token {
                    kind: TokenKind::Identifier | TokenKind::QualifiedIdentifier,
                    value,
                    ..
                } => {
                    let mut bytes = value;
                    bytes.insert(0, b'\\');
                    bytes.length += 1;

                    (TokenKind::FullyQualifiedIdentifier, bytes)
                }
                Token {
                    kind: TokenKind::True,
                    ..
                } => (TokenKind::FullyQualifiedIdentifier, b"\\true".into()),
                Token {
                    kind: TokenKind::False,
                    ..
                } => (TokenKind::FullyQualifiedIdentifier, b"\\false".into()),
                Token {
                    kind: TokenKind::Null,
                    ..
                } => (TokenKind::FullyQualifiedIdentifier, b"\\null".into()),
                s => unreachable!("{:?}", s),
            }
        }
        [b'\\', ..] => {
            state.bytes.next();
            (TokenKind::NamespaceSeparator, b"\\".into())
        }
        [b'/', b'*', ..] => {
            state.bytes.next();
            let mut buffer = vec![b'/'];

            loop {
                match state.bytes.read(2) {
                    [b'*', b'/'] => {
                        state.bytes.skip(2);
                        buffer.extend_from_slice(b"*/");
                        break;
                    }
                    &[t, ..] => {
                        state.bytes.next();
                        buffer.push(t);
                    }
                    _ => {
                        break;
                    }
                }
            }

            if buffer.starts_with(b"/**") {
                (TokenKind::DocumentComment, buffer.into())
            } else {
                (TokenKind::MultiLineComment, buffer.into())
            }
        }
        [b'#', b'[', ..] => {
            state.bytes.skip(2);
            (TokenKind::Attribute, b"#[".into())
        }
        [ch @ b'/', b'/', ..] | [ch @ b'#', ..] => {
            let mut buffer = if *ch == b'/' {
                state.bytes.skip(2);
                b"//".to_vec()
            } else {
                state.bytes.next();
                b"#".to_vec()
            };

            while let Some(c) = state.bytes.current() {
                if *c == b'\n' {
                    state.bytes.next();
                    break;
                }

                if state.bytes.read(2) == [b'?', b'>'] {
                    break;
                }

                buffer.push(*c);
                state.bytes.next();
            }

            if buffer.starts_with(b"#") {
                (TokenKind::HashMarkComment, buffer.into())
            } else {
                (TokenKind::SingleLineComment, buffer.into())
            }
        }
        [b'/', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::SlashEquals, b"/=".into())
        }
        [b'/', ..] => {
            state.bytes.next();
            (TokenKind::Slash, b"/".into())
        }
        [b'*', b'*', b'=', ..] => {
            state.bytes.skip(3);
            (TokenKind::PowEquals, b"**=".into())
        }
        [b'<', b'<', b'='] => {
            state.bytes.skip(3);

            (TokenKind::LeftShiftEquals, b"<<=".into())
        }
        [b'<', b'=', b'>'] => {
            state.bytes.skip(3);
            (TokenKind::Spaceship, b"<=>".into())
        }
        [b'>', b'>', b'='] => {
            state.bytes.skip(3);
            (TokenKind::RightShiftEquals, b">>=".into())
        }
        [b'*', b'*', ..] => {
            state.bytes.skip(2);
            (TokenKind::Pow, b"**".into())
        }
        [b'*', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::AsteriskEquals, b"*=".into())
        }
        [b'*', ..] => {
            state.bytes.next();
            (TokenKind::Asterisk, b"*".into())
        }
        [b'|', b'|', ..] => {
            state.bytes.skip(2);
            (TokenKind::BooleanOr, b"||".into())
        }
        [b'|', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::PipeEquals, b"|=".into())
        }
        [b'|', ..] => {
            state.bytes.next();
            (TokenKind::Pipe, b"|".into())
        }
        [b'^', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::CaretEquals, b"^=".into())
        }
        [b'^', ..] => {
            state.bytes.next();
            (TokenKind::Caret, b"^".into())
        }
        [b'{', ..] => {
            state.bytes.next();
            (TokenKind::LeftBrace, b"{".into())
        }
        [b'}', ..] => {
            state.bytes.next();
            (TokenKind::RightBrace, b"}".into())
        }
        [b'(', ..] => {
            state.bytes.next();
            (TokenKind::LeftParen, b"(".into())
        }
        [b')', ..] => {
            state.bytes.next();
            (TokenKind::RightParen, b")".into())
        }
        [b';', ..] => {
            state.bytes.next();
            (TokenKind::SemiColon, b";".into())
        }
        [b'+', b'+', ..] => {
            state.bytes.skip(2);
            (TokenKind::Increment, b"++".into())
        }
        [b'+', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::PlusEquals, b"+=".into())
        }
        [b'+', ..] => {
            state.bytes.next();
            (TokenKind::Plus, b"+".into())
        }
        [b'%', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::PercentEquals, b"%=".into())
        }
        [b'%', ..] => {
            state.bytes.next();
            (TokenKind::Percent, b"%".into())
        }
        [b'-', b'-', ..] => {
            state.bytes.skip(2);
            (TokenKind::Decrement, b"--".into())
        }
        [b'-', b'>', ..] => {
            state.bytes.skip(2);
            (TokenKind::Arrow, b"->".into())
        }
        [b'-', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::MinusEquals, b"-=".into())
        }
        [b'-', ..] => {
            state.bytes.next();
            (TokenKind::Minus, b"-".into())
        }
        [b'<', b'<', ..] => {
            state.bytes.skip(2);
            (TokenKind::LeftShift, b"<<".into())
        }
        [b'<', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::LessThanEquals, b"<=".into())
        }
        [b'<', ..] => {
            state.bytes.next();
            (TokenKind::LessThan, b"<".into())
        }
        [b'>', b'>', ..] => {
            state.bytes.skip(2);
            (TokenKind::RightShift, b">>".into())
        }
        [b'>', b'=', ..] => {
            state.bytes.skip(2);
            (TokenKind::GreaterThanEquals, b">=".into())
        }
        [b'>', ..] => {
            state.bytes.next();
            (TokenKind::GreaterThan, b">".into())
        }
        [b',', ..] => {
            state.bytes.next();
            (TokenKind::Comma, b",".into())
        }
        [b'[', ..] => {
            state.bytes.next();
            (TokenKind::LeftBracket, b"[".into())
        }
        [b']', ..] => {
            state.bytes.next();
            (TokenKind::RightBracket, b"]".into())
        }
        [b':', b':', ..] => {
            state.bytes.skip(2);
            (TokenKind::DoubleColon, b"::".into())
        }
        [b':', ..] => {
            state.bytes.next();
            (TokenKind::Colon, b":".into())
        }
        [b'~', ..] => {
            state.bytes.next();
            (TokenKind::BitwiseNot, b"~".into())
        }
        [b'`', ..] => {
            state.bytes.next();
            (TokenKind::Backtick, b"`".into())
        }
        [_, ..] => {
            let (qualified, buffer) = identifier::tokenize(state)?;

            if qualified {
                (TokenKind::QualifiedIdentifier, buffer.into())
            } else {
                let kind = identifier::to_keyword(&buffer).unwrap_or(TokenKind::Identifier);

                (kind, buffer.into())
            }
        }
        [] => unreachable!(),
    };

    Ok(Token {
        kind,
        position,
        value,
    })
}
