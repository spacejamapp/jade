//! Build the service

fn main() {
    cjam::build(env!("CARGO_PKG_NAME"), Some(cjam::ModuleType::Authorizer)).ok();
}
