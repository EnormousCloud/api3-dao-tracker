use crate::nice;
use sauron::prelude::*;
use web3::types::U256;

pub fn staking_note<T>(apr: f64, stake_target: U256, total_staked: U256) -> Node<T> {
    let reached = nice::dec(stake_target, 10) <= nice::dec(total_staked, 18);
    let is_min = apr <= 0.025;
    let is_max = apr >= 0.75;

    let (color, txt) = if is_min {
        (
            "var(--color-error)",
            "DAO staking target is reached, and APR is at its minimum of 2.5%",
        )
    } else if is_max {
        (
            "var(--color-accent)",
            "DAO staking target is not reached, and APR is at its maximum of 75%",
        )
    } else if reached {
        ("var(--color-error)", "DAO staking target is reached, so APR will be decreased by 1% for the next epoch until it reaches 2.5%")
    } else {
        ("var(--color-accent)", "DAO staking target is not reached, so APR will be increased by 1% for the next epoch until it reaches 75%")
    };

    node! {
        <p class="note" title={format!("apr={} target={} total={}", apr, nice::dec(stake_target, 10), nice::dec(total_staked, 18))} style="text-align: center">
            {span(
                vec![styles([("color", color)])],
                vec![text(txt)],
            )}
        </p>
    }
}
