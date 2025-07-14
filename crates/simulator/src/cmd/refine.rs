//! Refine command

use crate::ext;
use anyhow::Result;
use podec::Decode;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::{crypto, env::WorkResult, Env, Execution};

/// Run the refine command
pub fn run(env: &Env) -> Result<Execution> {
    let mut accounts = ext::accounts(env);
    let items = env.package.items.len();
    let mut env = env.clone();
    for index in 0..items {
        let code = accounts
            .get(&env.id)
            .ok_or(anyhow::anyhow!("Service not found"))?
            .code;
        let executed = Interpreter::refine(
            env.timeslot,
            &mut accounts,
            index,
            &ext::package(&env),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        let res = codec::encode(&executed.executed.exec)?;

        env.result.push(WorkResult {
            service_id: env.id,
            code_hash: code,
            payload_hash: crypto::blake2b(&env.package.items[index].payload),
            accumulate_gas: 0,
            result: Decode::decode(&mut res.as_slice())?,
            refine_load: Default::default(),
        });
    }

    Ok(Execution { logs: vec![], env })
}
