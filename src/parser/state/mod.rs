use std::fmt::Display;

use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_source::source::Source;

use crate::lexer::iterator::TokenIterator;
use crate::lexer::token::Token;
use crate::parser::issue;
use crate::parser::result::ParseResult;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::identifier::Identifier;

#[derive(Debug)]
pub struct State<'a> {
    pub source: &'a Source,
    pub iterator: &'a mut TokenIterator<'a>,
    pub namespace: Option<Identifier>,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub issues: Vec<Issue>,
    pub ignored_shift_at: Option<&'a Token>,
}

impl<'a> State<'a> {
    pub fn new(source: &'a Source, iterator: &'a mut TokenIterator<'a>) -> Self {
        Self {
            source,
            iterator,
            namespace: None,
            attributes: vec![],
            issues: vec![],
            ignored_shift_at: None,
        }
    }

    pub fn attribute(&mut self, attr: AttributeGroupDefinition) {
        self.attributes.push(attr);
    }

    pub fn get_attributes(&mut self) -> Vec<AttributeGroupDefinition> {
        let mut attributes = vec![];

        std::mem::swap(&mut self.attributes, &mut attributes);

        attributes
    }

    pub fn record(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    pub fn report(&mut self, issue: Issue) -> Report {
        if let Some(token) = self.ignored_shift_at {
            issue::report!(self, unexpected_token(vec![">".to_string()], token));
        }

        let mut issues = vec![];

        std::mem::swap(&mut self.issues, &mut issues);

        Report {
            issues: issues.into_iter().chain(std::iter::once(issue)).collect(),
        }
    }

    pub fn finish<T>(&mut self, item: T) -> ParseResult<T> {
        if let Some(token) = self.ignored_shift_at {
            issue::report!(self, unexpected_token(vec![">".to_string()], token));
        }

        if self.issues.is_empty() {
            Ok(item)
        } else {
            Err(Box::new(Report {
                issues: self.issues.drain(..).collect(),
            }))
        }
    }

    pub fn namespace(&mut self, namespace: Identifier) {
        self.namespace = Some(namespace);
    }

    pub fn named<T: Display + ?Sized>(&self, name: &T) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}\\{}", namespace, name)
        } else {
            name.to_string()
        }
    }
}
