//! Accumulate command

use crate::ext;
use anyhow::Result;
use pvm::Invocation;
use pvmi::Interpreter;
use testing::{env::Account, Env};

/// Run the accumulate command
pub fn run(env: &Env) -> Result<Env> {
    let operands = ext::operands(env);
    let state = ext::accumulate_state(env);
    let executed = Interpreter::accumulate(
        state,
        env.timeslot,
        env.id,
        1_000_000,
        operands,
        Default::default(),
    );

    let mut env = env.clone();
    println!("executed: {:?}", executed.context.accounts);
    env.accounts = executed
        .context
        .accounts
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                Account {
                    storage: v.storage,
                    preimage: v.preimage,
                    lookup: v.lookup,
                    code: v.code,
                    balance: v.balance,
                    accumulate_gas: v.accumulate_gas,
                    transfer_gas: v.transfer_gas,
                },
            )
        })
        .collect();
    Ok(env)
}
