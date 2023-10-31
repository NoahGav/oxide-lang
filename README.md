# 1. Implicit Lifetime Handling in Oxide

Oxide's design philosophy centers around simplicity and efficiency without
sacrificing safety. One key aspect of this approach is its implicit lifetime
handling. Oxide borrows Rust's borrow checking mechanism but dispenses with the
need for explicit lifetime annotations, making it more accessible to developers.

### 1.1 Eliminating Explicit Lifetime Annotations

In Oxide, the handling of lifetimes is simplified by eliding all lifetimes, even
within structs. This approach significantly simplifies lifetime management,
eliminating the need for explicit annotations. While Rust theoretically supports
elided lifetimes, Oxide embraces this feature more explicitly from the ground
up. This means that lifetimes are automatically inferred and validated by the
compiler, reducing the cognitive burden on developers.

### 1.2 Ban on Returning References from Functions

To maintain clarity and simplicity while avoiding the complexity of explicit
lifetimes, Oxide introduces a restriction on returning references from
functions. While the Rust language encounters challenges when multiple `&`
references of the same type are passed to a function, Oxide simplifies this
scenario by disallowing functions to return references. For instance, in Rust, a
function like:

```rust
// This code gives a compile-time error as rust cannot
// know if the lifetime returned is from a, b, or c.
fn foo<T>(a: &T, b: &T, c: &T) -> &T { ... }

// However, the following code has no errors as rust
// ensures the lifetime returned is the same as the
// &self lifetime. This is why oxide prohibits returning
// references from functions, but not methods.
struct Foo;

impl Foo {
    fn foo<T>(&self, a: &T, b: &T, c: &T) -> &T {
        todo!()
    }
}
```

Presents challenges for the compiler to determine which reference's lifetime
should be returned. Oxide circumvents this ambiguity by enforcing a ban on
returning references from functions. However, references can still be returned
in methods where the lifetime of the returned reference is guaranteed to be at
least the same as the `&self` reference. This approach optimizes Oxide for
application development and simplifies the codebase without sacrificing safety.

### 1.3 Optimized for Application Development

Oxide's approach to implicit lifetime handling is tailored for application
development, offering the benefits of Rust's robust safety features while
streamlining the coding process. This design choice optimizes Oxide for
application development tasks, ensuring that developers can build reliable and
efficient software with ease. This simplified lifetime management not only
enhances code readability but also reduces the need for extensive error
checking, making development in Oxide more intuitive and productive.

# 2. Implicit Trait Implementation with Attribute Support

### 2.1 Implicit Trait Implementation

In Oxide, trait implementation is primarily implicit. When a type defines all
the methods required by a specific trait, it automatically implements that
trait. This approach draws inspiration from Go's interfaces, offering a
simplified way to define and use traits without the need for explicit
declarations.

### 2.2 Attribute Support for Trait Implementation

Oxide introduces the @impl(Trait); attribute to address scenarios where multiple
traits may have methods with interfering signatures. This attribute allows
developers to specify the trait to which a method implementation belongs,
resolving potential conflicts and ensuring precise trait behavior for each trait
a type implements.

# 3. Unified Approach to Data Allocation

### 3.1 Stack and Heap Unification

Oxide eliminates the distinction between stack and heap allocation, offering a
more unified and straightforward approach to data allocation. The compiler
automatically determines the appropriate allocation strategy based on the
runtime size of data, reducing developer overhead and complexity. This unified
approach simplifies working with data in Oxide and streamlines the language's
memory management.

# 4. Powerful Macros in Oxide

Oxide features a sophisticated and powerful macro system that provides
comprehensive compile-time reflection and code generation capabilities. Unlike
traditional macros in many languages, Oxide's macros are defined as regular
functions that take in compile-time reflection objects, empowering developers
with an enhanced level of control and expressiveness.

### 4.1 Macros as Regular Functions

In Oxide, macros are designed to resemble regular functions, simplifying their
usage and making them more approachable. Rather than relying on token streams,
macros take advantage of compile-time reflection objects, such as the Type
object. This compilation-time data holds complete semantic information about the
entire program, enabling tailored implementations that are both powerful and
precise.

### 4.2 @derive(Trait, ...) Macro

