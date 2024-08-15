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
