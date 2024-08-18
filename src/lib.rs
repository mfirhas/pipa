/// pipa! only supports plain expressions(non-try-able and not async).
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

/// pipa_try! only supports expressions returning try-able value.
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

/// pipa_await_try only supports async expressions returning try-able value.
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

/// p! supports all mixes of expressions below:
/// - functions
/// - methods
/// - functions returning try-able value
/// - methods returning try-able value
/// - async functions
/// - async functions returning try-able value
/// - async methods
/// - async methods returning try-able value
#[macro_export]
macro_rules! p {
    // Single expression: p!(3), p!(function())
    ($e:expr) => {
        $e
    };

    //-- functions
    // Piping a simple function call: p!(val => func)
    ($e:expr => $func:ident) => {
        $func($e)
    };

    // Piping a function returning Try-able values (Result & Option) using `?`: p!(val => func?)
    ($e:expr => $func:ident?) => {
        $func($e)?
    };

    // Piping an async function using `.await`: p!(val => func.await)
    ($e:expr => $func:ident.await) => {
        $func($e).await
    };

    // Piping an async function returning Try-able values using `.await?`: p!(val => func.await?)
    ($e:expr => $func:ident.await?) => {
        $func($e).await?
    };
    //-- functions (END)

    //-- methods
    // Piping a method call: p!(val => obj.method)
    ($e:expr => $obj:ident.$method:ident) => {
        $obj.$method($e)
    };

    // Piping a method returning Try-able values using `?`: p!(val => obj.method?)
    ($e:expr => $obj:ident.$method:ident?) => {
        $obj.$method($e)?
    };

    // Piping a method call with `await`: p!(val => obj.method.await)
    ($e:expr => $obj:ident.$method:ident.await) => {
        $obj.$method($e).await
    };

    // Piping a method call with `await?`: p!(val => obj.method.await?)
    ($e:expr => $obj:ident.$method:ident.await?) => {
        $obj.$method($e).await?
    };
    //-- methods (END)

    //-- associated functions
    // Piping an associated function: p!(val => type::func)
    ($e:expr => $type:ident::$func:ident) => {
        $type::$func($e)
    };

    // Piping an associated function returning Try-able values using `?`: p!(val => type::func?)
    ($e:expr => $type:ident::$func:ident?) => {
        $type::$func($e)?
    };

    // Piping an associated function with `await`: p!(val => type::func.await)
    ($e:expr => $type:ident::$func:ident.await) => {
        $type::$func($e).await
    };

    // Piping an associated function with `await?`: p!(val => type::func.await?)
    ($e:expr => $type:ident::$func:ident.await?) => {
        $type::$func($e).await?
    };
    //-- associated functions (END)

    //-- functions
    // Piping multiple operations with a simple function call: p!(v => func1 => func2)
    ($e:expr => $func:ident => $($rest:tt)*) => {
        p!($func($e) => $($rest)*)
    };

    // Piping multiple operations with a function returning Try-able values using `?`: p!(v => func1? => expr)
    ($e:expr => $func:ident? => $($rest:tt)*) => {
        p!($func($e)? => $($rest)*)
    };

    // Piping multiple operations with an async function using `.await`: p!(v => func1.await => expr)
    ($e:expr => $func:ident.await => $($rest:tt)*) => {
        p!($func($e).await => $($rest)*)
    };

    // Piping multiple operations with an async function returning Try-able values using `.await?`: p!(v => func1.await? => func2.await?)
    ($e:expr => $func:ident.await? => $($rest:tt)*) => {
        p!($func($e).await? => $($rest)*)
    };
    //-- functions (END)

    //-- methods
    // Piping multiple operations with a method call: p!(v => obj.method => obj.method2 => obj2.method)
    ($e:expr => $obj:ident.$method:ident => $($rest:tt)*) => {
        p!($obj.$method($e) => $($rest)*)
    };

    // Piping multiple operations with a method call returning Try-able values using `?`: p!(v => obj.method? => obj.method2? => obj2.method?)
    ($e:expr => $obj:ident.$method:ident? => $($rest:tt)*) => {
        p!($obj.$method($e)? => $($rest)*)
    };

    // Piping multiple operations with a method call returning Try-able values using `?`: p!(v => obj.method.await => obj.method2? => obj2.method.await)
    ($e:expr => $obj:ident.$method:ident.await => $($rest:tt)*) => {
        p!($obj.$method($e).await => $($rest)*)
    };

    // Piping multiple operations with a method call returning Try-able values using `?`: p!(v => obj.method.await? => obj.method2? => obj2.method.await?)
    ($e:expr => $obj:ident.$method:ident.await? => $($rest:tt)*) => {
        p!($obj.$method($e).await? => $($rest)*)
    };
    //-- methods (END)

    //-- associated functions
    // Piping multiple associated functions: p!(val => type::func => type::func2)
    ($e:expr => $type:ident::$func:ident => $($rest:tt)*) => {
        p!($type::$func($e) => $($rest)*)
    };

    // Piping multiple associated function returning Try-able values using `?`: p!(val => type::func? => type::func2?)
    ($e:expr => $type:ident::$func:ident? => $($rest:tt)*) => {
        p!($type::$func($e)? => $($rest)*)
    };

    // Piping multiple associated function with `await`: p!(val => type::func.await => type::func2.await)
    ($e:expr => $type:ident::$func:ident.await => $($rest:tt)*) => {
        p!($type::$func($e).await => $($rest)*)
    };

    // Piping multiple associated function with `await?`: p!(val => type::func.await? => type::func2.await?)
    ($e:expr => $type:ident::$func:ident.await? => $($rest:tt)*) => {
        p!($type::$func($e).await? => $($rest)*)
    };
    //-- associated functions (END)
}
