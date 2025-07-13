//! Authorize command

use anyhow::Result;
use podec::Decode;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::{Env, Execution};

/// Run the authorize command
pub fn run(env: &Env) -> Result<Execution> {
    let executed = Interpreter::is_authorized(&env.authorize.code, env.authorize.index);
    let res = codec::encode(&executed.exec)?;

    Ok(Execution {
        logs: vec![],
        env: env.clone(),
        gas: executed.gas,
        output: Decode::decode(&mut res.as_slice())?,
    })
}
