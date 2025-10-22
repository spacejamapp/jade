# Jade Framework

![main](https://github.com/spacejamapp/jade/actions/workflows/main.yml/badge.svg)
[![crate](https://img.shields.io/crates/v/jade.svg)](https://crates.io/crates/jade)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/jade/)
[![downloads](https://img.shields.io/crates/d/jade.svg)](https://crates.io/crates/jade)
[![telegram](https://img.shields.io/badge/telegram-blue?logo=telegram)](https://t.me/spacejamapp)
[![LICENSE](https://img.shields.io/crates/l/jade.svg)](https://choosealicense.com/licenses/gpl-3.0/)

Jade framework for building JAM services by [SpaceJam](https://spacejam.app).

## Quick Start

```
cargo install jade
jade new my-service
cd my-service
cargo build
cargo test
```

- See [the scripts in the CI][CI_TPL] for a working example
- For the template service generated via `jade new`, check [service-template][template]

## License

GPL-3.0

[CI_TPL]: https://github.com/spacejamapp/service-template/blob/main/.github/workflows/main.yml#L62-L66
[template]: https://github.com/spacejamapp/service-template
