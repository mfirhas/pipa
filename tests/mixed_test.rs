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

fn read_status(s: Result<String, &'static str>) -> Result<u8, &'static str> {
    if s.is_ok() {
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
        => read_status?
        => finalize.await
    )?;
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
