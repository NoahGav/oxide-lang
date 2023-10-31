# Error Handling in Oxide

## Introduction

Error handling is a fundamental aspect of software development, and Oxide
strives to provide a simple, yet robust and powerful approach to error
management. This formal specification outlines the various techniques and
features Oxide offers to handle errors effectively while ensuring code safety
and reliability.

### 1. T? and the ? Operator

#### 1.1. Result Type (T?)

In Oxide, error handling begins with the use of the T? type, which signifies the
potential for errors. Functions that may produce errors return a T?, where T is
the result type. This standardized approach ensures clear and explicit error
signaling.

#### 1.2. The ? Operator

The ? operator is a central element of Oxide's error handling mechanism. It
allows functions to handle errors without panicking. When applied to an
expression, the ? operator checks for errors and either returns the error or
unwraps the value, depending on the outcome. This enables systematic error
propagation.

### 2. @bail Macro

The @bail macro simplifies the process of returning an error from within a
function. It provides a convenient shortcut for returning an error at any point
in the code, enhancing the manageability of exceptional cases.

### 3. Custom Error Types

Oxide allows developers to define custom error types by implementing the Error
trait. This flexibility empowers developers to create tailored error types for
specific use cases, improving error handling precision. This can be done using
the @derive(Error); macro.

### 4. Implicit Error Handling

Oxide encourages explicit error handling by design. Functions returning T? and
the ? operator make it clear that error handling is a natural part of the code.
This approach minimizes the potential for unexpected runtime panics and promotes
safer and more controlled error management.

### 5. try { ... } Blocks

The try { ... } blocks provide a convenient syntax to implicitly apply the ?
operator to all expressions that possibly contain an error within a block. This
simplifies error handling when multiple operations may return T?.

```oxide
let result = try {
    let value = potentially_failing_operation(); // The ? is not required here.
    value + 42
};
```

### 6. Pattern Matching on the Error Trait

Oxide allows developers to create custom error types and implement the Error
trait for them. These custom error types can be pattern matched for specific
error handling. This example illustrates the creation of a custom error type and
pattern matching:

```oxide
@derive(Error);
type MyError(
    description: string;
);

fn handle_error(error: Error) {
    if let MyError(description) = error {
        // Handle specific error
    }
}
```

### Conclusion

Error handling in Oxide prioritizes safety, clarity, and reliability. By
providing systematic and standardized error handling techniques, the language
empowers developers to create more robust and maintainable code while avoiding
unexpected runtime errors. Oxide's approach to error management is simple, yet
powerful, making it a valuable tool for application development.

# The `=>` Syntax in Oxide

## Introduction

Oxide introduces the `=>` syntax as a convenient and expressive feature to
streamline code blocks that can be represented as single expressions. This
formal specification provides an overview of the `=>` syntax and its
applications, enhancing code precision and readability.

### 1. Single Expression Statements

The primary use of the `=>` syntax is to simplify single-expression statements.
In Oxide, it allows developers to express these statements without the need for
`{}` block delimiters.

#### 1.1. Function Definitions

When defining functions with single expressions as their bodies, the `=>` syntax
becomes a powerful tool for concise code. For example:

```oxide
fn add(a: i32, b: i32) -> i32 => a + b;
```

In this case, the entire function body consists of a single expression
(`a + b`). The `=>` syntax eliminates the need for explicit `{}` blocks,
providing a more precise and clean representation.

#### 1.2. Conditional Statements

The `=>` syntax can be employed to simplify conditional statements, such as `if`
and `else` expressions.

```oxide
if condition => do_something();
else => do_something_else();
```

Here, the `if` and `else` branches are single expressions (`do_something()` and
`do_something_else()`). The `=>` syntax streamlines the code, making it more
readable and less verbose.

### 2. Error Handling Precision

The `=>` syntax also plays a role in error handling. For functions that don't
care fo explicitly handling errors, wrapping the entire function body in a `try`
block is a common practice.

```oxide
fn do_something() -> T? => try {
    ... // We don't have to worry about any errors in this block.
};
```

In this example, the `try` block serves as the statement body, ensuring that
error handling is handled implicitly. The `=>` syntax allows developers to
create precise error-handling functions without the need for additional `{}`
blocks.

## Conclusion

The `=>` syntax in Oxide offers a valuable tool for enhancing code precision and
readability. By simplifying single-expression statements, it allows developers
to express code more concisely, reducing verbosity and providing a clean and
precise representation of functions, conditional statements, and error handling.
Oxide's `=>` syntax contributes to a more efficient and expressive coding
experience, ultimately making the language more developer-friendly.
