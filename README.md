# pipa

Pipe Operator Library for Rust.

Features:
- Pipe things into chain of expressions.
- Support different types.
- Support pipe for functions, closures and methods returning `Try`-able types(Option & Result).
- Support pipe for async functions and async methods returning `Try`-able types(Option & Result).
- Support pipe for associated functions returning `Try`-able types(Option & Result).
- Support pipe for async associated functions returning `Try`-able types(Option & Result).
- Support pipe for methods calls returning `Try`-able types(Option & Result).
- Support pipe for async methods calls returning `Try`-able types(Option & Result).
- Support pipe for inline closures.
- Support Railway-Oriented Programming pattern with simple and terse syntax.

Example:
```rust
let ret = pipa!(123 => f => g => h);
let ret = pipa_try!(123 => f? => ? => h?);
let ret = pipa_await_try!(123 => f.await => g.await => h.await);
```

Example mixing multiple kinds of expressions:
```rust
let clo = |v: i32| -> i32 { v * 5 };

let result = p!(5
    => func
    => clo
    => func_rop?
    => func_2.await
    => func_2_rop.await?
    => obj.method
    => func
    => obj.method_rop?
    => func_rop?
    => obj.method_async.await
    => obj.method_async_rop.await?
);

let ret = p!(5
    => D::anu
    => D::anu_try?
    => D::async_anu.await
    => D::async_anu_try.await?
);
assert_eq!(ret, 9000);

let ret = p!(5
    => E::anu
    => E::anu_try?
    => E::async_anu.await
    => E::async_anu_try.await?
);
assert_eq!(ret, 3000);

// inline closure tests
let ret = p!(5 => |i32| 2 => |n: i32| n + 5);
assert_eq!(ret, 7);
dbg!(&ret);

let ret = p!(5 => |n: i32| n * 2 => |n: i32| n + 5);
assert_eq!(ret, 15);
dbg!(&ret);

let ret = p!(5 => |i32| {2} => |n: i32| n + 5);
assert_eq!(ret, 7);
dbg!(&ret);

let ret = p!(5 => |n: i32| {n * 2} => |n: i32| n + 5);
assert_eq!(ret, 15);
dbg!(&ret);

let ret = p!(5 => |i32| -> i32 {2} => |n: i32| n + 5);
assert_eq!(ret, 7);
dbg!(&ret);

let ret = p!(5 => |n: i32| -> i32 { n * 2 } => |n: i32| n + 5);
assert_eq!(ret, 15);
dbg!(&ret);

let ret = p!(2 => func_2.await => |n: i32| -> i32 { n * 4 } => func => |x: i32| x - 2 => |y: i32| -> f32 { (y / 4) as f32 } );
assert_eq!(ret, 15_f32);
dbg!(&ret);

```

More detail in `tests/test.rs` & `tests/mixed_test.rs`.
