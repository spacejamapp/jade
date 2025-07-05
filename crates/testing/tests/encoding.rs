//! encoding tests

use codec::JamCodec;
use score::service::ServiceAccount;

#[test]
fn test_account_encoding() {
    let account = ServiceAccount::default();
    let encoded = codec::encode(&account).expect("failed to encode");
    let decoded = ServiceAccount::decode(&encoded[..]).unwrap();
    assert_eq!(account, decoded);
}
