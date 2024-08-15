const OPTION_TYPE: &str = "core::option::Option";
const RESULT_TYPE: &str = "core::result::Result";

enum Types {
    Tryable,
    Others,
}

fn type_of<T>(_: &T) -> Types {
    let type_str = std::any::type_name::<T>();
    if type_str.len() >= 20 {
        match &type_str[..20] {
            OPTION_TYPE => return Types::Tryable,
            RESULT_TYPE => return Types::Tryable,
            _ => return Types::Others,
        }
    }
    Types::Others
}

#[macro_export]
macro_rules! pipa {
    ($init:expr $(=> $fn:expr)*) => {{
        let r = $init;
        $(
            let r = $fn(r);
        )*
        r
    }};
}

#[macro_export]
macro_rules! pipa_try {
    ($init:expr $(=> $fn:expr)*) => {{
        let r = $init;
        $(
            let r = $fn(r)?;
        )*
        r
    }};
}

#[cfg(test)]
mod pipa_tests {

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
        let ret = pipa!(123);
        assert_eq!(123, ret);
        let ret = pipa!(123 => f => g => h);
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

    #[test]
    fn pipe2() {
        let result = sdf().unwrap_or_default();
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

    #[test]
    fn pipe3() {
        let result = try_sdf().unwrap_or_default();
        assert_eq!(12, result);
    }
}
