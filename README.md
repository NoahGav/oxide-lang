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

# 5. Algebraic Types (Tagged Unions)

In Oxide, algebraic types, commonly referred to as tagged unions, provide a
powerful and flexible mechanism for defining complex data structures that can
have multiple shapes or variants. These types, although conceptually similar to
Rust's enums, are designed with the aim of enhancing code expressiveness and
simplifying data modeling.

### 5.1 Defining Algebraic Types

Defining algebraic types in Oxide is straightforward, allowing you to declare a
type with multiple variants using a syntax that is reminiscent of Rust. The
syntax provides an intuitive way to specify these variants and their associated
data. Algebraic types can be declared as follows:

```rust
type EnumName =
    Variant1 |
    Variant2 |
    Variant3(Foo) |
    Variant4(bar: Bar);
```

- `EnumName` is the name of the algebraic type.
- `Variant1`, `Variant2`, `Variant3`, and `Variant4` are the possible variants
  of the type.
- Variants can include associated data, like `Variant3(Foo)` and
  `Variant4(bar: Bar)`. This allows you to attach additional information to a
  variant when necessary.

### 5.2 Pattern Matching

Pattern matching on algebraic types in Oxide is nearly identical to Rust,
offering a familiar and powerful way to handle different variants. Pattern
matching allows developers to write code that responds to the shape of data,
making it an essential tool for data processing and control flow.

Here is an example of pattern matching in Oxide:

```rust
fn process_enum(enum_val: EnumName) => match enum_val {
    Variant1 => {
        // Handle Variant1
    }
    Variant2 => {
        // Handle Variant2
    }
    Variant3(foo) => {
        // Handle Variant3 with associated data `foo`
    }
    Variant4 { bar } => {
        // Handle Variant4 with associated data `bar`
    }
};
```

Pattern matching enables developers to inspect and process data based on its
variant, making it a powerful tool for working with algebraic types.

### 5.3 Use Cases for Algebraic Types

Algebraic types are particularly valuable when modeling data with different
shapes or behaviors. They can represent a variety of scenarios, such as:

- Representing different states of an application or system.

- Modeling data with variable structures, such as nodes in a tree or items in a
  document.

- Defining error types with different error codes and associated data.

- Handling complex parsing or transformation tasks by distinguishing between
  various forms or structures.

By allowing for clear and concise modeling of these scenarios, algebraic types
contribute to improved code readability and maintainability in Oxide. They are a
fundamental feature for creating data structures that can adapt to different
situations in a clean and expressive manner.

### 5.4 Benefits of Algebraic Types

The use of algebraic types in Oxide offers several benefits:

- **Code Clarity**: Algebraic types make the code more self-explanatory by
  clearly defining the possible shapes of data.

- **Type Safety**: Pattern matching ensures that you handle all possible
  variants, reducing the risk of unexpected behaviors.

- **Flexible Data Modeling**: They allow you to create data structures that can
  evolve with changing requirements without extensive code modifications.

- **Error Handling**: Algebraic types provide an elegant way to model and handle
  errors, making error management more structured and predictable.

- **Readable and Maintainable Code**: By modeling data with algebraic types, you
  create code that is easier to understand, modify, and maintain.

Algebraic types, as tagged unions, contribute to Oxide's aim of providing a
reliable and efficient language while maintaining simplicity and expressiveness
in code development.

# 6. Error Handling in Oxide

## Introduction

Error handling is a fundamental aspect of software development, and Oxide
strives to provide a simple, yet robust and powerful approach to error
management. This formal specification outlines the various techniques and
features Oxide offers to handle errors effectively while ensuring code safety
and reliability.

### 6.1 Result Type (T?)

In Oxide, error handling begins with the use of the T? type, which signifies the
potential for errors. Functions that may produce errors return a T?, where T is
the result type. This standardized approach ensures clear and explicit error
signaling.

### 6.2 The ? Operator

The ? operator is a central element of Oxide's error handling mechanism. It
allows functions to handle errors without panicking. When applied to an
expression, the ? operator checks for errors and either returns the error or
unwraps the value, depending on the outcome. This enables systematic error
propagation.

### 6.3 @bail Macro

The @bail macro simplifies the process of returning an error from within a
function. It provides a convenient shortcut for returning an error at any point
in the code, enhancing the manageability of exceptional cases.

### 6.4 Custom Error Types

