# Oxide Language Proposal (Revision #2)

Oxide is a proposal for a new programming language designed to provide many of
the safety and performance benefits of Rust while simplifying the development
process. This document outlines the core features of Oxide, focusing on its
unique approach to borrow checking and lifetimes.

# 1. References in Oxide

One of the most distinctive features of Oxide is its approach to reference
management. In Oxide, all references are reference counted. However, Oxide
combines reference counting with borrowing checking, ensuring that references
are used safely, while eliminating the need for lifetimes.

In this section, we will delve into the details and provide illustrative
examples to demonstrate how Oxide combines reference counting with borrow
checking to ensuring the safe and efficient use of references while eliminating
the need for explicit lifetimes.

### Under the Hood

Oxide's key objective is to simplify language constructs without compromising
safety or performance (as much as possible). In the case of references, Oxide
adopts an internal representation akin to `Arc<RefCell<T>>` as found in Rust.
While this might appear unconventional, Oxide utilizes static analysis to
minimize the need for reference count increments and decrements on `Arc`, and it
efficiently bypasses `RefCell` when the compiler statically determines safety,
which is achieved by using `unsafe { &*t.as_ptr() }` in Rust.

This reference representation in Oxide allows for the efficient passage of
references, nearly matching the performance of Rust. By passing the `Arc` by
reference (`&Arc`) whenever possible and directly accessing the pointer stored
within `RefCell`, Oxide maintains performance while ensuring the reference count
increases when the compiler cannot statically guarantee the lifetime of a borrow
(avoiding common memory-related bugs in languages without features like these).

### Returning References

In Oxide, due to the absence of reference lifetimes, returning references from
functions is facilitated by utilizing `Arc`. Oxide automatically increases the
reference count of the returned reference, statically guaranteeing that it will
persist long enough for its intended usage without requiring knowledge of the
lifetimes of the references passed into the function.

```rust
// Oxide
fn max(a: &i32, b: &i32) -> &i32
    => if a > b { a } else { b };

fn main() {
    let a = 4;
    let b = 3;
    let max = max(&a, &b);

    std::io::println(`{max}`); // prints 4.
}

// Generated Rust Code

// In Oxide, the absence of lifetimes is resolved by returning a cloned
// `Arc` instance of the maximum value, ensuring the reference's lifetime
// is always long enough to be used.
fn max(a: &Arc<RefCell<i32>>, b: &Arc<RefCell<i32>>) -> Arc<RefCell<i32>> {
    // Once again, this is actually perfectly safe.
    if unsafe { &*a.as_ptr() } > unsafe { &*b.as_ptr() } {
        a.clone()
    } else {
        b.clone()
    }
}

fn main() {
    let a = Arc::new(RefCell::new(4));
    let b = Arc::new(RefCell::new(3));
    let max = max(&a, &b);

    // This is safe as Oxide determines that the returned reference
    // should have a lifetime, generally equivalent to min('a, 'b).
    // Since both 'a' and 'b' are still valid at this point, the compiler
    // ensures safe access.
    println!("{}", unsafe { &*max.as_ptr() });
}
```

### Handling Reference Alias Indeterminism

Oxide's approach to reference aliasing goes beyond simplifying reference
lifetimes; it ensures determinism in scenarios where aliasing may not be
statically analyzed. Here, we explore how Oxide manages indeterminism and
runtime borrow checking:

_**Reference Return Mechanism:**_

In Oxide, references can be returned from functions without requiring explicit
lifetimes. This is made possible by incrementing the reference count through the
cloning of `Arc`. This strategy guarantees that the returned reference lives as
long as necessary, all without the need for detailed lifetime annotations.

_**Runtime Borrow Checking:**_

However, in some cases, the compiler might face challenges when attempting to
statically analyze reference aliasing. When the compiler cannot provide a
definitive assurance that the use of some reference follows the strict aliasing
rules, Oxide employs explicit runtime borrow checking.

_**Similar to C# Nullables:**_

In situations where the compiler cannot statically determine compliance with
aliasing rules for a reference, the reference's usage falls back to runtime
borrow checking provided by `RefCell`. This parallels the approach used in C#
nullables, where the C# compiler employs a similar mechanism (except it tracks
whether a reference is or isn't null, and at times marking them as
`may be null here` requiring explicit handling).

_**Explicit Handling:**_

When runtime borrow checking is triggered due to aliasing indeterminism, it
results in a runtime error. This deliberate design choice requires developers to
explicitly manage potential issues, effectively mitigating memory-related bugs
that might arise in languages lacking such features.

## TODO:

- References can be returned from functions without any lifetimes because Oxide
  can simply increment the reference count (by cloning the `Arc`). This will
  ensure that whatever reference is actually returned lives as long as it needs
  to. However, sometimes this can lead to issues where the compiler cannot
  statically analyze the aliasing of references. If this happens, the compiler
  will give an error when accessing a field owned by the reference or
  dereferencing the reference itself (this is a runtime error performed by
  runtime borrow checking, this means it can easily be handled without the need
  for panics). This is very similar to C# nullables (except with reference
  aliasing as references cannot be null in Oxide) where the compiler statically
  checks when it knows if a reference isn't null and if the compiler cannot know
  if the reference might be null, it requires explicit handling to check.

