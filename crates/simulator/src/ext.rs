//! Extended methods for the environment

use jam_pvm_common::jam_types::WorkError;
use score::{
    safrole::ValidatorData,
    service::{Authorizer, RefineContext, ServiceAccount, WorkExecResult, WorkItem, WorkPackage},
    vm::{AccumulateState, Operand},
};
use std::collections::BTreeMap;
use testing::Env;

/// Get the accounts of the environment
pub fn accounts(env: &Env) -> BTreeMap<u32, ServiceAccount> {
    let mut accounts = BTreeMap::new();
    for (id, account) in env.accounts.iter() {
        let acc = ServiceAccount {
            code: account.code,
            storage: account.storage.clone(),
            preimage: account.preimage.clone(),
            lookup: account.lookup.clone(),
            balance: account.balance,
            accumulate_gas: account.accumulate_gas,
            transfer_gas: account.transfer_gas,
        };
        accounts.insert(*id, acc);
    }
    accounts
}

/// Get the work package of the environment
pub fn package(env: &Env) -> WorkPackage {
    let mut package = WorkPackage::default();
    package.authorization = env.package.authorization.0.clone();
    package.auth_code_host = env.package.auth_code_host;
    package.authorizer = Authorizer {
        code_hash: env.package.authorizer.code_hash.0,
        params: env.package.authorizer.config.0.clone(),
    };
    package.context = RefineContext {
        anchor: env.package.context.anchor.0,
        state_root: env.package.context.state_root.0,
        beefy_root: env.package.context.beefy_root.0,
        lookup_anchor: env.package.context.lookup_anchor.0,
        lookup_anchor_slot: env.package.context.lookup_anchor_slot,
        prerequisites: Default::default(),
    };
    for item in env.package.items.iter() {
        package.items.push(WorkItem {
            service: item.service,
            code_hash: item.code_hash.0,
            payload: item.payload.0.clone(),
            refine_gas_limit: item.refine_gas_limit,
            accumulate_gas_limit: item.accumulate_gas_limit,
            import_segments: Default::default(),
            extrinsic: Default::default(),
            export_count: item.export_count,
        });
    }
    package
}

/// Get the accumulate state of the environment
pub fn accumulate_state(env: &Env) -> AccumulateState<BTreeMap<u32, ServiceAccount>> {
    let accounts = self::accounts(env);
    let mut state = AccumulateState {
        accounts,
        validators: Default::default(),
        authorization: Default::default(),
        privileges: Default::default(),
    };

    for validator in env.validators.iter() {
        state.validators.push(ValidatorData {
            bandersnatch: validator.bandersnatch,
            ed25519: validator.ed25519,
            bls: validator.bls,
            metadata: validator.metadata,
        });
    }

    state
}

/// Get the operands from the environment
pub fn operands(env: &Env) -> Vec<Operand> {
    let mut operands = Vec::new();
    for result in env.result.iter() {
        operands.push(Operand {
            package: Default::default(),
            exports_root: Default::default(),
            authorizer_hash: Default::default(),
            payload: result.payload_hash,
            gas: result.accumulate_gas,
            data: match &result.result {
                Ok(data) => WorkExecResult::Ok(data.clone()),
                Err(e) => match e {
                    WorkError::OutOfGas => WorkExecResult::OutOfGas,
                    WorkError::Panic => WorkExecResult::Panic,
                    WorkError::BadExports => WorkExecResult::BadExports,
                    WorkError::BadCode => WorkExecResult::BadCode,
                    WorkError::CodeOversize => WorkExecResult::CodeOversize,
                },
            },
            auth_output: Default::default(),
        });
    }

    operands
}

/// Convert the work exec result to a vector of bytes
pub fn result(result: WorkExecResult) -> Result<Vec<u8>, WorkError> {
    match result {
        WorkExecResult::Ok(data) => Ok(data),
        WorkExecResult::OutOfGas => Err(WorkError::OutOfGas),
        WorkExecResult::Panic => Err(WorkError::Panic),
        WorkExecResult::BadExports => Err(WorkError::BadExports),
        WorkExecResult::BadCode => Err(WorkError::BadCode),
        WorkExecResult::CodeOversize => Err(WorkError::CodeOversize),
    }
}
