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
ara_parser = "0.1.0"
```

## Example

```rust
use ara_parser::parser;
use ara_reporting::builder::Charset;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_source::loader::load_directories;

fn main() -> Result<(), Error> {

    let root = "/path/to/ara/project";

    let source_map = load_directories(root, vec![&format!("{}/{}", root, "src/")]).unwrap();

    match parser::parse_map(&source_map) {
        Ok(tree_map) => tree_map.trees.iter().for_each(|tree| {
            println!("{:#?}", tree.definitions);
        }),
        Err(report) => {
            ReportBuilder::new(&source_map, report)
                .with_charset(Charset::Unicode)
                .with_colors(ColorChoice::Always)
                .print()?;
        }
    }

    Ok(())
}
```

## Syntax

```ara
type num = int|float;

final readonly class Collection<T> {
    public function __construct(
        private vec<T> $items = vec[],
    ) {}

    public function filter((fn(T): bool) $func): Collection<T> {
        $result = vec[];
        foreach $this->items as $item {
            if $func($item) {
                $result[] = $item;
            }
        }

        return new Collection::<T>($result);
    }

    public function map<Tout>((fn(T): Tout) $func): Collection<Tout> {
        $result = vec[];
        foreach $this->items as $item {
            $result[] = $func($item);
        }

        return new Collection::<Tout>($result);
    }

    public function sum(): int where T is num {
        $result = 0;
        foreach $this->items as $item {
            $result += $item;
        }

        $result
    }
}

function example(): int {
    $collection = new Collection::<num>(vec[
        1, 2, 3.0, 4.0, 5, 6, 7.0, 9.0, 10
    ]);

    $collection
      ->filter(fn(num $n): bool => $n < 8)
      ->map::<float>(
        fn(num $n): float => $n is float ? $n : $n + 0.0
      )
      ->sum()
}
```

### Diffrerences from PHP

`Ara` syntax is similar to PHP, with a handful of notable differences, the following is a list of the features that `Ara` does not support from PHP:

- [x] **No opening, or closing tags** - `Ara` does not allow opening, or closing tags, so you can't use `<?php`, `<?=`, `<?`, or `?>` syntax.
- [x] **No top-level statement** - `Ara` does not allow top-level code, so you can't use code outside of a class, or function.
- [x] **No `switch`** - `Ara` does not allow `switch` statements, use `if` or `match` instead.
- [x] **No type casting** - `Ara` does not allow type casting, so you can't use `(int)`, `(float)`, `(string)`, `(array)`, `(object)`, `(bool)` syntax.
- [x] **No dynamic identifiers** - `Ara` does not allow dynamic identifiers, so you can't use `$foo->{$a}();` syntax.
- [x] **No variable identifiers** - `Ara` does not allow variable identifiers, so you can't use `$foo->$bar();` syntax
- [x] **No dynamic variables** - `Ara` does not allow dynamic variables, so you can't use `${$a}` syntax.
- [x] **No variable variables** - `Ara` does not allow variable-variables, so you can't use `$$a` syntax.
- [x] **No heredoc** - `Ara` does not allow heredoc, so you can't use `<<<` syntax.
- [x] **No nowdoc** - `Ara` does not allow nowdoc, so you can't use `<<<'` syntax.
- [x] **No interpolation** - `Ara` does not allow interpolation, so you can't use `${}` syntax.
- [x] **No string interpolation** - `Ara` does not allow string interpolation, so you can't use variables, or arbitrary expressions inside strings.
- [x] **No backticks** - `Ara` does not allow backticks as an alias for `shell_exec`, so you can't use `` $result = `command -arg`; `` syntax.
- [x] **No `include`** - `Ara` does not allow `include`, so you can't use `include` or `include_once` syntax.
- [x] **No `require`** - `Ara` does not allow `require`, so you can't use `require` or `require_once` syntax.
- [x] **No `eval`** - `Ara` does not allow `eval`, so you can't use `eval` syntax.
- [x] **No `die`** - `Ara` does not allow `die`, so you can't use `die` syntax.  ( note: `exit` is allowed )
- [x] **No goto** - `Ara` does not allow `goto`, so you can't use `goto` syntax.
- [x] **No declare(...)** - `Ara` does not allow `declare(...)` statements, so you can't use `declare(ticks=1)` syntax.
- [x] **No `if(...): ... endif;`** - `Ara` does not allow `if(...): ... endif;` syntax, use `if(...) { ... }` instead.
- [x] **No `for(...): ... endfor;`** - `Ara` does not allow `for(...): ... endfor;` syntax, use `for(...) { ... }` instead.
- [x] **No `foreach(...): ... endforeach;`** - `Ara` does not allow `foreach(...): ... endforeach;` syntax, use `foreach(...) { ... }` instead.
- [x] **No `while(...): ... endwhile;`** - `Ara` does not allow `while(...): ... endwhile;` syntax, use `while(...) { ... }` instead.
- [x] **No traits** - `Ara` does not have `trait`s.
    > **Note:** Trait support is planned for the future, once the language is more stable.
