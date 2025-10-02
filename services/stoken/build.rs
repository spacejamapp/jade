//! Build the service

fn main() {
    let _ = cjam::util::build(
        env!("CARGO_PKG_NAME"),
        Some(cjam::ModuleType::Service),
        None,
    )
    .ok();
}
