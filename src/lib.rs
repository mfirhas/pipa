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

#[macro_export]
macro_rules! pipa_await_try {
    ($init:expr $(=> $fn:expr)*) => {{
        let r = $init;
        $(
            let r = $fn(r).await?;
        )*
        r
    }};
}
