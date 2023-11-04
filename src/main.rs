use oxide_parser::parse;

fn main() {
    let src = "fn foo(bar: i32)";
    let tree = parse(src);
    println!("{:#?}", tree);

    // tree.nodes.iter().for_each(|node| {
    //     if let Ok(node) = node {
    //         match node {
    //             syntax::Node::FnDecl(fn_decl) => println!("{:?}", tree.text(&fn_decl.name)),
    //         }
    //     }
    // });
}

// Use the "lowering" technique when building the semantic model.
// The lowering technique is where higher level language constructs
// are defined in terms of the lower level constructs. For example,
// a while loop and a for loop can be represented as a for loop.
// This means the semantic model should represent both as for loops.
// This makes the language more robust as well as making analysis
// easier as you have less things to handle.
