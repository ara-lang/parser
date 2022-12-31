use ara_reporting::issue::Issue;

pub type SyntaxResult<T> = Result<T, Box<Issue>>;