One of the standout examples of Oxide's macro capabilities is the @derive(Trait,
...) macro. This macro is responsible for generating implementations of a
specified trait for a given type. By leveraging the Type object, the @derive
macro creates Method objects, representing the implementations of the trait for
the type. This approach provides a level of detail and customization that is not
possible with simplistic token streams.

### 4.3 Incremental Computation

Oxide's macro system incorporates a concept known as incremental computation.
When the compiler processes a program, it first builds a semantic model without
macros. Subsequently, macros are executed to generate code, and the semantic
model is reconstructed. Thanks to this incremental computation, the reevaluation
of the semantic model becomes highly efficient, especially when source files
remain unchanged.

Oxide's approach to macros offers greater flexibility and precision, enabling
developers to create custom code generators and extensions with ease. The use of
compile-time reflection objects enhances the robustness of macros and simplifies
the code generation process, making it a standout feature of the language.

# 5. Error Handling in Oxide

## Introduction

Error handling is a fundamental aspect of software development, and Oxide
strives to provide a simple, yet robust and powerful approach to error
management. This formal specification outlines the various techniques and
features Oxide offers to handle errors effectively while ensuring code safety
and reliability.

### T? and the ? Operator

#### 5.1 Result Type (T?)

In Oxide, error handling begins with the use of the T? type, which signifies the
potential for errors. Functions that may produce errors return a T?, where T is
the result type. This standardized approach ensures clear and explicit error
signaling.

#### 5.2 The ? Operator

The ? operator is a central element of Oxide's error handling mechanism. It
allows functions to handle errors without panicking. When applied to an
expression, the ? operator checks for errors and either returns the error or
unwraps the value, depending on the outcome. This enables systematic error
propagation.

#### 5.3 @bail Macro

The @bail macro simplifies the process of returning an error from within a
function. It provides a convenient shortcut for returning an error at any point
in the code, enhancing the manageability of exceptional cases.

#### 5.4 Custom Error Types

Oxide allows developers to define custom error types by implementing the Error
trait. This flexibility empowers developers to create tailored error types for
specific use cases, improving error handling precision. This can be done using
the @derive(Error); macro.

#### 5.5 Implicit Error Handling

Oxide encourages explicit error handling by design. Functions returning T? and
the ? operator make it clear that error handling is a natural part of the code.
This approach minimizes the potential for unexpected runtime panics and promotes
safer and more controlled error management.

#### 5.6 try { ... } Blocks

The try { ... } blocks provide a convenient syntax to implicitly apply the ?
operator to all expressions that possibly contain an error within a block. This
simplifies error handling when multiple operations may return T?.

```rust
let result = try {
    let value = potentially_failing_operation(); // The ? is not required here.
    value + 42
};
```

#### 5.7 Pattern Matching on the Error Trait

Oxide allows developers to create custom error types and implement the Error
trait for them. These custom error types can be pattern matched for specific
error handling. This example illustrates the creation of a custom error type and
pattern matching:

