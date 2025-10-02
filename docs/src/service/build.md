# Build Service

There are two ways to build a service: using jade command line tool or using rust build script.

## Using jade command line tool

```bash
cargo install jade
cd my-service
jade build
```

## Using rust build script

```toml
# my-service/Cargo.toml
#
# ...
#
[build-dependencies]
cjam = "*"
```

```rust
// my-service/build.rs

fn main() {
    cjam::util::build(
        env!("CARGO_PKG_NAME"),
        // or Some(cjam::ModuleType::Authorizer)
        Some(cjam::ModuleType::Service),
    ).ok();
}
```
