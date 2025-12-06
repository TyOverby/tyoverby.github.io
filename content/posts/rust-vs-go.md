---
title: Rust and Go (Error Handling)
subtitle: Sums and Products
date: 2015-10-30
tags: ["rust", "go", "error-handling", "language-design"]
---

The Rust and Go programming languages are relatively new programming languages with an emphasis on
writing and maintaining very large codebases.  Go was designed to be used internally at Google in their internal services
and one of the main driving forces behind rust is Servo: a web-browser written from the ground up focusing on layout parallelism and safety.

What I think is really interesting about both is that they decided against using Exceptions as the main method of
error handling; (D and Nim are other newish compiled languages that stick with Exceptions).
Both Rust and Go treat their error values like any other object or value in the program, however the way that
choose to implement it differs greatly.

## Errors in Go

In Go, functions can return multiple values.   It looks like this:

```go
func firstFibs() (uint32, uint32) {
    return 0, 1
}
```

When a function can fail, it is good practice to return a value AND an error.

```go
func canFail() (uint32, error) {
...
}
```

Here's the catch; in the case that the function completed without failing, the second return value (`error`)
is `nil`.  If there was a failure, `error` will be non-`nil` and will contain a description of the error that
occurred.

When calling a function that can fail, handling the error is necessary.

Here is an example of a function that calls our previously defined failure function.
It simply propogates errors by returning them if any contained function call failed.

```go
func doesThings() (uint32, error)
    res1, err1 = canFail()

    if err1 != nil {
        return error
    }

    res2, err2 = canFail()

    if err2 != nil {
        return error
    }

    return res1 + res2, nil
}
```

Instead of propogating these errors by returning them, we could have handled the error by inspecting it
just like any other Go value.

## Errors in Rust

In Rust, errors are returned by-value too, but because Rust has sum-types, the result value and the error
are combined into one type called Result.  Here's what the `Result` enum looks like:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

Then, a function that can return either a value or an error would have a type signature like this one:

```rust
// IoError is an error type commonly used in IO operations where everything can fail.
fn can_fail() -> Result<u32, IoError>;
```

When calling these functions, we still run into the same issue that we had in the Go code.
Namely that we need to handle those errors and use the values inside.  Here's how it looks.

```rust
fn does_things() -> Result<u32, IoError> {
    let res1 = match can_fail() {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    let res2 = match can_fail() {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    return Ok(res1 + res2);
}
```

Although we went from having multiple return values in Go (only one of which is valid),
to having a single return value (with only one valid variant), the code looks roughly the same.

However, a more ideomatic way of writing this rust code would be to use the `try!` macro.
This macro inserts the match and early return for us and can really clean up the code.

```rust
fn does_things() -> Result<u32, IoError> {
    let res1 = try!(canFail())
    let res2 = try!(canFail())
    return Ok(res1 + res2);
}
```

When a programmer needs to handle errors in a way that isn't simply returning a result, he would
have to inspect the result like in the first example, but if transforming one error value to another
is enough, the `try!` macro can do that too.

For more information on error handling in rust, read the chapter from the
[Rust Book](https://doc.rust-lang.org/book/error-handling.html).
