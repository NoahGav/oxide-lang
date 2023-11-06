# Prototype

### Branch

The main development branch is the
[prototype](https://github.com/NoahGav/oxide-lang/tree/prototype) branch.

### Contributions

To contribute:

1. [Fork](https://github.com/NoahGav/oxide-lang/fork) the repo.
2. Checkout the
   [prototype branch](https://github.com/NoahGav/oxide-lang/tree/prototype)
   (`git checkout prototype`).
3. Make your contributions.
4. Then submit a [pull request](https://github.com/NoahGav/oxide-lang/pulls)
   when you are finished.

### Usage

To use the Oxide binaries:

1. run `cargo build --release`.
2. Add `target/release` to the `Path` environment variable (on Windows).
3. Open a terminal and run a command (e.g. `oxide analyzer`).

To use (and develop) the vscode extension:

1. Open `editors/vscode` in it's own vscode window.
2. Start debugging the extension by pressing `F5`.
3. A new vscode window will open. You should open an Oxide project in that
   window.

# Language Specification

The Oxide language is currently in its prototype stage, and its design is a work
in progress. This section of the README.md serves as a comprehensive repository
of information regarding the language, encompassing high-level goals, features,
language design (including syntax, semantics, and code generation), tooling, and
any other relevant aspects. As the language evolves, we encourage contributors
to actively update this documentation to reflect design decisions and
improvements made by the community. When you make contributions to the language,
please ensure that you modify this section accordingly to keep everyone informed
about the latest developments. Your input is essential in shaping the future of
Oxide.
