//! Authorization related stuffs

use crate::Jam;
use service::{OpaqueHash, ServiceId, service::ServiceAccount};

/// Authorization related stuffs
#[derive(Default)]
pub struct Auth {
    /// The authorization token
    pub token: Vec<u8>,

    /// The authorization host
    pub host: ServiceId,

    /// The auth code hash
    pub code_hash: OpaqueHash,

    /// The authorizer config
    pub config: Vec<u8>,
}

impl Auth {
    /// Set the authorization token
    pub fn with_token(mut self, token: Vec<u8>) -> Self {
        self.token = token;
        self
    }

    /// Set the authorizer
    pub fn with_authorizer(mut self, service: ServiceId, code: OpaqueHash) -> Self {
        self.host = service;
        self.code_hash = code;
        self
    }

    /// Set the authorizer config
    pub fn with_config(mut self, config: Vec<u8>) -> Self {
        self.config = config;
        self
    }
}

impl Jam {
    /// Set the authorization
    pub fn with_auth(mut self, service: ServiceId, _code: Vec<u8>) -> Self {
        let mut auth = ServiceAccount::default();
        auth.info.balance = 1000;
        // let hash = auth.add_preimage(code, self.chain.finalized.slot);
        self.auth.host = service;
        // self.auth.code_hash = hash;
        self.add_account(service, auth);
        self
    }

    /// Set the authorization token
    pub fn with_auth_token(mut self, token: Vec<u8>) -> Self {
        self.auth.token = token;
        self
    }

    /// Set the authorizer config
    pub fn with_auth_config(mut self, config: Vec<u8>) -> Self {
        self.auth.config = config;
        self
    }

    /// Set the authorization
    pub fn with_authorizer(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }
}
