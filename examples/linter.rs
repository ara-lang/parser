use std::env;

use ara_parser::parser;
use ara_parser::traverser::visitor::NodeVisitor;
use ara_parser::traverser::TreeTraverser;
use ara_parser::tree::cast;
use ara_parser::tree::definition::function::FunctionLikeParameterDefinition;
use ara_parser::tree::Node;
use ara_reporting::annotation::Annotation;
use ara_reporting::builder::Charset;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_source::loader::FileSourceLoader;
use ara_source::loader::SourceLoader;
use ara_source::source::Source;

struct NoVariadicParameterRuleVisitor;

impl NodeVisitor<Issue> for NoVariadicParameterRuleVisitor {
    fn visit(
        &mut self,
        source: &Source,
        node: &dyn Node,
        _parent: Option<&dyn Node>,
    ) -> Result<(), Issue> {
        if let Some(parameter) = cast::<FunctionLikeParameterDefinition>(node) {
            if let Some(span) = parameter.ellipsis {
                let issue = Issue::warning(
                    "some-code",
                    "variadic parameters are forbidden",
                    source.name(),
                    span.position,
                    span.position + 3,
                )
                .with_annotation(Annotation::new(
                    source.name(),
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

    let loader = FileSourceLoader::new(format!("{}/", cargo_manifest_dir));

    let source_map = loader
        .load(&format!(
            "{}/{}",
            cargo_manifest_dir, "examples/project/format.ara"
        ))
        .unwrap();

    match parser::parse_map(&source_map) {
        Ok(tree_map) => tree_map.trees.iter().for_each(|tree| {
            let mut traverser =
                TreeTraverser::new(tree, vec![Box::new(NoVariadicParameterRuleVisitor {})]);

            match traverser.traverse() {
                Ok(_) => {}
                Err(report) => {
                    ReportBuilder::new(&source_map, Report { issues: report })
                        .with_charset(Charset::Unicode)
                        .with_colors(ColorChoice::Always)
                        .print()
                        .unwrap();
                }
            }
        }),
        Err(report) => {
            ReportBuilder::new(&source_map, *report)
                .with_charset(Charset::Unicode)
                .with_colors(ColorChoice::Always)
                .print()?;
        }
    }

    Ok(())
}
