# pipa

Pipe Operator Library for Rust.

Features:
- Pipe things into chain of expressions.
- Support different types.
- Support pipe for functions and methods returning `Try`-able types(Option & Result).
- Support pipe for async functions and async methods returning `Try`-able types(Option & Result).
- Support pipe for methods calls returning `Try`-able types(Option & Result).
- Support pipe for async methods calls returning `Try`-able types(Option & Result).
- Support Railway-Oriented Programming pattern with simple and terse syntax.

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

Example mixing multiple kinds of expressions:
```rust
struct Obj {
    val: i32,
}

impl Obj {
    fn method(&self, x: i32) -> i32 {
        x + self.val
    }

    fn method_rop(&self, x: i32) -> Result<i32, &'static str> {
        if x > 10 {
            Ok(x + self.val)
        } else {
            Err("Value too small")
        }
    }

    async fn method_async(&self, x: i32) -> i32 {
        x * 2
    }

    async fn method_async_rop(&self, x: i32) -> Result<i32, &'static str> {
        if x > 10 {
            Ok(x * 2)
        } else {
            Err("Value too small")
        }
    }
}

fn func(x: i32) -> i32 {
    x * 2
}

fn func_rop(x: i32) -> Result<i32, &'static str> {
    if x > 10 {
        Ok(x * 3)
    } else {
        Err("Value too small")
    }
}

async fn func_2(x: i32) -> i32 {
    x * 4
}

async fn func_2_rop(x: i32) -> Result<i32, &'static str> {
    if x > 10 {
        Ok(x * 5)
    } else {
        Err("Value too small")
    }
}

async fn run_them_all(obj: Obj) -> Result<i32, &'static str> {
    let clo = |v: i32| -> i32 { v * 5 };

    // chain them all together!
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

    Ok(result)
}
```

More examples are in `tests/test.rs` & `tests/mixed_test.rs`.
