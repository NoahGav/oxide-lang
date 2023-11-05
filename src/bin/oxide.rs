use clap::{command, Command};

/// The "oxide" binary is the entry point to all operations for Oxide.
/// It is just a simple cli tool that is used to launch other binaries.
/// Most of the binaries for Oxide will be language tools (obviously).
/// These language tools will almost all use the Oxide compiler. To
/// ensure safety between these tools, they should always acquire a
/// lock on the lock file for their duration. Theres really only one
/// exception to this rule which is the analyzer (oxide-analyzer). The
/// analyzer can (and will) run in parallel with other tools (more like
/// the other tools run in parallel with the analyzer).
fn main() {
    let matches = command!()
        .name("oxide")
        .subcommand(Command::new("analyzer").about("Starts the oxide-analyzer language server"))
        .get_matches();

    if matches.subcommand_matches("analyzer").is_some() {
        std::process::Command::new("oxide-analyzer")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
