use compiler::COMPILER;

fn main() {
    COMPILER.initialize(
        "",
        compiler::InitializeOptions {
            watch: compiler::Watch::No,
            block: compiler::Block::No,
        },
    );

    COMPILER.open_file("main.ox", "fn foo(bar: i32) {}");

    let snapshot = COMPILER.snapshot();
    let parsed = snapshot.parse("main.ox");
    println!("{}", snapshot.foo(parsed));
}
