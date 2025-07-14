//! Refine command

use crate::ext;
use anyhow::Result;
use podec::Decode;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::{Env, Execution};

/// Run the refine command
pub fn run(env: &mut Env) -> Result<Execution> {
    let mut accounts = ext::accounts(env);
    let executed = Interpreter::refine(
        env.timeslot,
        &mut accounts,
        env.authorize.index as usize,
        &ext::package(env),
        Default::default(),
        Default::default(),
        Default::default(),
    );
    let res = codec::encode(&executed.executed.exec)?;

    Ok(Execution {
        logs: vec![],
        env: env.clone(),
        gas: executed.executed.gas,
        output: Decode::decode(&mut res.as_slice())?,
    })
}
