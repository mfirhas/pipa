use std::time::Duration;

use pipa::p;

fn double_it(n: i32) -> i32 {
    n * 2
}

fn validate(n: i32) -> Result<i32, &'static str> {
    if n > 5 {
        return Ok(n);
    }
    Err("validation error: too small")
}

fn add_one(n: i32) -> i32 {
    n + 1
}

fn parse_to_string(n: i32) -> String {
    n.to_string()
}

async fn send(s: String) -> Result<String, &'static str> {
    tokio::time::sleep(Duration::from_millis(800)).await;
    Ok(format!("{} SENT SUCCESSFULLY!", s))
}

async fn send_2(s: Result<String, &'static str>) -> Result<String, &'static str> {
    tokio::time::sleep(Duration::from_millis(200)).await;
    if s.is_ok() {
        return Ok(format!("{} SENT_2 SUCCESSFULLY!", s.unwrap()));
    }
    Err("send_2 failed")
}

fn read_status(s: String) -> Result<u8, &'static str> {
    if !s.is_empty() {
        return Ok(1);
    }
    Err("FAILED!!")
}

async fn finalize(x: u8) -> Result<bool, &'static str> {
    if x == 1 {
        return Ok(true);
    }
    Err("FAILED FINALIZED!!")
}

async fn run_all(n: i32) -> Result<(), &'static str> {
    let ret = p!(
        double_it(n)
        => validate?
        => add_one
        => parse_to_string
        => send.await
        => send_2.await?
        => read_status?
        => finalize.await?
    );
    if ret {
        return Ok(());
    }
    Err("all failed")
}

#[tokio::test]
async fn test_mixed() {
    let n = 10;
    let ret = run_all(n).await;
    println!("RESULT of PIPE => {:?}", ret);
    assert!(ret.is_ok());
}

#[test]
fn test_method() {
    struct Anu {
        s: i32,
        d: String,
    }

    impl Anu {
        fn method(&self, n: i32) -> String {
            format!("{}-{}-{}", self.s, self.d, n)
        }
    }

    let anu = Anu {
        s: 123,
        d: String::from("this"),
    };

    fn asd(s: String) -> String {
        s
    }

    let ret = p!(anu.method(124) => asd);
    assert_eq!(ret, "123-this-124")
}

#[tokio::test]
async fn test_await_try() {
    let ret = run_done().await;
    dbg!(&ret);
    assert!(ret.is_ok());
}

async fn run_done() -> Result<(), &'static str> {
    let ret = p!(123 => run.await? => done.await?);
    Ok(ret)
}

async fn run(n: u32) -> Result<String, &'static str> {
    Ok(n.to_string())
}

async fn done(s: String) -> Result<(), &'static str> {
    if s == "123" {
        return Ok(());
    }
    Err("failed done")
}

// ALL
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

struct D;

impl D {
    fn anu(n: i32) -> String {
        format!("-> {}", n)
    }

    fn anu_try(s: String) -> Result<i32, &'static str> {
        if !s.is_empty() {
            return Ok(9000);
        }
        Err("failed anu try")
    }

    async fn async_anu(s: i32) -> u32 {
        s as u32
    }

    async fn async_anu_try(s: u32) -> Result<u32, &'static str> {
        if s > 0 {
            return Ok(s);
        }
        Err("cannot accept 0")
    }
}

async fn run_them_all(obj: Obj) -> Result<i32, &'static str> {
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

    Ok(result)
}

#[tokio::test]
async fn test_all() {
    let obj = Obj { val: 5 };

    let ret = run_them_all(obj).await;
    dbg!(&ret);

    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 72180);
}