- [x] **No `__TRAIT__`** - `Ara` does not allow `__TRAIT__`, so you can't use `__TRAIT__` syntax.
- [x] **No `__halt_compiler`** - `Ara` does not allow `__halt_compiler`, so you can't use `__halt_compiler` syntax.
- [x] **No `__COMPILER_HALT_OFFSET__`** - `Ara` does not allow `__COMPILER_HALT_OFFSET__`, so you can't use `__COMPILER_HALT_OFFSET__` syntax.
- [x] **No `var`** - `Ara` does not allow `var` properties.
- [x] **No `global`** - `Ara` does not allow `global` variables.
- [x] **No `static` variables** - `Ara` does not allow `static` variables, `static` class properties, and methods are allowed.
- [x] **No `empty`** - `Ara` does not allow `empty` syntax for checking if a variable is empty, the type of a variable in `Ara` always known, so you can use `if $foo { ... }` for boolean checks, `if $foo != '' { ... }` for string checks, `if $foo != 0 { ... }` for integer checks, ..etc.
- [x] **No `@`** - `Ara` does not allow `@` syntax for suppressing errors.
- [x] **No `list(...)`** - `Ara` does not allow `list(...)` syntax for destructuring arrays.
- [x] **No `array(...)`** - `Ara` does not allow `array(...)` syntax for creating arrays.
- [x] **No `[...]`** - `Ara` does not allow `[...]` syntax for creating arrays.
- [x] **No `isset(...)`** - `Ara` does not allow `isset(...)` syntax for checking if a variable is set.
- [x] **No `unset(...)`** - `Ara` does not allow `unset(...)` syntax for unsetting a variable.
- [x] **No single statement bodies** - `Ara` does not allow single statement bodies, so you can't use `if (...) $a = 1;` syntax, use `if (...) { $a = 1; }` instead.
- [x] **No `callable` type** - `Ara` does not allow `callable` type, use `(fn(T): R)` type instead.
- [x] **Required types** - `Ara` requires types for all properties, parameters, and return types, so you can't use `function foo($a) { ... }` syntax, use `function foo(int $a): void { ... }` instead.
- [x] **No grouped `use`** - `Ara` does not allow grouped `use` statements, so you can't use `use Foo\{Bar, Baz};` syntax.
- [x] **No `echo`, `print`** - `Ara` does not allow `echo`, or `print`, use `Psl\IO\write(..)`, `Psl\IO\write_line(..)`, `Psl\IO\write_error(..)`, or `Psl\IO\write_error_line(..)` instead.
- [x] **Required parentheses for `new`** - `Ara` requires parentheses for `new` expression, so you can't use `new Foo` syntax, use `new Foo()` instead.

    > **Note:** this also applies to `new static`, `new self`, and `new parent`, and `new class { ... }`
    > use `new static()`, `new self()`, `new parent()`, and `new class() { ... }` instead.

### Features

`Ara` introduces the following features:

- [x] **`vec[]`** - `Ara` allows `vec[]` syntax for creating vectors.

    > **Note:** `vec[]` is a type-safe alternative to `[]`, it is compiled to `[]` at runtime.

    Examples:

    ```ara
    $vec = vec[1, 2, 3];
    ```

- [x] **`dict[]`** - `Ara` allows `dict[]` syntax for creating dictionaries.

    > **Note:** `dict[]` is a type-safe alternative to `[]`, it is compiled to `[]` at runtime.

    Examples:

    ```ara
    $dict = dict['foo' => 1, 'bar' => 2];
    ```

