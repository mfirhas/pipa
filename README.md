# pipa

Pipe Operator Library for Rust.

Features:
- Pipe things into chain of functions.
- Support different types.
- ...

Example:
```rust
fn f(a: i32) -> u64 {
    (a + 1) as u64
}

fn g(a: u64) -> String {
    (a + 1).to_string()
}

fn h(s: String) -> u64 {
    let ret = s.parse::<u64>();
    ret.unwrap() + 10
}

#[test]
fn pipe() {
    let ret = pipa!(123 => f => g => h);
    assert_eq!(135, ret);
}
```
