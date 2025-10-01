//! Build the service

fn main() {
    cjam::util::build(
        env!("CARGO_PKG_NAME"),
        Some(cjam::ModuleType::Service),
        None,
    )
    .expect("Failed to build service");
}
