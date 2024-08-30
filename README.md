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
// call functions f taking 123 as its only parameter, and return as g's only parameter and g returns as h's only parameter, and so on.
// `fn f(n: i32) -> T`
// `fn g(n: T) -> U`
// `fn h(n: U) -> V`
// ret == V
let ret = pipa!(123 => f => g => h);

// same as above, except all functions chained appended with `?` as they return Result/Option.
// the T of each Result<T, E> or Option<T> will be input argument for the next functions.
// `fn f(n: i32) -> Result<T, E>`
// `fn g(n: T) -> Result<U, E>`
// `fn h(n: U) -> Result<V, E>`
// ret == V
// Try operator automatically applied in each calls chain.
let ret = pipa_try!(123 => f => g => h);

// same as both above, except functions are async, await-ed, and return Result/Option.
// `async fn f(n: i32) -> Result<T, E>`
// `async fn g(n: T) -> Result<U, E>`
// `async fn h(n: U) -> Result<V, E>`
// ret == V
// Each functions get await-ed and try operator automatically applied in each calls chain.
let ret = pipa_await_try!(123 => f => g => h);
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
