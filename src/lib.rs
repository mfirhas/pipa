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

/// p!(...) mixes expressions of plain value, try-able value, await-able value, and await-try-able value
#[macro_export]
macro_rules! p {
    // single expression: p!(3), p!(function())
    ($e:expr) => {
        $e
    };

    // piping multiple expression with method calls: p!(o.method())
    ($e:expr => $func:ident()) => {
        $e.$func()
    };

    // piping simple function or method: p!(func())
    ($e:expr => $func:ident) => {
        $func($e)
    };

    // piping with function returning Try-able values(Result & Option) using `?`: p!(4 => process?)
    ($e:expr => $func:ident?) => {
        $func($e)?
    };

    // piping async function with `.await`: p!(5 => send.await)
    ($e:expr => $func:ident.await) => {
        $func($e).await
    };

    // piping async returning try-able value: p!(func.await?)
    ($e:expr => $func:ident.await?) => {
        $func($e).await?
    };

    // piping method calls with following functions: p!(o.method() => func)
    ($e:expr => $func:ident() => $($rest:tt)*) => {
        p!($e.$func() => $($rest)*)
    };

    // piping function calls with following functions: p!(func(params) => func)
    ($e:expr => $func:ident => $($rest:tt)*) => {
        p!($func($e) => $($rest)*)
    };

    // piping functions returning Try-able value: p!(val => func?)
    ($e:expr => $func:ident? => $($rest:tt)*) => {
        p!($func($e)? => $($rest)*)
    };

    // piping async functions: p!(val => func.await)
    ($e:expr => $func:ident.await => $($rest:tt)*) => {
        p!($func($e).await => $($rest)*)
    };

    // piping async returning try-able value with other expressions: p!(val => func.await? => expr)
    ($e:expr => $func:ident.await? => $($rest:tt)*) => {
        p!($func($e).await? => $($rest)*)
    };
}
