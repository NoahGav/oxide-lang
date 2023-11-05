# Prototype

### Branch

The main development branch is the [prototype](https://github.com/NoahGav/oxide-lang/tree/prototype) branch.

### Contributions

To contribute:

1. [Fork](https://github.com/NoahGav/oxide-lang/fork) the repo.
2. Checkout the [prototype branch](https://github.com/NoahGav/oxide-lang/tree/prototype) (`git checkout prototype`).
3. Make your contributions.
4. Then submit a [pull request](https://github.com/NoahGav/oxide-lang/pulls) when you are finished.

### Usage

To use the Oxide binaries:
1. run `cargo build --release`.
2. Add `target/release` to the `Path` environment variable (on Windows).
3. Open a terminal and run a command using (e.g. `oxide analyzer`).

To use (and develop) the vscode extension:
1. Open `editors/vscode` in it's own vscode window.
2. Start debugging the extension by pressing `F5`.
3. A new vscode will open. You should open an Oxide project in that window.
