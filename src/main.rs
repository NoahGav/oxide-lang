use oxide_parser::parse;

fn main() {
    let src = "fn main()";
    println!("{:#?}", parse(src));
}

// Use the "lowering" technique when building the semantic model.
// The lowering technique is where higher level language constructs
// are defined in terms of the lower level constructs. For example,
// a while loop and a for loop can be represented as a for loop.
// This means the semantic model should represent both as for loops.
// This makes the language more robust as well as making analysis
// easier as you have less things to handle.