- [x] **generics** - `Ara` allows generics, so you can use `function foo<T>(T $value): T { ... }` syntax.

    > **Note:** generics are erased at runtime, and are only used for type checking, and type inference, parameters and return types are compiled to the type they are constrained to, or `mixed` if no constraint is provided.

    Examples:

    ```ara
    function foo<T>(T $value): T {
        $value
    }

    function bar(): int {
        foo::<int>(1)
    }
    ```

    Generics can also be used with classes, and interfaces as well.

    Examples:

    ```ara
    interface ContainerInterface<T> {
        public function get(): T;
    }

    final readonly class Container<T> implements ContainerInterface<T> {
        public function __construct(private T $value) {}

        public function get(): T {
            $this->value
        }
    }
    ```

    Template constraints can be used to constrain the type of a generic template.

    - `T as sometype`, meaning that `T` must be a subtype of `sometype`

    Examples:

    ```ara
    function add<T as int|float>(T $first, T $second): T {
        $first + $second
    }

    function example(): void {
        $a = add::<int>(1, 2); // OK - int is a subtype of int|float
        $b = add::<float>(1.0, 2.0); // OK - float is a subtype of int|float
        $c = add::<int|float>(1, 2.0); // OK - int|float is a subtype of int|float
        $d = add::<int|float>(1.0, 2); // OK - int|float is a subtype of int|float
        $e = add::<string>('1', '2'); // ERROR - string is not a subtype of int|float
    }
    ```

    Template covariance

    - `+T`, meaning that `T` is covariant

    If no variance annotation is provided, the type parameter is invariant.

    For example, consider the following:

    ```ara
    interface Animal {
        public function talk(): string;
    }

    final class Dog implements Animal {
        public function talk(): string { 'woof' }
    }

    final class Cat implements Animal {
        public function talk(): string { 'miaow' }
    }

    interface Collection<T> extends Iterator<int, T> {}

    function take_animals(Collection<Animal> $animals): void {
        foreach $animals as $animal {
            echo $animal->talk();
        }
    }

    function take_dogs(Collection<Dog> $dogs): void {
        take_animals($dogs); // ERROR - expected Collection<Animal>, got Collection<Dog>
    }
    ```

    This results in an error, because `Collection<Dog>` is not a subtype of `Collection<Animal>`.

    We can fix this by making `Collection` covariant, telling `Ara` that is it safe to pass subtypes for the templated param T.

    ```ara
    interface Animal {
        public function talk(): string;
    }

    final class Dog implements Animal {
        public function talk(): string { 'woof' }
    }

    final class Cat implements Animal {
        public function talk(): string { 'miaow' }
    }

    interface Collection<+T> extends Iterator<int, T> {}

    function take_animals(Collection<Animal> $animals): void {
        foreach $animals as $animal {
            echo $animal->talk();
        }
    }

    function take_dogs(Collection<Dog> $dogs): void {
        take_animals($dogs); // OK - Collection<Dog> is a subtype of Collection<Animal>
    }
    ```

- [x] **`(fn(T1, T2, ...): R)` type** - `Ara` has `(fn(T1, T2, ...): R)` type as a type-safe alternative to `callable`.

    Examples:

    ```ara
    function map<T, R>(vec<T> $items, (fn(T): R) $fn): vec<R> {
        $result = vec[];
        foreach $items as $item {
            $result[] = $fn($item);
        }

        $result
    }

    function example(): void {
        $items = vec[1, 2, 3];
        $result1 = map($items, fn(int $item): int => $item * 2); // OK
        $result2 = map($items, fn(string $item): string => $item); // Type error
    }
    ```

- [x] **type aliases** - `Ara` allows type aliases, so you can use `type scalar = int | float | string | bool;` syntax.

    Examples:

    ```ara
    type scalar = int | float | string | bool;

    function to_scalar(mixed $value): scalar {
        $value as scalar
    }
    ```

- [x] **`is`** - `Ara` allows `is` syntax for type checks, so you can use `$foo is string` syntax.

    Examples:

    ```ara
    function to_integer(mixed $value): int {
        if $value is int {
            return $value;
        }

        throw new InvalidArgumentException('Expected an integer.');
    }
    ```

- [x] **`as`** - `Ara` allows `as` syntax for type assertions, so you can use `$foo as string` syntax.

    > **Note:** `as` would result in a runtime error if the type assertion fails, so it's not recommended to use it in performance critical code.

    Examples:

    ```ara
    function to_integer(mixed $value): int {
        $value as int
    }
    ```

    `as` assertion can also be used within a `foreach` loop:

    ```ara
    function example(mixed $items): void {
        foreach $items as vec<int> as $index => $value {
            do_something($index, $value);
        }
    }
    ```

