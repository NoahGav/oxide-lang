fn main() {}

// Ok, so the language is simple.
// 1). No explicit lifetimes. All lifetimes will be determined by the compiler and validated using borrow checking.
//
// 2). Traits will be implicitly implemented by simply defining all the methods for a type (like go).
// REMOVED: 2 has been removed, traits have to be explicitly implemented. This is to avoid the problems of go
// where if two traits have the same signature, but mean different things, it's impossible for a type to implement
// both. This is still under consideration though, because we could have a macro attribute that can be used to
// allow a type to implement multiple methods with the same signature, but specify which trait it's for specifically.
// The attribute would be @impl(Trait).
//
// 3). No distinction between objects on the heap or stack. Anything where the size cannot be known at runtime
//     will be boxed by the compiler and treated like everything else. This also means that traits can be used
//     like any other type (e.g. Trait instead of Box<dyn Trait>).
//
// 4). Instead of having Rc<T> or Arc<T>, instead there will be a shared type. Anything that is a shared
//     type uses Gc<T>, which is a reference counted cyclic garbage collector (meaning it uses reference
//     counting and only has a cyclic garbage detect to detect cycles, otherwise it's unused). The shared
//     types will be mutable using RefCell (or RwLock if sent across threads). The way it will work is that
//     each field of Gc<Foo> (e.g. bar in struct Foo { bar: i32 }) will be wrapped in a RefCell. Any time
//     you borrow from Foo itself, the compiler treats it as valid and will instead panic at runtime. After
//     you borrow from Foo, the compiler does borrow checking as usual. It's only the initial borrow from
//     each field.
//
// 4). Macros will be far superior to rust and use the @macro(...) syntax (for using it).
//     Macros will just be regular functions. The way they will work is instead of receiving a token
//     stream, they instead receive reflection objects. This is very similar to c# where if you apply
//     a macro to a function, you get a Function object. Macros have full semantic information for the
//     program and are run at compile-time. The way it works is via incremental computation. The semantic model
//     is first built without any macros. Then the macros are run and essentially generate code. Then
//     the semantic model is built again, but since the actual source files haven't changed most of the
//     model won't be recomputed. This will make the 2nd pass very fast (also, only macros whose input
//     has changed will run again making it even faster in subsequent incremental compilations).
//
// 5). Panicking in this language will be strictly forbidden and non-existent. If you want your function
//     to have the potential of failure it will need to return T? instead of T. The ? token indicates that
//     the function may return an Error. Error is a trait that any type can implement. Therefore, creating
//     any custom error type is easy. To allow for easy handling of errors, a function that returns T? can
//     use the ? operator on T? expressions. What it does is simply checks if it was an error, if so it
//     returns the error, otherwise it simply evaluates to the value. Functions that return T? can also use
//     the @bail macro, which is just a shortcut to returning an error at any point. If a function doesn't
//     return anything, but still might fail (like println), then it can return ?.
//
// 6). Traits marked with the @match attribute will have a runtime type-id added to all implementors of the
//     trait. This will then allow you to pattern match that trait. For example, you could pattern match the
//     Error trait in order to handle specific errors differently. Since it's not possible in general for the
//     compiler to know all implementors of a trait at compile-time (obviously), this means all pattern matching
//     done on traits will require the catch-all pattern (_ => ...).
//
// Example:
//
// use std;
//
// @derive(Debug);
// type Foo(bar: i32);
//
// fn Foo.add(self, rhs: i32) -> Self {
//   Self(bar: self.bar + rhs)
// }
//
// fn main() -> ? {
//   let foo = Foo(bar: 41) + 1; // This is possible because Foo implicitly implements the std::ops::Add trait.
//   io::println(`{foo.bar}`)?; // we can use io instead of the fully qualified name std::io because of the "use std;" statement.
// }
