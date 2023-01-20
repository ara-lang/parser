use std::env;

use ara_parser::parser;
use ara_parser::traverser::visitor::NodeVisitor;
use ara_parser::traverser::TreeTraverser;
use ara_parser::tree::definition::function::FunctionLikeParameterDefinition;
use ara_parser::tree::downcast;
use ara_parser::tree::Node;
use ara_reporting::annotation::Annotation;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_source::loader::FileSourceLoader;
use ara_source::loader::SourceLoader;

struct NoVariadicParameterRuleVisitor;

impl NodeVisitor<Issue> for NoVariadicParameterRuleVisitor {
    fn visit(
        &mut self,
        source: &str,
        node: &dyn Node,
        _parent: Option<&dyn Node>,
    ) -> Result<(), Issue> {
        if let Some(parameter) = downcast::<FunctionLikeParameterDefinition>(node) {
            if let Some(position) = parameter.ellipsis {
                let issue = Issue::warning("some-code", "variadic parameters are forbidden")
                    .with_source(source, position, position + 3)
                    .with_annotation(Annotation::secondary(
                        source,
                        parameter.initial_position(),
                        parameter.final_position(),
                    ));

                return Err(issue);
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let loader = FileSourceLoader::new(&cargo_manifest_dir);

    let source_map = loader.load(&"examples/project/format.ara").unwrap();

    match parser::parse_map(&source_map) {
        Ok(tree_map) => {
            let mut traverser =
                TreeTraverser::new(vec![Box::new(NoVariadicParameterRuleVisitor {})]);

            match traverser.traverse(&tree_map) {
                Ok(_) => {}
                Err(issues) => {
                    let report = Report {
                        issues,
                        footer: None,
                    };
                    ReportBuilder::new(&source_map)
                        .with_charset(CharSet::Unicode)
                        .with_colors(ColorChoice::Always)
                        .print(&report)
                        .unwrap();
                }
            }
        }
        Err(report) => {
            ReportBuilder::new(&source_map)
                .with_charset(CharSet::Unicode)
                .with_colors(ColorChoice::Always)
                .print(report.as_ref())?;
        }
    }

    Ok(())
}
