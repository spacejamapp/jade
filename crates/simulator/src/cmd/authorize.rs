//! Authorize command

use anyhow::Result;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::{Env, Execution};

/// Run the authorize command
pub fn run(env: &Env) -> Result<Execution> {
    let executed = Interpreter::is_authorized(&env.authorize.code, env.authorize.index);
    let _res = codec::encode(&executed.exec)?;

    Ok(Execution {
        logs: vec![],
        env: env.clone(),
    })
}
