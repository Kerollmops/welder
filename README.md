# welder
A tool to help concatenate, implemented with a builder pattern

## Examples

```rust
let welder = Welder::start(' ', "foo");

let welder = welder.push("bar");
let welder = welder.push("baz");
let welder = welder.push("boat");

let string: String = welder.weld();

assert_eq!("foo bar baz boat", &string);
```

```rust
let welder = Welder::start(0, 12);

let vec: Vec<_> = welder.push(14)
                        .push(16)
                        .push(18)
                        .weld();

assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
```
