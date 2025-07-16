//! Accumulate command

use crate::ext;
use anyhow::Result;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::Env;

/// Run the accumulate command
pub fn run(env: &Env) -> Result<Env> {
    let operands = ext::operands(env);
    let state = ext::accumulate_state(env);
    let _executed = Interpreter::accumulate(
        state,
        env.timeslot,
        env.id,
        1_000_000,
        operands,
        Default::default(),
    );

    Ok(env.clone())
}
