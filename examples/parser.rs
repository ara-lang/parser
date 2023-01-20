use std::env;

use ara_parser::parser;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_source::loader::load_directories;

fn main() -> Result<(), Error> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let source_map = load_directories(cargo_manifest_dir, vec!["examples/project/"]).unwrap();

    match parser::parse_map(&source_map) {
        Ok(tree_map) => tree_map.trees.iter().for_each(|tree| {
            println!("{:#?}", tree.definitions);
        }),
        Err(report) => {
            ReportBuilder::new(&source_map)
                .with_charset(CharSet::Unicode)
                .with_colors(ColorChoice::Always)
                .print(report.as_ref())?;
        }
    }

    Ok(())
}
