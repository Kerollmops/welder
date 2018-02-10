# welder
A tool to help concatenate, implemented with a builder pattern

## Examples

```rust
let mut welder = Welder::from(' ', "foo");

welder.push("bar");
welder.push("baz");
welder.push("boat");

let string: String = welder.weld();

assert_eq!("foo bar baz boat", &string);
```

```rust
let base = &[12][..];
let mut welder = Welder::from(0, base);

welder.push(14);
welder.push(16);
welder.push(18);

let vec: Vec<_> = welder.weld();

assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
```
