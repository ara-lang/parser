#[macro_export]
macro_rules! parser_report {
    ($state:expr, $issue:ident($($args:expr),+$(,)?)$(,)?) => {
        {
            let issue = $crate::parser::issue::$issue($state, $($args,)+);

            $state.record(issue);
        }
    };
    ($state:expr, $issue:ident) => {
        {
            let issue = $crate::parser::issue::$issue($state);

            $state.record(issue);
        }
    };
}

#[macro_export]
macro_rules! parser_bail {
    ($state:expr, $issue:ident($($args:expr),+$(,)?)$(,)?) => {
        {
            let issue = $crate::parser::issue::$issue($state, $($args,)+);

            return Err(Box::new($state.report(issue)));
        }
    };
    ($state:expr, $issue:ident) => {
        {
            let issue = $crate::parser::issue::$issue($state);

            return Err(Box::new($state.report(issue)));
        }
    };
}
