use ara_reporting::Report;
use ara_source::source::Source;
use ara_source::SourceMap;

use crate::lexer;
use crate::lexer::iterator::TokenIterator;
use crate::lexer::token::Token;
use crate::parser::internal::definition;
use crate::parser::state::State;
use crate::tree::Tree;
use crate::tree::TreeMap;

pub mod issue;

pub(in crate::parser) mod internal;
pub(in crate::parser) mod macros;
pub(in crate::parser) mod result;
pub(in crate::parser) mod state;

pub fn parse_map(map: &SourceMap) -> Result<TreeMap, Box<Report>> {
    let mut trees = vec![];
    let mut reports = vec![];

    for source in &map.sources {
        match parse(source) {
            Ok(tree) => trees.push(tree),
            Err(report) => reports.push(report),
        }
    }

    if !reports.is_empty() {
        let mut issues = vec![];
        for mut report in reports {
            issues.append(&mut report.issues);
        }

        Err(Box::new(Report { issues }))
    } else {
        Ok(TreeMap { map, trees })
    }
}

pub fn parse(source: &Source) -> Result<Tree, Box<Report>> {
    let tokens = match lexer::lex(source) {
        Ok(tokens) => tokens,
        Err(issue) => {
            return Err(Box::new(Report {
                issues: vec![*issue],
            }))
        }
    };

    construct(source, &tokens)
}

pub fn construct<'a, 'b>(source: &'a Source, tokens: &'b [Token]) -> Result<Tree<'a>, Box<Report>> {
    let mut iterator = TokenIterator::new(tokens);
    let mut state = State::new(source, &mut iterator);

    let definitions = definition::tree(&mut state)?;

    state.finish(Tree {
        definitions,
        source,
    })
}
