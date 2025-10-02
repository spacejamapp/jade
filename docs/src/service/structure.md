# Service Structure

Jade currently supports two types of services: [authorizer service](#authorizer-service) and [general service](#general-service), there is also `corevm service` supported in our toolkit but it has not been
official integrated in JAM implementations yet.

- `[no_std]`: we need to avoid standard rust library on building `riscv64` services
  for making our compiled binaries compatible with the JAM virtual machines.
- `#[jade::is_authorized]`: the entrypoint of the `is_authorized` logic.
- `#[jade::refine]`: the entrypoint of the `refine` logic.
- `#[jade::accumulate]`: the entrypoint of the `accumulate` logic.

> Note that authorizer service and general service are different service types, you cannot
> declare both `#[jade::is_authorized]` and `#[jade::refine]` (or `#[jade::accumulate]`)
> in the same service.

### [Authorizer Service][nauth]

```rust
#![cfg_attr(target_arch = "riscv64", no_std)]
use jade::prelude::{AuthTrace, CoreIndex};

#[jade::is_authorized]
fn is_authorized(_core_index: CoreIndex) -> AuthTrace {
    Default::default()
}
```

### [General Service][stoken]

```rust
#![cfg_attr(target_arch = "riscv64", no_std)]
use jade::prelude::{AuthTrace, CoreIndex};

#[jade::refine]
fn refine(
    core: u16,
    index: u16,
    id: u32,
    payload: Vec<u8>,
    package_hash: OpaqueHash,
) -> Vec<u8> {
    // ... refine logic here
}

#[jade::accumulate]
fn accumulate(now: u32, id: u32, results: Vec<Operand>) -> Option<OpaqueHash> {
    // ... accumulate logic here
}
```

[nauth]: https://github.com/spacejamapp/jade/blob/main/services/nauth/src/lib.rs
[stoken]: https://github.com/spacejamapp/jade/blob/main/services/stoken/src/lib.rs
