use std::env;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use ara_parser::parser;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_source::loader::FileSourceLoader;
use ara_source::loader::SourceLoader;

fn main() -> io::Result<()> {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = format!("{}/tests/samples/", manifest);

    let mut entries = read_dir(&root)?
        .flatten()
        .map(|entry| entry.path())
        .filter(|entry| entry.is_dir())
        .collect::<Vec<PathBuf>>();

    entries.sort();

    let loader = FileSourceLoader::new(&root);
    for entry in entries {
        let code_filename = entry.join("code.ara");
        let tree_filename = entry.join("tree.txt");
        let error_filename = entry.join("error.txt");

        if !code_filename.exists() {
            continue;
        }

        if tree_filename.exists() {
            std::fs::remove_file(&tree_filename)?;
        }

        if error_filename.exists() {
            std::fs::remove_file(&error_filename)?;
        }

        let source_map = loader.load(&code_filename).unwrap();
        match parser::parse(&source_map.sources[0]) {
            Ok(tree) => {
                std::fs::write(tree_filename, format!("{:#?}", tree.definitions))?;
                println!(
                    "✅ generated `tree.txt` for `{}`",
                    source_map.sources[0].name()
                );
            }
            Err(report) => {
                let builder = ReportBuilder::new(&source_map, *report)
                    .with_charset(CharSet::Ascii)
                    .with_colors(ColorChoice::Never);

                let error = builder.as_string().unwrap();

                std::fs::write(error_filename, error)?;

                println!(
                    "✅ generated `error.txt` for `{}`",
                    source_map.sources[0].name()
                );
            }
        }
    }

    Ok(())
}
