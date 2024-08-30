#![allow(rustdoc::invalid_rust_codeblocks)]

//! `pipa` is a declarative macro for pipe operator in Rust to support chaining multiple expressions.
//!
//! - `pipa!`: Supports only plain expressions(non-try-able and not async). When you want to make sure only plain values chained.
//! - `pipa_try!`: Supports only expressions returning try-able values like `Option` and `Result`. When you want to implement ROP pattern.
//! - `pipa_await_try!`: Supports only async expressions return try-able values. When you want to ROP on async functions only.
//! - `p!`: Supports mixes expressions:
//!     - functions
//!     - associated functions
//!     - methods
//!     - async functions
//!     - async associated functions
//!     - async methods
//!     - functions returning try-able value
//!     - associated functions returning try-able value
//!     - methods returning try-able value
//!     - async functions returning try-able value
//!     - async methods returning try-able value
//!     - async associated functions returning try-able value

/// pipa! only supports plain expressions(non-try-able and not async).
///
/// Example:
/// ```rust,ignore
/// use pipa::pipa;
///
/// pipa!(5 => double_it => add_one); // return 11
/// ```
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
///
/// Example:
/// ```rust,ignore
/// use pipa::pipa_try;
///
/// pipa_try!(5 => double_it => add_one); // return 11, if none `None` nor `Err` found in the chains
/// ```
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

/// pipa_await_try only! supports async expressions returning try-able value.
///
/// Example:
/// ```rust,ignore
/// use pipa::pipa_try;
///
/// pipa_await_try!(5 => double_it => add_one); // return 11 concurrently, if none `None` nor `Err` found in the chains
/// ```
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

/// p! supports all mixes of expressions
///
/// Example:
/// ```rust,ignore
/// use pipa::p;
///
/// async fn run_them_all(obj: Obj) -> Result<i32, &'static str> {
///     let clo = |v: i32| -> i32 { v * 5 };
///
///     let result = p!(5
///         => func
///         => clo
///         => func_rop?
///         => func_2.await
///         => func_2_rop.await?
///         => obj.method
///         => func
///         => obj.method_rop?
///         => func_rop?
///         => obj.method_async.await
///         => obj.method_async_rop.await?
///     );
///
///     let ret = p!(5
///         => D::anu
///         => D::anu_try?
///         => D::async_anu.await
///         => D::async_anu_try.await?
///     );
///     assert_eq!(ret, 9000);
///
///     let ret = p!(5
///         => E::anu
///         => E::anu_try?
///         => E::async_anu.await
///         => E::async_anu_try.await?
///     );
///     assert_eq!(ret, 3000);
///
///     Ok(result)
/// }
/// ```
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

    //-- inline closures
    // Piping with an inline closure: p!(val => |arg| expr)
    ($e:expr => |$arg:ident| $body:expr) => {
        (|$arg| $body)($e)
    };

    // Piping with an inline closure with parameter type: p!(val => |arg: ty| expr)
    ($e:expr => |$arg:ident : $type:ty| $body:expr) => {
        (|$arg: $type| $body)($e)
    };

    // Piping with an inline closure with return type: p!(val => |arg| -> ty { expr })
    ($e:expr => |$arg:ident| -> $ret:ty { $($body:tt)* }) => {
        (|$arg| -> $ret { $($body)* })($e)
    };

    // Piping with an inline closure with parameter type and return type: p!(val => |arg: ty| -> ret { expr })
    ($e:expr => |$arg:ident : $type:ty| -> $ret:ty { $($body:tt)* }) => {
        (|$arg: $type| -> $ret { $($body)* })($e)
    };

    // Piping with an inline closure using braces: p!(val => |arg| { /* closure body */ })
    ($e:expr => |$arg:ident| { $($body:tt)* }) => {
        (|$arg| { $($body)* })($e)
    };

    // Piping with an inline closure with parameter type using braces: p!(val => |arg: ty| { /* closure body */ })
    ($e:expr => |$arg:ident : $type:ty| { $($body:tt)* }) => {
        (|$arg: $type| { $($body)* })($e)
    };

    // Piping multiple inline closures: p!(val => |arg| expr => next_op)
    ($e:expr => |$arg:ident| $body:expr => $($rest:tt)*) => {
        p!((|$arg| $body)($e) => $($rest)*)
    };

    // Piping multiple inline closures operations with a closure with parameter type: p!(val => |arg: ty| expr => next_op)
    ($e:expr => |$arg:ident : $type:ty| $body:expr => $($rest:tt)*) => {
        p!((|$arg: $type| $body)($e) => $($rest)*)
    };

    // piping multiple inline closures with unused args: p!(val => |arg| -> ret { expr })
    ($e:expr => |$arg:ident| -> $ret:ty { $($body:tt)* } => $($rest:tt)*) => {
        p!((|$arg| -> $ret { $($body)* })($e) => $($rest)*)
    };


    // Piping multiple inline closures operations with a closure with parameter type and return type: p!(val => |arg: ty| -> ret { expr } => next_op)
    ($e:expr => |$arg:ident : $type:ty| -> $ret:ty { $($body:tt)* } => $($rest:tt)*) => {
        p!((|$arg: $type| -> $ret { $($body)* })($e) => $($rest)*)
    };

    // Piping multiple inline closures operations with a closure using braces: p!(val => |$arg| { $($body:tt)* } => next_op)
    ($e:expr => |$arg:ident| { $($body:tt)* } => $($rest:tt)*) => {
        p!((|$arg| { $($body)* })($e) => $($rest)*)
    };

    // Piping multiple inline closures operations with a closure with parameter type using braces: p!(val => |$arg:ident : $type:ty| { $($body:tt)* } => next_op)
    ($e:expr => |$arg:ident : $type:ty| { $($body:tt)* } => $($rest:tt)*) => {
        p!((|$arg: $type| { $($body)* })($e) => $($rest)*)
    };
    //-- inline closures (END)
}