```rust
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

### 6. The `??` Operator

The `??` operator in Oxide introduces a powerful and concise way to handle
errors by replacing them with alternative values. This operator is especially
useful when you want to provide a default or fallback value in case of errors,
simplifying error handling and making your code more readable.

#### 6.1 Usage

The `??` operator is used as follows:

```rust
let value = some_operation() ?? fallback_value;
```

In this expression:

- If `some_operation()` succeeds and returns a valid result, `value` will be
  assigned the value returned by `some_operation()`.
- If `some_operation()` encounters an error, the `??` operator replaces the
  error with the `fallback_value`.

#### 6.2 Benefits

The `??` operator offers several benefits:

- **Simplified Error Handling:** It streamlines error handling by allowing you
  to specify fallback values for specific operations.

- **Clear Code:** Your code becomes more concise and easier to read, as you can
  express error handling and fallback behavior in a single line.

- **Reduced Error-Checking Code:** The `??` operator reduces the need for
  extensive error-checking code when you don't care about the error and can
  provide a sensible fallback.

#### 6.3 Examples

Here are some examples of how to use the `??` operator in Oxide:

```rust
let value = potentially_failing_operation() ?? 0;
```

In this example, if `potentially_failing_operation()` encounters an error, the
`??` operator replaces the error with the value `0`.

```rust
let value = fetch_data() ?? load_default_data();
```

In this example, if `fetch_data()` fails to retrieve data, the `??` operator
loads default data using `load_default_data()`.

The `??` operator is a valuable addition to Oxide's error handling toolbox,
enabling developers to handle errors with ease and precision while providing
fallback values when needed.

### Conclusion

Error handling in Oxide prioritizes safety, clarity, and reliability. By
providing systematic and standardized error handling techniques, the language
empowers developers to create more robust and maintainable code while avoiding
unexpected runtime errors. Oxide's approach to error management is simple, yet
powerful, making it a valuable tool for application development.

# 7. The `=>` Syntax in Oxide

## Introduction

Oxide introduces the `=>` syntax as a convenient and expressive feature to
streamline code blocks that can be represented as single expressions. This
formal specification provides an overview of the `=>` syntax and its
applications, enhancing code precision and readability.

### 7.1 Single Expression Statements

The primary use of the `=>` syntax is to simplify single-expression statements.
In Oxide, it allows developers to express these statements without the need for
`{}` block delimiters.

#### 7.2 Function Definitions

When defining functions with single expressions as their bodies, the `=>` syntax
becomes a powerful tool for concise code. For example:

```rust
fn add(a: i32, b: i32) -> i32 => a + b;
```

In this case, the entire function body consists of a single expression
(`a + b`). The `=>` syntax eliminates the need for explicit `{}` blocks,
providing a more precise and clean representation.

#### 7.3 Conditional Statements

The `=>` syntax can be employed to simplify conditional statements, such as `if`
and `else` expressions.

```rust
if condition => do_something();
else => do_something_else();
```

Here, the `if` and `else` branches are single expressions (`do_something()` and
`do_something_else()`). The `=>` syntax streamlines the code, making it more
readable and less verbose.

#### 7.4 Error Handling Precision

The `=>` syntax also plays a role in error handling. For functions that don't
care fo explicitly handling errors, wrapping the entire function body in a `try`
block is a common practice.

```rust
fn do_something() -> T? => try {
    ... // We don't have to worry about any errors in this block.
};
```

In this example, the `try` block serves as the statement body, ensuring that
error handling is handled implicitly. The `=>` syntax allows developers to
create precise error-handling functions without the need for additional `{}`
blocks.

### Conclusion

The `=>` syntax in Oxide offers a valuable tool for enhancing code precision and
readability. By simplifying single-expression statements, it allows developers
to express code more concisely, reducing verbosity and providing a clean and
precise representation of functions, conditional statements, and error handling.
Oxide's `=>` syntax contributes to a more efficient and expressive coding
experience, ultimately making the language more developer-friendly.

# 8. Examples

### 8.1 Error Handling

```rust
// main.ox, explicit error handling with ? operator.

use std; // This imports the std namespace.

@derive(Debug); // This automatically implements the Debug trait for type Foo.
type Foo (
    bar: i32,
);

// This implements the add method for type Foo.
fn Foo.add(self, rhs: i32) -> Self
    => Self(bar: self.bar + rhs);

// The ? return type means that the main function may return an error (but not a value).
fn main() -> ? {
    let foo = Foo(bar: 41) + 1; // This works because Foo properly implements the Add trait.
    
    // io is from the std namespace. since we imported it, we do
    // not have to use the fully-qualified name (std::io::println).
    io::println(`{foo:?}`)?; // The ? operator propagates any errors returned by println (if there was one).
}
```

```rust
// main.ox, implicit error handling with try block.

...

// Since we might not want to handle the errors in the main function, we can use an arrow body
// instead and wrap the whole function in a try block.
fn main() -> ? => try {
    let foo = Foo(bar: 41) + 1;

    // As you can see, we do not need the ? operator
    // here as it is inside a try block.
    io::println(`{foo:?}`);
}; // We need a semicolon because a try block is a statement.
```

```rust
// main.ox, handling a specific error (using trait pattern matching)

fn main() -> ? {
    let file = std::fs::open("./example.txt");

    if let Err(FileDoesntExist(path)) = file {
        std::io::println(`The file {path} does not exist.`)?;
    } else {
        // Do something with the file.
    }
}
```
