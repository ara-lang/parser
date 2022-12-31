use ara_reporting::Report;

pub type ParseResult<T> = Result<T, Box<Report>>;
