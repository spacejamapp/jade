//! encoding tests

use codec::JamCodec;
use score::service::ServiceAccount;
use testing::ext;

#[test]
fn test_account_encoding() {
    let account = ServiceAccount::default();
    let encoded = codec::encode(&account).expect("failed to encode");
    let decoded = ServiceAccount::decode(&encoded[..]).unwrap();
    assert_eq!(account, decoded);
}

#[test]
fn test_storage_key() {
    let holders = b"holders";
    let tkey = ext::storage_key(300, holders);
    let skey = score::state::account::storage(300, holders);
    assert_eq!(tkey, skey);
}
