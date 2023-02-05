# `Ara` Parser

[![Actions Status](https://github.com/ara-lang/parser/workflows/ci/badge.svg)](https://github.com/ara-lang/parser/actions)
[![Crates.io](https://img.shields.io/crates/v/ara_parser.svg)](https://crates.io/crates/ara_parser)
[![Docs](https://docs.rs/ara_parser/badge.svg)](https://docs.rs/ara_parser/latest/ara_parser/)

A fault-tolerant, recursive-descent parser for `Ara` Programming Language 🌲

> **Note:** This project is a hard-fork of [`php-rust-tools/parser`](https://github.com/php-rust-tools/parser)
>
> Special thanks to the original [authors](https://github.com/php-rust-tools/parser/graphs/contributors) for their work.

---

## Usage

Add `ara_parser` to your `Cargo.toml`, and you're good to go!

```toml
[dependencies]
ara_parser = "0.6.6"
```

## Example

```rust
use ara_parser::parser;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_source::loader::load_directories;

fn main() -> Result<(), Error> {
   let source_map = load_directories("/path/to/project", vec!["src/"]).unwrap();

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
```

## Documentation

See the [documentation](https://ara-lang.io) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

* [Saif Eddin Gmati](https://github.com/azjezz)
* [All contributors](https://github.com/ara-lang/parser/graphs/contributors)