Oxide allows developers to define custom error types by implementing the Error
trait. This flexibility empowers developers to create tailored error types for
specific use cases, improving error handling precision. This can be done using
the @derive(Error); macro.

### 6.5 Implicit Error Handling

Oxide encourages explicit error handling by design. Functions returning T? and
the ? operator make it clear that error handling is a natural part of the code.
This approach minimizes the potential for unexpected runtime panics and promotes
safer and more controlled error management.

### 6.6 try { ... } Blocks

The try { ... } blocks provide a convenient syntax to implicitly apply the ?
operator to all expressions that possibly contain an error within a block. This
simplifies error handling when multiple operations may return T?.

```rust
let result = try {
    let value = potentially_failing_operation(); // The ? is not required here.
    value + 42
};
```

#### Note

try blocks do not propagate the errors upwards towards the return of the
function. Instead, they propagate the error to the result of the try block
expression. This means that a try block evaluates to T? and not T. If you do not
want to handle the error produced by a try block, you could still use the ?
operator.

```rust
// Because we use the ? operator, the error from the try block
// is propagated to the return of the enclosing function. That's
// why the variable is named value and not result.
let value = try {
    let value = potentially_failing_operation();
    value + 42
}?;
```

### 6.7 Pattern Matching on the Error Trait

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

# 7. The `??` Operator

The `??` operator in Oxide introduces a powerful and concise way to handle
errors by replacing them with alternative values. This operator is especially
useful when you want to provide a default or fallback value in case of errors,
simplifying error handling and making your code more readable.

### 7.1 Usage

The `??` operator is used as follows:

```rust
let value = some_operation() ?? fallback_value;
```

In this expression:

- If `some_operation()` succeeds and returns a valid result, `value` will be
  assigned the value returned by `some_operation()`.
- If `some_operation()` encounters an error, the `??` operator replaces the
  error with the `fallback_value`.

### 7.2 Benefits

The `??` operator offers several benefits:

- **Simplified Error Handling:** It streamlines error handling by allowing you
  to specify fallback values for specific operations.

- **Clear Code:** Your code becomes more concise and easier to read, as you can
  express error handling and fallback behavior in a single line.

- **Reduced Error-Checking Code:** The `??` operator reduces the need for
  extensive error-checking code when you don't care about the error and can
  provide a sensible fallback.

### 7.3 Examples

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

# 8. The `=>` Syntax in Oxide

## Introduction

Oxide introduces the `=>` syntax as a convenient and expressive feature to
streamline code blocks that can be represented as single expressions. This
formal specification provides an overview of the `=>` syntax and its
applications, enhancing code precision and readability.

### 8.1 Single Expression Statements

The primary use of the `=>` syntax is to simplify single-expression statements.
In Oxide, it allows developers to express these statements without the need for
`{}` block delimiters.

### 8.2 Function Definitions

When defining functions with single expressions as their bodies, the `=>` syntax
becomes a powerful tool for concise code. For example:

```rust
fn add(a: i32, b: i32) -> i32 => a + b;
```

In this case, the entire function body consists of a single expression
(`a + b`). The `=>` syntax eliminates the need for explicit `{}` blocks,
providing a more precise and clean representation.

### 8.3 Conditional Statements

The `=>` syntax can be employed to simplify conditional statements, such as `if`
and `else` expressions.

```rust
if condition => do_something();
else => do_something_else();
```

Here, the `if` and `else` branches are single expressions (`do_something()` and
`do_something_else()`). The `=>` syntax streamlines the code, making it more
readable and less verbose.

### 8.4 Error Handling Precision

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

# 9. Garbage-Collecting Shared References in Oxide

Oxide introduces a built-in type called `Gc<T>` that simplifies managing shared
references with built-in cyclic garbage collection. This section provides an
overview of `Gc<T>` and its advantages in handling shared data efficiently.

### 9.1 The `Gc<T>` Type

`Gc<T>` stands for "Garbage-Collected" and is a reference-counted smart pointer
that facilitates sharing data across multiple parts of your Oxide application.
It provides a concurrent reference counting mechanism that allows you to share
data efficiently while mitigating the risk of memory leaks caused by circular
references.

### 9.2 Automatic Interior Mutability

One of the primary features of `Gc<T>` is its ability to manage interior
mutability. When a type `T` is wrapped in `Gc<T>`, Oxide implicitly wraps the
fields of `T` in `RefCell` or `RwLock`, depending on whether `Gc<T>` is used in
a single-threaded or multi-threaded context. This automatic interior mutability
management ensures that concurrent access to shared data remains safe.