- References stored in structs implicitly have lifetimes. These lifetimes are
  treated as generic parameters that are filled out by inference when creating
  an instance of the struct. This allows references to be safely stored in
  structs without the need for lifetimes at all (while still allowing the
  compiler to statically analyze the program and do borrow checking). However,
  if the compiler cannot determine the lifetime statically, it will instead
  clone the `Arc` (to ensure it lives long enough by increasing the reference
  count) and give the reference the `'static` lifetime. References with the
  `'static` lifetime cannot be known statically to follow the aliasing rules.
  Therefore, whenever accessing a field or dereferencing the reference itself,
  you are required to explicitly handle the potential runtime borrow checking
  error (once again, this is basically like a c# nullable, where the c# compiler
  marks references as `may be null here`).

- References can be passed between threads in almost the same way as rust. The
  compiler will statically analyze the inferred lifetimes of all references and
  determine if they meet the aliasing rules. If they do, then the program will
  work fine. When it comes to immutable references, this is fairly simple, as
  all the compiler has to do is prevent the creating of mutable references
  during the time that the immutable references exist. Passing mutable
  references between threads is more restricted. Only if the compiler can
  statically know that no immutable references exist during the lifetime of the
  mutable reference can it be passed between threads. If the compiler cannot
  know this, then it's simply not possible. If you want to share a shared
  mutable structure between threads (or just shared mutable ownership in
  general) then you must use interior mutability. For example, if you want to
  pass around a reference to a concurrent hashmap, then you just pass it by
  immutable reference and wrap the hashmap shards in a RwLock. This allows the
  hashmap to be mutated with only a `&self` (and not `&mut self`) reference,
  while still statically ensuring no data races.

- Technically, for copy types it would probably be better to use `Arc<Cell<T>>`.

- Maybe the `!` operator (similar to c#) where an expression that may return an
  error is told that we know that it will never be an error (and so if it is, it
  panics, which would be a bug). The `!` operator (similar to the `?` operator)
  would be equivalent to `unwrap` in rust. For example, `let foo = bar()!;`.
  Here we are saying that bar will never return an error and so we are sure `!`
  it will never panic.

- Someone suggested using `[T]` for generics (instead of `<T>`) and `(0)` for
  indexing (instead of `[0]`). I think this might be a good idea. The main issue
  I see currently is it kind of collides with tuples `(i32)` (which would be an
  array of `i32`) and `(i32)` which would be a tuple of only one type.
  Obviously, tuples with only one type should probably never be a thing, but
  still.

## TODO: Examples of how borrow checking works without lifetimes

```rust
// Oxide
fn bar(foo: &i32) {
    std::io::println(`{foo}`);
}

fn main() {
    let foo = 42;
    bar(&foo);
}

// Generated Rust Code
fn bar(foo: &Arc<RefCell<i32>>) {
    // This is actually safe because we have a reference to the Arc.
    // By doing it this way, we prevent the runtime borrow checking
    // when we know a reference is safe to access (due to static
    // borrow checking).
    println!("{}", unsafe { &*foo.as_ptr() });
}

fn main() {
    // Since foo is passed by reference, we wrap it in a Arc.
    let foo = Arc::new(RefCell::new(42));

    // Passing the Arc by reference avoids cloning the Arc
    // (and incrementing the reference count). It also
    // uses a smaller memory footprint.
    bar(&foo);
}
```

```rust
// Oxide
use std;

fn increment(foo: &mut i32) {
    *foo += 1;
}

fn main() {
    let foo = 41;
    
    // The creation of the &mut reference is borrow checked and analyzed
    // for aliasing (which isn't allowed with mutable references). In
    // this case, the compiler can easily see that this reference is
    // allowed.
    foo(&mut foo);
    
    io::println(`{foo}`);
}

// Generated Rust Code
fn increment(foo: &Arc<RefCell<i32>>) {
    // This is safe to do since the mutable reference to foo
    // was already passed to this function. That means if this
    // function is ever called, it's safe to bypass the runtime
    // borrow checking.
    *unsafe { &mut *foo.as_ptr() } += 1;
}

fn main() {
    let foo = Arc::new(RefCell::new(41));
    
    increment(&foo);
    
    println!("{}", unsafe { &*foo.as_ptr() });
}
```

# Implementing a Linked List (TODO)

This is an example implementation of a hypothetical linked list in Oxide.

```rust
// Oxide
type Node(next: Option<&mut Node>);

fn main() {
    let node = Node(next: None);
    let node2 = Node(next: None);

    node.next = Some(&mut node2);
}

// Generated Rust Code
struct Node {
    next: Option<Arc<RefCell<Node>>>,
}

fn main() {
    let node = Arc::new(RefCell::new(Node { next: None }));
    let node2 = Arc::new(RefCell::new(Node { next: None }));

    unsafe { &mut *node.as_ptr() }.next = Some(node2.clone());
}
```
