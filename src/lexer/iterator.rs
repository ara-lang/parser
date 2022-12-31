use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

use crate::tree::comment::Comment;
use crate::tree::comment::CommentFormat;
use crate::tree::comment::CommentGroup;

/// Token stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenIterator<'a> {
    tokens: &'a [Token],
    length: usize,
    comments: Vec<&'a Token>,
    cursor: usize,
    collect_comments: bool,
}

/// Token stream.
impl<'a> TokenIterator<'a> {
    pub fn new(tokens: &'a [Token]) -> TokenIterator {
        let length = tokens.len();

        let mut stream = TokenIterator {
            tokens,
            length,
            comments: vec![],
            cursor: 0,
            collect_comments: true,
        };

        stream.collect_comments();

        stream
    }

    /// Reset the cursor.
    pub fn reset(&mut self) {
        self.cursor = 0;
        self.comments.clear();
    }

    /// Reset the cursor, and disable comment collection.
    pub fn without_comment_collection(&mut self) {
        self.reset();
        self.collect_comments = false;
    }

    /// Reset the cursor, and enable comment collection.
    pub fn with_comment_collection(&mut self) {
        self.reset();
        self.collect_comments = true;
    }

    /// Move cursor to next token.
    pub fn next(&mut self) {
        self.cursor += 1;

        self.collect_comments();
    }

    /// Get current token.
    pub const fn current(&self) -> &'a Token {
        let position = if self.cursor >= self.length {
            self.length - 1
        } else {
            self.cursor
        };

        &self.tokens[position]
    }

    /// Get previous token.
    pub const fn previous(&self) -> &'a Token {
        let position = if self.cursor == 0 { 0 } else { self.cursor - 1 };

        &self.tokens[position]
    }

    /// lookahead to the n'th token.
    pub const fn lookahead(&self, n: usize) -> &'a Token {
        let mut cursor = self.cursor + 1;
        let mut target = 1;
        loop {
            if cursor >= self.length {
                return &self.tokens[self.length - 1];
            }

            let current = &self.tokens[cursor];

            if self.collect_comments
                && matches!(
                    current.kind,
                    TokenKind::SingleLineComment
                        | TokenKind::MultiLineComment
                        | TokenKind::HashMarkComment
                        | TokenKind::DocumentComment
                )
            {
                cursor += 1;
                continue;
            }

            if target == n {
                return current;
            }

            target += 1;
            cursor += 1;
        }
    }

    /// Check if current token is EOF.
    pub fn is_eof(&self) -> bool {
        if self.cursor >= self.length {
            return true;
        }

        self.tokens[self.cursor].kind == TokenKind::Eof
    }

    /// Get collected comments.
    ///
    /// If comment collection is disabled, this will return an empty `CommentGroup`.
    pub fn comments(&mut self) -> CommentGroup {
        let mut comments = vec![];

        std::mem::swap(&mut self.comments, &mut comments);

        CommentGroup {
            comments: comments
                .iter()
                .map(|token| match token {
                    Token {
                        kind: TokenKind::SingleLineComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::SingleLine,
                        content: value.clone(),
                    },
                    Token {
                        kind: TokenKind::MultiLineComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::MultiLine,
                        content: value.clone(),
                    },
                    Token {
                        kind: TokenKind::HashMarkComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::HashMark,
                        content: value.clone(),
                    },
                    Token {
                        kind: TokenKind::DocumentComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::Document,
                        content: value.clone(),
                    },
                    _ => unreachable!(),
                })
                .collect(),
        }
    }

    fn collect_comments(&mut self) {
        if !self.collect_comments {
            return;
        }

        loop {
            if self.cursor >= self.length {
                break;
            }

            let current = &self.tokens[self.cursor];

            if !matches!(
                current.kind,
                TokenKind::SingleLineComment
                    | TokenKind::MultiLineComment
                    | TokenKind::HashMarkComment
                    | TokenKind::DocumentComment
            ) {
                break;
            }

            self.comments.push(current);
            self.cursor += 1;
        }
    }
}

impl<'a> Default for TokenIterator<'a> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a> From<&'a Vec<Token>> for TokenIterator<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
        Self::new(tokens.as_slice())
    }
}
