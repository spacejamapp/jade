//! Build the service

fn main() {
    cjam::util::build(
        env!("CARGO_PKG_NAME"),
        Some(cjam::ModuleType::Authorizer),
        None,
    )
    .expect("Failed to build service");
}
