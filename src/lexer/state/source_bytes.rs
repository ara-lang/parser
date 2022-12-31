#[derive(Debug)]
pub struct SourceBytes<'a> {
    input: &'a [u8],
    length: usize,
    position: usize,
}

impl<'a> SourceBytes<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let input = input;
        let length = input.len();

        Self {
            input,
            length,
            position: 0,
        }
    }

    pub const fn position(&self) -> usize {
        self.position
    }

    pub const fn eof(&self) -> bool {
        self.position >= self.length
    }

    pub fn next(&mut self) {
        self.position += 1;
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
        if self.position >= self.length {
            None
        } else {
            Some(&self.input[self.position])
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
        let from = self.position + i;
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
        if self.position >= self.length {
            return (self.length, self.length);
        }

        let mut until = self.position + n;

        if until >= self.length {
            until = self.length;
        }

        (self.position, until)
    }
}
