# welder
A tool to help concatenate, implemented with a consuming builder pattern.

## Examples

```rust
let welder = Welder::with_start(' ', "foo");

let welder = welder.elem("bar");
let welder = welder.elem("baz");
let welder = welder.elem("boat");

let string: String = welder.weld();

assert_eq!("foo bar baz boat", &string);
```

```rust
let welder = Welder::with_start(0, 12);

let vec: Vec<_> = welder.elem(14)
                        .elem(16)
                        .elem(18)
                        .weld();

assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
```