When accessing fields from `Gc<T>`, Oxide returns a `F?` instead of `F`, where
`F` represents the type of the field. This design choice reflects the shared
ownership of the data and the fact that Oxide cannot guarantee, at compile-time,
adherence to the "aliasing xor mutable" rule. Instead, runtime mechanisms handle
error checking, and any operation on a field may potentially return an error (of
type `F?`) if the borrow checking rules are violated.

### 9.3 Cyclic Garbage Collection in `Gc<T>`

`Gc<T>` employs a reference counting mechanism to keep track of shared
references and ensure proper memory management. When the reference count for a
particular piece of shared data reaches zero, it indicates that no active
references exist. However, the responsibility for releasing memory associated
with `Gc<T>` is shared between reference counting and a cyclic garbage
collection mechanism.

#### 9.3.1 Reference Counting

Reference counting in `Gc<T>` effectively tracks the number of active references
to shared data. It precisely increases the count when new references are created
and decreases it when references go out of scope or are no longer needed. When
the reference count drops to zero, it indicates that the shared data has no
active references. At this point, the object is freed immediately, and its
associated `drop` method is called, adhering to the same reference counting
principles as a regular reference-counted type.

This immediate memory release ensures that Oxide applications efficiently manage
memory when references are no longer needed, without introducing any delay in
the process.

#### 9.3.2 Cyclic Garbage Collection

While reference counting efficiently manages individual references, it cannot
detect circular references within a group of `Gc<T>` objects. To address this,
Oxide incorporates a cyclic garbage collection mechanism. This collector is
responsible for identifying and releasing memory associated with reference
cycles.

When cyclic references occur, the cyclic garbage collector identifies them and
intervenes to free the memory. By doing so, it ensures that `Gc<T>` remains a
memory-efficient solution for shared data management, even in the presence of
complex reference relationships.

The combination of reference counting and cyclic garbage collection in `Gc<T>`
provides a comprehensive and reliable memory management strategy, ensuring that
your Oxide application remains both efficient and free from memory leaks.

### 9.4 Using `Gc<T>` in Oxide

To use `Gc<T>`, you can wrap a type `T` using `Gc::new()`. Once wrapped, you can
seamlessly pass `Gc<T>` across different parts of your Oxide application. It
allows for sharing data without the complexity of lifetime management, offering
a straightforward solution for shared data scenarios.

### 9.5 Implementation of the Copy Trait

In Oxide, the `Gc` type implements the `Copy` trait. This means that `Gc`
instances are implicitly cloned when moved, and their reference counts are
increased accordingly. The `Copy` trait ensures that `Gc` behaves consistently
with other `Copy` types in the language, providing a convenient and efficient
way to handle reference counting for shared data.

### 9.6 Example

Here's a simple example of using `Gc<T>` to share data in Oxide:

```rust
use std;

@derive(Debug);
type SharedData(value: i32);

fn main() -> ? => try {
    let shared = Gc::new(SharedData(value: 42));

    std::io::println(`The shared value is {shared.value:?}`);
};
```

In this example, `Gc::new()` wraps the `SharedData` type, and you can access its
fields without needing to manage explicit borrows, thanks to the automatic
interior mutability provided by `Gc<T>`.

With `Gc<T>`, Oxide streamlines shared data handling and ensures your
application remains memory-efficient and free from common issues related to
shared data management.

# 10. The Copy Trait in Oxide

In Oxide, the `Copy` trait is a fundamental concept governing the behavior of
types when they are moved. Unlike Rust, where the `Copy` trait means that a type
can be directly duplicated through memory copying (e.g., `memcpy`), Oxide
interprets the `Copy` trait differently.

### 10.1 Implicit Cloning on Move

In Oxide, a type marked as `Copy` doesn't necessarily support low-level memory
copying but indicates a different behavior. When you move a `Copy` type, Oxide
implicitly clones it instead of transferring ownership. This means that the
original value remains intact and accessible in its original location, while a
new copy of the value is created at the target location. This behavior ensures
that changes to one instance of the value do not affect others, allowing
developers to work with data efficiently while maintaining the integrity of the
original.

### 10.2 Interactions with the Clone Trait

In Oxide, when implementing the `Copy` trait, there's no need for explicit
`Clone` trait implementations; developers can use `@derive(Clone)` for
convenience. However, it is important to note that the `Clone` trait is
implicitly assumed to be implemented when defining `Copy`, as `Copy` relies on
the cloning mechanism to perform implicit cloning during moves.

