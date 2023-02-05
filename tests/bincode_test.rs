use std::io;

use ara_source::source::Source;
use ara_source::source::SourceKind;

use ara_parser::parser;
use ara_parser::tree::Tree;

#[test]
fn test_bincode() -> io::Result<()> {
    let code = r#"
        function println(string $format, string|int|float ...$args): void {
            printf($format, ...$args);
            printf("\n");
        }

        function main(): void {
            $a = 1;
            $b = 2;
            $c = $a + $b;

            println("a = %d", $a);
            println("b = %d", $b);
            println("c = a + b = %d", $c);
        }
    "#;

    let source = Source::inline(SourceKind::Script, code);
    let tree = parser::parse(&source).unwrap();

    let config = bincode::config::standard();
    let encoded_tree = bincode::encode_to_vec(&tree, config).unwrap();
    let decoded_tree = bincode::decode_from_slice::<Tree, _>(&encoded_tree[..], config)
        .unwrap()
        .0;

    dbg!(encoded_tree);
    dbg!(decoded_tree);

    Ok(())
}