- [x] **`where`** - `Ara` adds support for `where` type constraint on methods.

    > **Note:** `where` type constraints are enforced at compile time, not at runtime.
    > **Note:** `where` can only be used on methods, not on functions.

    Examples:

    ```ara
    use Psl\Str;

    final class Box<T> {
        public function __construct(public T $value) { }

        public function length(): int where T is string {
            Str\length($this->value)
        }
    }

    function example(): void {
        $box = new Box::<string>('hello world');
        $box->length(); // OK

        $box = new Box::<int>(123);
        $box->length(); // Type error, method `Box<T = int>::length` requires `T` to be `string`, but `int` was provided.
    }
    ```

    A method can also have more than one `where` type constraint.

    ```ara
    use Psl\Str;

    final class Foo<T, U> {
        public function __construct(public T $value, public U $other) { }

        public function bar(): int where
            T is string,
            U is int,
        {
            Str\length($this->value) + $this->other
        }
    }

    function example(): void {
        $foo = new Foo::<string, int>('hello world', 123);
        $foo->bar(); // OK

        $foo = new Foo::<int, string>(123, 'hello world');
        $foo->bar(); // Type error, method `Foo::<T = int, U = string>` requires `T` to be `string`, but `int` was provided.
    }
    ```

- [x] **`(1, 2, 3)`** - `Ara` allows `(1, 2, 3)` syntax for creating tuples.

    > **Note:** `(1, 2, 3)` is a type-safe alternative to `[]`, it is compiled to `[]` at runtime.

    Examples:

    ```ara
    $tuple = (1, 2, 3);
    ```

- [x] **`async`** - `Ara` allows `async` syntax for running functions asynchronously.

    > **Note:** `async` expressions are compiled to `Psl\Async\run(...)` at runtime.

    Examples:

    ```ara
    function do_something(): int {
        // ...

        return 1;
    }

    function do_something_async(): Awaitable<int> {
        $result = async do_something();

        $result
    }
    ```

- [x] **`await`** - `Ara` allows `await` syntax for awaiting asynchronous expressions.

    > **Note:** `await` expressions are compiled to `Psl\Async\await(...)` at runtime.

    Examples:

    ```ara
    function something(): Awaitable<int> {
        // ...

        return $result;
    }

    function wait_for_something(): int {
        $result = await something();

        $result
    }
    ```

- [x] **`concurrently`** - `Ara` allows `concurrently` syntax for running asynchronous expressions concurrently.

    > **Note:** `concurrently` expressions are compiled to `Psl\Async\concurrently(...)` at runtime.

    Examples:

    ```ara
    function something(): Awaitable<int> {
        // ...

        return $result;
    }

    function something_else(): Awaitable<int> {
        // ...

        return $result;
    }

    function everything(): (int, int) {
        ($a, $b) = concurrently {
            await something(),
            await something_else(),
        };

        ($a, $b)
    }
    ```

- [x] **implict `return`** - `Ara` allows implicit `return` syntax for functions, so you can use `function foo(): int { 1 }` syntax.

    > **Note:** implicit return expressions are compiled to `return` at runtime.

    Examples:

    ```ara
    function foo(): int {
        1
    }
    ```

- [x] **optional condition parentheses** - `Ara` allows optional parentheses around conditions, so you can use `if $foo { ... }` syntax.

    > **Note:** optional parentheses are compiled to `(...)` at runtime.

    Examples:

    ```ara
    if $foo {
        // ...
    } else if $bar {
        // ...
    } else {
        // ...
    }

    while $foo {
        // ...
    }

    do {
        // ...
    } while $foo;

    for $foo = 1; $foo < 10; $foo++ {
        // ...
    }

    foreach $foo as $bar {
        // ...
    }

    foreach $foo as $bar => $baz {
        // ...
    }
    ```

- [x] **`nonnull` type** - `Ara` adds support for `nonnull` type.

    Examples:

    ```ara
    function example(nonnull $value): void {
        // ...
    }
    ```

- [x] **`resource` type** - `Ara` adds support for `resource` type.

    Examples:

    ```ara
    function example(resource $value): void {
        // ...
    }
    ```


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
