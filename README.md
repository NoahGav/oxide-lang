# Oxide Language Proposal/Specification

## Introduction

Oxide is a new programming language designed to provide many of the safety and
performance benefits of Rust while simplifying the development process. This
document outlines the core features of Oxide, focusing on its unique approach to
borrow checking and lifetimes.

## Oxide's Simplified Borrow Checking and (Lack of) Lifetimes

One of the most distinctive features of Oxide is its approach to reference
management. In Oxide, all references are reference counted, a departure from
Rust's strict ownership and borrowing model. However, Oxide combines reference
counting with a form of borrowing checking, ensuring that references are used
safely, while significantly reducing the complexity associated with lifetimes.

## TODO:

- It is a Rust inspired language where it has the advantages of rust (borrow
  checking, reference aliasing, deterministic dropping), but without the mental
  overhead and explicitness of rust.

- References are equivalent to `Arc<RefCell<T>>` in rust. This might seem weird,
  but Oxide will statically ensure that all the rules of rust are followed and
  will optimize the code (It will be passed by reference as much as possible
  `&Arc<RefCell<T>>` (to avoid incrementing and decrementing the atomic
  reference count and to avoid memcpying) and the runtime borrow checking of the
  `RefCell` will be bypassed using `unsafe { &*foo.as_ptr() }` whenever it is
  statically proven to be safe (most of the time)).

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
  error (once again, this is basically like a c# nullable, where the compiler
  marks `'static` references as `may be null`).

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

// Rust Equivalent
fn  bar(foo:  &Arc<RefCell<i32>>) {
	// This is actually safe because we have a reference to the Arc.
	// By doing it this way, we prevent the runtime borrow checking
	// when we know a reference is safe to access (due to static
	// borrow checking).
	println!("{}", unsafe { &*foo.as_ptr() });
}

fn  main() {
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
fn max(a: &i32, b: &i32) -> &i32
	=> if a > b { a } else { b };

fn main() {
	let a = 4;
	let b = 3;
	let max = max(&a, &b);
	
	std::io::println(`{max}`); // prints 4.
}

// Rust Equivalent
// Since oxide has no lifetimes, we get around this by returning a clone
// of the max Arc passed in, instead of a reference to the Arc. This increments
// the reference count and so statically guarantees that the reference
// will live at least as long as the user stores it.
fn  max(a:  &Arc<RefCell<i32>>, b:  &Arc<RefCell<i32>>) ->  Arc<RefCell<i32>> {
	if  a  >  b {
		a.clone()
	} else {
		b.clone()
	}
}

fn  main() {
	let  a  =  Arc::new(RefCell::new(4));
	let  b  =  Arc::new(RefCell::new(3));
	let  max  =  max(&a, &b);

	// This is actually safe as oxide can determine that the returned
	// reference must have a lifetime (in the general case) of min('a, 'b).
	// Since both a and b are still valid at this point, the compiler
	// knows that you can safely access it.
	println!("{}", unsafe { &*max.as_ptr() });
}
```
