//! jade command line interface

#[cfg(not(target_arch = "riscv64"))]
fn main() {
    use cjam::{clap::Parser, cmd::App};
    let app = App::parse();
    app.run().unwrap()
}

#[cfg(target_arch = "riscv64")]
fn main() {}
