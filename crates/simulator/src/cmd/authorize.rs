//! Authorize command

use anyhow::Result;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::Env;

/// Run the authorize command
pub fn run(env: &Env) -> Result<Env> {
    let executed = Interpreter::is_authorized(&env.authorize.code, env.authorize.index);

    // TODO: AUTH output should be embedded in ENV
    let _res = codec::encode(&executed.exec)
        .map_err(|e| anyhow::anyhow!("Failed to encode authorize result: {e}"))?;

    Ok(env.clone())
}
