use std::env;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use pretty_assertions::assert_str_eq;

use ara_parser::parser;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_source::loader::FileSourceLoader;
use ara_source::loader::SourceLoader;

#[test]
fn test_fixtures() -> io::Result<()> {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = format!("{manifest}/tests/samples/");

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

        let source_map = loader.load(&code_filename).unwrap();
        match parser::parse(&source_map.sources[0]) {
            Ok(tree) => {
                let expected_tree = std::fs::read_to_string(&tree_filename)?;

                assert_str_eq!(
                    expected_tree,
                    format!("{:#?}", tree.definitions),
                    "tree mismatch for sample `{}`",
                    source_map.sources[0].name()
                );

                assert!(
                    !error_filename.exists(),
                    "found `error.txt` for `{}` but was expected.",
                    source_map.sources[0].name()
                );
            }
            Err(report) => {
                let builder = ReportBuilder::new(&source_map)
                    .with_charset(CharSet::Ascii)
                    .with_colors(ColorChoice::Never);

                let error = builder.as_string(report.as_ref()).unwrap();

                let expected_error = std::fs::read_to_string(&error_filename)?;

                assert_str_eq!(
                    expected_error,
                    format!("{error}"),
                    "error mismatch for sample `{}`",
                    source_map.sources[0].name()
                );

                assert!(
                    !tree_filename.exists(),
                    "found `tree.txt` for `{}` but was expected.",
                    source_map.sources[0].name()
                );
            }
        }
    }

    Ok(())
}
