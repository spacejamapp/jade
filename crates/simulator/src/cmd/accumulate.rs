//! Accumulate command

use crate::ext;
use anyhow::Result;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::{Env, Execution};

/// Run the accumulate command
///
/// TODO: large decoding/encoding work here
pub fn run(env: &Env) -> Result<Execution> {
    let state = ext::accumulate_state(env);
    let _executed = Interpreter::accumulate(
        state,
        env.timeslot,
        env.id,
        Default::default(),
        Default::default(),
        Default::default(),
    );

    Ok(Execution {
        logs: vec![],
        env: env.clone(),
    })
}
