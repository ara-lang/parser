use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use ara_parser::parser;
use ara_parser::tree::Tree;
use ara_source::source::Source;
use ara_source::source::SourceKind;

fn decode_benchmark(criterion: &mut Criterion) {
    let source = Source::inline(SourceKind::Script, CODE_SAMPLE);
    let tree = parser::parse(&source).unwrap();
    let config = bincode::config::standard();
    let encoded_tree = bincode::encode_to_vec(&tree, config).unwrap();

    criterion
        .bench_function("decode tree", |bencher| {
            bencher.iter(|| {
                black_box(bincode::decode_from_slice::<Tree, _>(
                    black_box(&encoded_tree[..]),
                    black_box(config),
                ))
            })
        })
        .bench_function("parse code", |bencher| {
            bencher.iter(|| black_box(parser::parse(black_box(&source))))
        });
}

criterion_group!(benches, decode_benchmark);
criterion_main!(benches);

static CODE_SAMPLE: &str = r#"
    namespace A\B\C\D\E;

    function foo(string $s): self {
        exit(0);
    }

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

    interface Foo<S> {
        public static function baz<T, U>(T $a, U $b): Q<T, (U, T)> where
            S is T|U,
            S is nonnnull
        ;
    }

    final class Bar implements Foo<string> {
        public static function baz<T, U>(T $a, U $b): Q<T, (U, T)>
        {
            return new Q::<T, (U, T)>($a, ($b, $a));
        }
    }

    final class Q<T, U> {
        public function __construct(
            public readonly T $a,
            public readonly U $b
        ) {}
    }

    function baz<T, U>(T $a, U $b): Q<T, (U, T)> {
        return new Q::<T, (U, T)>($a, ($b, $a));
    }

    function foo(): void {
        $a?->b($c);
    }

    function q(): void {
        $a = 2 ** 2;

        $b = 1 ? 2 : 3;

        $c = 1 ? 2 ? 3 : 4 : 5;

        $d = 1 ?: 2 ?: 3;

        $e = 1 ?? 2;

        $f = 1 ?? 2 ?? 3;
    }

    function h(): void {
        $foo['bar'];
        $foo['bar']['baz'];
        $foo['bar'] = 'baz';
    }

    function q(): void {
        $a = new Foo();
        $b = +1;
        $c = ~2;
        $d = --$b;
        $e = ++$d;
    }

    #[A, B]
    #[C, D]
    interface A extends B, C {
        #[R]
        const int F = 344;

        #[R]
        public const int O = 344;

        #[R]
        #[P]
        final public const int R = 344;

        #[R]
        #[P]
        final const int M = 34;

        #[M]
        public function bar(): void;

        #[Q]
        #[S]
        public static function baz(): void;
    }

    interface A extends B, C {
        #[R]
        const u16 F = 344;

        #[R]
        public const u16 O = 344;

        #[R]
        #[P]
        final public const u16 R = 344;

        #[R(), P()]
        final public const u16  P = 214;
    }

    const u8 FOO = 1;
    const u8 BAR = FOO;

    type Predicate<T> = Closure<(T), bool>;

    function filter<T>(vec<T> $vec, Predicate<T> $predicate): vec<T> {
        $result = vec[];
        foreach $vec as $item {
            if $predicate($item) {
                $result[] = $item;
            }
        }

        return $result;
    }

    type Mapper<K, V, U> = Closure<(K, V), U>|SomeOtherType<K, V, U>;

    function map<K, V, U>(dict<K, V> $dict, Mapper<K, V, U> $mapper): dict<K, U> {
        $result = dict[];
        if $mapper is SomeOtherType<_, _, _> {
            foreach $dict as $key => $value {
                $result[$key] = $mapper->doMap($key, $value);
            }
        } else {
            foreach $dict as $key => $value {
                $result[$key] = $mapper($key, $value);
            }
        }

        return $result;
    }

    function foo(): void {
        using {

        }

        using if $a {

        }

        using $a = foo() if $a {

        }

        using $a = foo() {

        }
    }

    function main(): void {
        $foo->bar;
        $foo->bar->baz;
    }
"#;
