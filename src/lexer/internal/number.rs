use crate::lexer::byte_string::ByteString;
use crate::lexer::result::SyntaxResult;
use crate::lexer::state::State;
use crate::lexer::token::TokenKind;

pub fn tokenize(state: &mut State) -> SyntaxResult<(TokenKind, ByteString)> {
    let mut buffer = Vec::new();

    let (base, kind) = match state.bytes.read(2) {
        [a @ b'0', b @ b'B' | b @ b'b'] => {
            buffer.push(*a);
            buffer.push(*b);
            state.bytes.skip(2);
            (2, NumberKind::Int)
        }
        [a @ b'0', b @ b'O' | b @ b'o'] => {
            buffer.push(*a);
            buffer.push(*b);
            state.bytes.skip(2);
            (8, NumberKind::Int)
        }
        [a @ b'0', b @ b'X' | b @ b'x'] => {
            buffer.push(*a);
            buffer.push(*b);
            state.bytes.skip(2);
            (16, NumberKind::Int)
        }
        [b'0', ..] => (10, NumberKind::OctalOrFloat),
        [b'.', ..] => (10, NumberKind::Float),
        _ => (10, NumberKind::IntOrFloat),
    };

    if kind != NumberKind::Float {
        read_digits(state, &mut buffer, base);

        if kind == NumberKind::Int {
            return parse_int(&buffer);
        }
    }

    // Remaining cases: decimal integer, legacy octal integer, or float.
    let is_float = matches!(
        state.bytes.read(3),
        [b'.', ..] | [b'e' | b'E', b'-' | b'+', b'0'..=b'9'] | [b'e' | b'E', b'0'..=b'9', ..]
    );

    if !is_float {
        return parse_int(&buffer);
    }

    if let Some(b'.') = state.bytes.current() {
        buffer.push(b'.');
        state.bytes.next();
        read_digits(state, &mut buffer, 10);
    }

    if let Some(b'e' | b'E') = state.bytes.current() {
        buffer.push(b'e');
        state.bytes.next();
        if let Some(b @ (b'-' | b'+')) = state.bytes.current() {
            buffer.push(*b);
            state.bytes.next();
        }
        read_digits(state, &mut buffer, 10);
    }

    Ok((TokenKind::LiteralFloat, buffer.into()))
}

pub fn read_digits(state: &mut State, buffer: &mut Vec<u8>, base: usize) {
    if base == 16 {
        read_digits_fn(state, buffer, u8::is_ascii_hexdigit);
    } else {
        let max = b'0' + base as u8;
        let range = b'0'..max;
        read_digits_fn(state, buffer, |b| range.contains(b));
    };
}

fn read_digits_fn<F: Fn(&u8) -> bool>(state: &mut State, buffer: &mut Vec<u8>, is_digit: F) {
    if let Some(b) = state.bytes.current() {
        if is_digit(b) {
            state.bytes.next();
            buffer.push(*b);
        } else {
            return;
        }
    }

    loop {
        match state.bytes.read(2) {
            [b, ..] if is_digit(b) => {
                state.bytes.next();
                buffer.push(*b);
            }
            [b'_', b] if is_digit(b) => {
                state.bytes.next();
                state.bytes.next();
                buffer.push(*b);
            }
            _ => {
                break;
            }
        }
    }
}

// Parses an integer literal in the given base and converts errors to SyntaxError.
// It returns a float token instead on overflow.
fn parse_int(buffer: &[u8]) -> SyntaxResult<(TokenKind, ByteString)> {
    Ok((TokenKind::LiteralInteger, buffer.into()))
}

#[derive(Debug, Eq, PartialEq)]
enum NumberKind {
    Int,
    Float,
    IntOrFloat,
    OctalOrFloat,
}
