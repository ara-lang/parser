use crate::lexer::byte_string::ByteString;
use crate::lexer::issue;
use crate::lexer::result::SyntaxResult;
use crate::lexer::state::State;
use crate::lexer::token::TokenKind;

pub fn tokenize_single_quote(
    state: &mut State,
    opening: &[u8],
    opening_position: usize,
) -> SyntaxResult<(TokenKind, ByteString)> {
    let mut buffer = opening.to_vec();

    loop {
        match state.bytes.read(2) {
            [b'\'', ..] => {
                buffer.push(b'\'');
                state.bytes.next();
                break;
            }
            &[b'\\', b'\\'] => {
                state.bytes.skip(2);
                buffer.push(b'\\');
                buffer.push(b'\\');
            }
            &[b'\\', b'\''] => {
                state.bytes.skip(2);
                buffer.push(b'\\');
                buffer.push(b'\'');
            }
            &[b, ..] => {
                state.bytes.next();
                buffer.push(b);
            }
            [] => issue::bail!(state, unclosed_string_literal(opening_position)),
        }
    }

    Ok((TokenKind::LiteralString, buffer.into()))
}

pub fn tokenize_double_quote(
    state: &mut State,
    opening: &[u8],
    opening_position: usize,
) -> SyntaxResult<(TokenKind, ByteString)> {
    let mut buffer = opening.to_vec();

    loop {
        match state.bytes.read(3) {
            [b'"', ..] => {
                buffer.push(b'"');
                state.bytes.next();
                break;
            }
            &[b'\\', b @ (b'"' | b'\\' | b'$'), ..] => {
                state.bytes.skip(2);
                buffer.push(b);
            }
            &[b'\\', b'n', ..] => {
                state.bytes.skip(2);
                buffer.push(b'\n');
            }
            &[b'\\', b'r', ..] => {
                state.bytes.skip(2);
                buffer.push(b'\r');
            }
            &[b'\\', b't', ..] => {
                state.bytes.skip(2);
                buffer.push(b'\t');
            }
            &[b'\\', b'v', ..] => {
                state.bytes.skip(2);
                buffer.push(b'\x0b');
            }
            &[b'\\', b'e', ..] => {
                state.bytes.skip(2);
                buffer.push(b'\x1b');
            }
            &[b'\\', b'f', ..] => {
                state.bytes.skip(2);
                buffer.push(b'\x0c');
            }
            &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                state.bytes.skip(3);

                let mut hex = String::from(b as char);
                if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) = state.bytes.current() {
                    state.bytes.next();
                    hex.push(*b as char);
                }

                let b = u8::from_str_radix(&hex, 16).unwrap();
                buffer.push(b);
            }
            &[b'\\', b'u', b'{'] => {
                let start = state.bytes.span().position;

                state.bytes.skip(3);

                let mut code_point = String::new();
                while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                    state.bytes.current()
                {
                    state.bytes.next();
                    code_point.push(*b as char);
                }

                if code_point.is_empty() || state.bytes.current() != Some(&b'}') {
                    issue::bail!(state, invalid_unicode_escape_sequence(start));
                }

                state.bytes.next();

                let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                    c
                } else {
                    issue::bail!(state, invalid_unicode_escape_sequence(start));
                };

                if let Some(c) = char::from_u32(c) {
                    let mut tmp = [0; 4];
                    let bytes = c.encode_utf8(&mut tmp);
                    buffer.extend(bytes.as_bytes());
                } else {
                    issue::bail!(state, invalid_unicode_escape_sequence(start));
                }
            }
            &[b'\\', b @ b'0'..=b'7', ..] => {
                let start = state.bytes.span().position;
                state.bytes.skip(2);

                let mut octal = String::from(b as char);
                if let Some(b @ b'0'..=b'7') = state.bytes.current() {
                    state.bytes.next();
                    octal.push(*b as char);
                }

                if let Some(b @ b'0'..=b'7') = state.bytes.current() {
                    state.bytes.next();
                    octal.push(*b as char);
                }

                if let Ok(b) = u8::from_str_radix(&octal, 8) {
                    buffer.push(b);
                } else {
                    issue::bail!(state, invalid_octal_escape_sequence(start));
                }
            }
            &[b, ..] => {
                state.bytes.next();
                buffer.push(b);
            }
            [] => issue::bail!(state, unclosed_string_literal(opening_position)),
        }
    }

    Ok((TokenKind::LiteralString, buffer.into()))
}
