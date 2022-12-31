use crate::lexer::token::Span;

#[derive(Debug)]
pub struct SourceBytes<'a> {
    input: &'a [u8],
    length: usize,
    span: Span,
}

impl<'a> SourceBytes<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let input = input;
        let length = input.len();

        Self {
            input,
            length,
            span: Span::new(1, 1, 0),
        }
    }

    pub const fn span(&self) -> Span {
        self.span
    }

    pub const fn eof(&self) -> bool {
        self.span.position >= self.length
    }

    pub fn next(&mut self) {
        if !self.eof() {
            match self.input[self.span.position] {
                b'\n' => {
                    self.span.line += 1;
                    self.span.column = 1;
                }
                _ => self.span.column += 1,
            }
        }

        self.span.position += 1;
    }

    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.next();
        }
    }

    pub fn read_and_skip(&mut self, count: usize) -> &'a [u8] {
        let (from, until) = self.to_bound(count);

        self.skip(count);

        &self.input[from..until]
    }

    pub fn current(&self) -> Option<&'a u8> {
        if self.span.position >= self.length {
            None
        } else {
            Some(&self.input[self.span.position])
        }
    }

    pub fn read(&self, n: usize) -> &'a [u8] {
        let (from, until) = self.to_bound(n);

        &self.input[from..until]
    }

    pub fn at_case_insensitive(&self, search: &[u8], len: usize) -> bool {
        let (from, until) = self.to_bound(len);

        let slice = &self.input[from..until];

        slice.eq_ignore_ascii_case(search)
    }

    pub fn peek(&self, i: usize, n: usize) -> &'a [u8] {
        let from = self.span.position + i;
        if from >= self.length {
            return &self.input[self.length..self.length];
        }

        let mut until = from + n;
        if until >= self.length {
            until = self.length;
        }

        &self.input[from..until]
    }

    const fn to_bound(&self, n: usize) -> (usize, usize) {
        if self.span.position >= self.length {
            return (self.length, self.length);
        }

        let mut until = self.span.position + n;

        if until >= self.length {
            until = self.length;
        }

        (self.span.position, until)
    }
}
