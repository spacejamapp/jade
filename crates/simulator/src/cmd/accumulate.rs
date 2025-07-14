//! Accumulate command

use crate::ext;
use anyhow::Result;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::{Env, Execution};

/// Run the accumulate command
pub fn run(env: &Env) -> Result<Execution> {
    let state = ext::accumulate_state(env);

    // TODO: get operands from env.result
    let operands = ext::operands(env);
    let _executed = Interpreter::accumulate(
        state,
        env.timeslot,
        env.id,
        Default::default(),
        operands,
        Default::default(),
    );

    Ok(Execution {
        logs: vec![],
        env: env.clone(),
    })
}
