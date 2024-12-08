## Setup Guide
### Rust Toolchain
Having a fixed version of Rust toolchain is one of the crucial parts of building a deterministic runtime.

Create a `rust-toolchain.toml` file in the root of your project and add the following content:

```toml
[toolchain]
# Using Rust `1.81.0` as an example, you can replace it with your desired version.
channel    = "1.81.0"
# `clippy`, `rust-src` and `rustfmt` are optional, you can remove them if you don't need them.
components = ["cargo", "clippy", "rust-src", "rustc", "rustfmt"]
# `minimal` is the basic and minimal one to build the runtime, you can choose the profile that you need.
# https://rust-lang.github.io/rustup/concepts/profiles.html.
profile    = "minimal"
targets    = ["wasm32-unknown-unknown"]
```

With this file in place, PPR will automatically use the specified version of Rust toolchain.

This is the best way to manage the Rust toolchain version for your project.

If you don't have a `rust-toolchain.toml` file, PPR will use the default version of Rust toolchain installed on your system.

If you don't have any Rust toolchain installed, PPR will exit with an error.

### Docker
The second crucial part of building a deterministic runtime is to use the same environment for building the runtime.

We use Docker to create a deterministic environment for building the runtime.

So, to use the runtime build feature, you need to have Docker installed on your system.

### Run
```sh
prr build \
	--cache-output --cache-registry <RUNTIME_CRATE_NAME>
```
