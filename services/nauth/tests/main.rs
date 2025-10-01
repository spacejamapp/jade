//! Basic VM tests

use jade::testing::{self, Jam};
use nauth::SERVICE;

const AUTHORIZER: u32 = 500;

#[test]
fn test_nauth() {
    testing::util::init_logger();

    let mut jam = Jam::default().with_auth(AUTHORIZER, SERVICE.to_vec());
    let package = jam
        .send(AUTHORIZER, vec![])
        .expect("failed to send work item");

    let result = jam.authorize(&package, 0).expect("failed to authorize");
    assert!(result.is_ok());
}
