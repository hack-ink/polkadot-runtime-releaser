<div align="center">

# Polkadot Runtime Releaser
### Streamline the process of releasing a new runtime for the Polkadot-SDK-based chain.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/polkadot-runtime-releaser)](https://github.com/hack-ink/polkadot-runtime-releaser/tags)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/polkadot-runtime-releaser?color=red&style=plastic)](https://github.com/hack-ink/polkadot-runtime-releaser)
</div>

Not support Windows.

## Setup Guide
### Rust Toolchain
It's important to have fixed version of Rust toolchain to avoid any compatibility issues.

We use `rustc-1.77.0` as an example, you can replace it with your desired version.

Create a `rust-toolchain.toml` file in the root of your project and add the following content:

```toml
[toolchain]
channel    = "1.77.0"
components = ["cargo", "clippy", "rust-src", "rustc", "rustfmt"]
profile    = "minimal"
targets    = ["wasm32-unknown-unknown"]
```



### Installation


<div align="right">

#### License
<sup>Licensed under [GPL-3.0](LICENSE).</sup>
</div>