It's important to emphasize that the reverse relationship does not apply. While
a type can implement `Clone` and have the ability to clone explicitly, this does
not necessarily imply that the type can implement `Copy`. The `Copy` trait is
reserved for data types that are implicitly cloned on move, ensuring that they
behave consistently with other `Copy` types.

This revision should accurately represent the relationship between `Copy` and
`Clone` traits in Oxide.

### 10.3 Simplified Data Sharing

The distinction between `Copy` and `Clone` traits in Oxide contributes to more
straightforward data sharing. By understanding the implicit cloning behavior of
`Copy` types on move, developers can manage shared data with minimal effort and
without the need for manual cloning operations. Oxide's approach to the `Copy`
trait streamlines data handling, promoting efficient and reliable code
development.

# 11. Examples

### 11.1 Error Handling

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

# 12. IDE and Tooling Support

Oxide is designed not only with the language itself in mind but also with robust
tooling and development environments. Its official compiler, found under the
"oxide-lang::compiler" module, provides a straightforward and efficient API for
developers, enabling the creation of powerful IDEs and various tools.

### 12.1 The Oxide Compiler API

The Oxide Compiler API allows developers to interact with the compiler
programmatically, making it a valuable tool for building integrated development
environments (IDEs) and other code-related applications. The API provides a
simple yet comprehensive interface to analyze, manipulate, and work with Oxide
source code. Here's a basic example of how to use the API:

```rust
use oxide_lang::compiler::{Compiler, Document};

fn main() { let mut compiler = Compiler::new();

    compiler.mutate(|state| {
        state.add(Document::new(
            "main.ox",
            "fn main() -> ? { ... }",
        });
    });

    let snapshot = compiler.snapshot();
    let model = snapshot.get_semantic_model();

    println!("{:#?}", model);

}
```

Key features of the Oxide Compiler API include:

#### **Incremental Compilation**

The compiler is designed with incremental compilation in mind. It ensures that
only the necessary parts of the code are analyzed and recomputed when changes
are made, making it exceptionally fast. Snapshots, created from the compiler,
provide access to semantic models, and they are both lazy and thread-safe.

#### **Concurrency**

The compiler API is designed with concurrency in mind. It allows for multiple
snapshots to be queried concurrently. Older snapshots can coexist with newer
ones, even while being queried in parallel. This level of concurrency and
parallelism ensures that developers can build high-performance and efficient
development tools.

#### **IDE Integration**

IDEs can leverage the Oxide Compiler API to offer features such as code
analysis, autocompletion, error checking, and more. The incremental nature of
the compiler and the rich semantic models provided by the snapshots enable IDEs
to offer real-time feedback and enhance the development experience for Oxide
users.

### 12.2 Compiler Plugins

A fundamental feature of the Oxide language is the support for compiler plugins
or extensions. These plugins, similar to popular tools like Vite and Webpack for
JavaScript, can extend the capabilities of the Oxide compiler. They can provide
additional functionality for tasks like code optimization, bundling, and more.
The unique aspect of Oxide's approach to compiler plugins is that they are
compiled to WebAssembly (Wasm) and executed using the Wasmtime runtime.

This approach has several advantages:

- **Dynamic Compilation**: Compiler plugins can be dynamically executed at
  compile-time. This dynamic nature allows for versatile and customizable build
  processes without needing to modify the core compiler.

- **Safety**: Plugins run in a WebAssembly sandbox, ensuring that they do not
  have access to sensitive parts of the system. This approach enhances security
  while offering extensibility.

- **Versatility**: Compiler plugins enable the community to extend the
  capabilities of Oxide for various development and deployment scenarios, from
  optimizing code to customizing build pipelines.

### 12.3 Versatile Tooling

With a robust compiler API and support for compiler plugins, the Oxide language
encourages the development of versatile tooling. IDEs, code linters, formatters,
and project builders can be created or enhanced with ease. The incremental
compilation and plugin system further empower tooling developers to offer
efficient and feature-rich solutions for the Oxide ecosystem.

Oxide's commitment to IDE and tooling support ensures that developers have the
necessary tools to write, analyze, and deploy Oxide code efficiently. This
approach not only simplifies the development process but also fosters a thriving
ecosystem around the language.

By providing a powerful and versatile compiler API and enabling dynamic compiler
plugins, Oxide aims to support a wide range of developer needs and create a
seamless development experience.
