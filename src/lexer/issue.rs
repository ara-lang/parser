#![macro_use]

use ara_reporting::issue::Issue;

use crate::lexer::state::State;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum LexerIssueCode {
    /// An unclosed string literal was encountered.
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(): void {
    ///     $a = "Hello, World!
    /// }
    /// ```
    UnclosedStringLiteral = 1,

    /// An invalid unicode escape sequence was encountered.
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(): void {
    ///    $a = "\u{1234567890}";
    /// }
    /// ```
    InvalidUnicodeEscapeSequence = 2,

    /// An invalid octal escape sequence was encountered.
    ///
    /// Example:
    ///
    /// ```ara
    /// function foo(): void {
    ///   $a = "\1234567890";
    /// }
    /// ```
    InvalidOctalEscapeSequence = 3,

    /// An unrecognizable token was encountered.
    UnrecognizableToken = 4,
}

pub(crate) fn unclosed_string_literal(state: &State, from: usize) -> Issue {
    let origin = state.source.name();

    Issue::error(
        LexerIssueCode::UnclosedStringLiteral,
        "Unclosed string literal",
        origin,
        from,
        state.bytes.position(),
    )
}

pub(crate) fn invalid_unicode_escape_sequence(state: &State, from: usize) -> Issue {
    let origin = state.source.name();

    Issue::error(
        LexerIssueCode::InvalidUnicodeEscapeSequence,
        "Invalid unicode escape sequence",
        origin,
        from,
        state.bytes.position(),
    )
}

pub(crate) fn invalid_octal_escape_sequence(state: &State, from: usize) -> Issue {
    let origin = state.source.name();

    Issue::error(
        LexerIssueCode::InvalidOctalEscapeSequence,
        "Invalid octal escape sequence",
        origin,
        from,
        state.bytes.position(),
    )
}

pub(crate) fn unrecognizable_token(state: &State) -> Issue {
    let origin = state.source.name();
    let position = state.bytes.position();

    Issue::error(
        LexerIssueCode::UnrecognizableToken,
        "Unrecognizable token",
        origin,
        position,
        position + 1,
    )
}

impl ::std::fmt::Display for LexerIssueCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "L{:04}", *self as u8)
    }
}

impl From<LexerIssueCode> for String {
    fn from(code: LexerIssueCode) -> String {
        format!("{}", code)
    }
}

#[macro_export]
macro_rules! __internal_lexer_issue_bail {
    ($state:expr, $issue:ident($($args:expr),+$(,)?)$(,)?) => {
        {
            let issue = $crate::lexer::issue::$issue($state, $($args,)+);

            return Err(Box::new(issue));
        }
    };
    ($state:expr, $issue:ident) => {
        {
            let issue = $crate::lexer::issue::$issue($state);

            return Err(Box::new(issue));
        }
    };
}

pub(crate) use __internal_lexer_issue_bail as bail;
