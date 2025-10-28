//! Lookup service end-to-end tests.

use jade::testing::Jam;
use lookup::{
    SERVICE,
    instruction::Instruction,
    storage::{LookupStore, LookupTarget},
};

const AUTHORIZER_ID: u32 = 500;
const SERVICE_ID: u32 = 601;
const SOURCE_ID: u32 = 602;

#[test]
fn test_lookup_caches_preimages() {
    jade::testing::util::init_logger();

    let mut jam = Jam::default().with_auth(AUTHORIZER_ID, nauth::SERVICE.to_vec());
    jam.add_service(SERVICE_ID, SERVICE.to_vec());

    let local_preimage = b"hello from lookup".to_vec();
    let local_hash = jam.add_preimage(SERVICE_ID, local_preimage.clone());

    let external_preimage = b"external data blob".to_vec();
    let external_hash = jam.add_preimage(SOURCE_ID, external_preimage.clone());

    let payload = codec::encode(&vec![
        Instruction::Lookup { hash: local_hash },
        Instruction::LookupFrom {
            service: SOURCE_ID as u64,
            hash: external_hash,
        },
    ])
    .expect("failed to encode payload");

    let info = jam
        .execute(SERVICE_ID, payload)
        .expect("failed to execute lookup request");

    let store: LookupStore = info
        .get_storage(SERVICE_ID, LookupStore::key())
        .expect("lookup store missing");

    let local_target = LookupTarget {
        service: SERVICE_ID as u64,
        hash: local_hash,
    };
    let external_target = LookupTarget {
        service: SOURCE_ID as u64,
        hash: external_hash,
    };

    assert_eq!(
        store
            .get_entry(&local_target)
            .expect("local preimage missing"),
        local_preimage.as_slice()
    );
    assert_eq!(
        store
            .get_entry(&external_target)
            .expect("external preimage missing"),
        external_preimage.as_slice()
    );
}
