<div align="center">

# Polkadot Runtime Releaser
### Streamline the process of releasing a new runtime for the Polkadot-SDK-based chain.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/polkadot-runtime-releaser/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/polkadot-runtime-releaser)](https://github.com/hack-ink/polkadot-runtime-releaser/tags)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/polkadot-runtime-releaser?color=red&style=plastic)](https://github.com/hack-ink/polkadot-runtime-releaser)
</div>

## Quick Guide
### CLI Tool
Check [cli/README.md](cli/README.md) for more details.

#### Installation
```sh
# From source code.
git clone https://github.com/hack-ink/polkadot-runtime-releaser
cd polkadot-runtime-releaser
cargo install --path cli
# From crates.io.
cargo install polkadot-runtime-releaser-cli
# From GitHub release.
curl -L ..
```

#### Build Deterministic Runtime
```sh
prr build <RUNTIME_CRATE_NAME>
```

#### Inspect Runtime WASM
```sh
prr inspect <RUNTIME_WASM_FILE>
```

### GitHub Actions
#### Release
TODO

<div align="right">

#### License
<sup>Licensed under [GPL-3.0](LICENSE).</sup>
</div>
