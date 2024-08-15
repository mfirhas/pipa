use pipa::{pipa, pipa_try};

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

fn pipe() {
    let ret = pipa!(123);
    println!("pipe: {}", ret);
    assert_eq!(123, ret);
    let ret = pipa!(123 => f => g => h);
    println!("pipe: {}", ret);
    assert_eq!(135, ret);
}

// ----

fn add_one(x: i32) -> Option<i32> {
    Some(x + 1)
}

fn double(x: i32) -> Option<i32> {
    Some(x * 2)
}

fn sdf() -> Option<i32> {
    let result = pipa_try!(5 => add_one => double);
    Some(result)
}

fn pipe2() {
    let result = sdf().unwrap_or_default();
    println!("pipe2: {}", result);
    assert_eq!(12, result);
}

fn try_add_one(x: i32) -> Result<i32, ()> {
    Ok(x + 1)
}

fn try_double(x: i32) -> Result<i32, ()> {
    Ok(x * 2)
}

fn try_sdf() -> Result<i32, ()> {
    let result = pipa_try!(5 => try_add_one => try_double);
    Ok(result)
}

fn pipe3() {
    let result = try_sdf().unwrap_or_default();
    println!("pipe3: {}", result);
    assert_eq!(12, result);
}

fn main() {
    pipe();
    pipe2();
    pipe3();
}
