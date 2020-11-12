# Rust Template for Graphics!

Congrats on your decision to use Rust for your journey in graphics class!

Go ahead, clone this repo, and get a feel for Rust.

# Environment
You need to install `rustc` and `cargo`. Preferred way is through `rustup`. See [official website] for more info.

Rust is designed to be cross-platform. So there shouldn't be any issue unless you're using a very exotic OS.

I really like using Rust (rust-analyzer) in VSCode. But there's also the [IntelliJ Rust Plugin](https://plugins.jetbrains.com/plugin/8182-rust) to go with most of their IDEs (in particular CLion). Rust Lang team is big on IDE and Tooling support, so it's getting really great even though the language is still relatively young. More options available on the [official website]

[official website]: https://www.rust-lang.org/learn/get-started


# Building Docs

After setting up your env, in your project root directory, run 

`$ cargo doc --document-private-items --open`
or
`$ make doc`
to build the documentation and have `cargo` open it for you automatically in your browser.

To build the documentation only without opening, run
`$ cargo doc --document-private-items`
or
`$ make build-doc`
The docs can be opened by opening `target/graphics/index.html` in your browser.

Things are all hyperlinked together. You can also:

- search for an item
- change the theme on the top left, to the right of the Rust logo

This template is heavily documented to make your life easier.

# Misc

All suggestions are welcomed!