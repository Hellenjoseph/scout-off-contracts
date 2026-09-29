#![allow(deprecated)]
use soroban_sdk::{Address, Env, String, Symbol};

pub fn milestone_approved(
    env: &Env,
    player_id: u64,
    validator: &Address,
    milestone_index: u32,
    description: &String,
    evidence_hash: &String,
) {
    env.events().publish(
        (
            Symbol::new(env, "milestone_approved"),
            validator.clone(),
            milestone_index,
        ),
        (player_id, description.clone(), evidence_hash.clone()),
    );
}

pub fn validator_registered(env: &Env, wallet: &Address) {
    env.events()
        .publish((Symbol::new(env, "validator_registered"),), wallet.clone());
}

pub fn validator_revoked(env: &Env, wallet: &Address, reason: &String) {
    env.events().publish(
        (Symbol::new(env, "validator_revoked"),),
        (wallet.clone(), reason.clone()),
    );
}

pub fn contract_paused(env: &Env, admin: &Address) {
    env.events()
        .publish((Symbol::new(env, "contract_paused"),), admin.clone());
}

pub fn contract_unpaused(env: &Env, admin: &Address) {
    env.events()
        .publish((Symbol::new(env, "contract_unpaused"),), admin.clone());
}

pub fn contract_initialized(env: &Env, admin: &Address) {
    env.events()
        .publish((Symbol::new(env, "contract_initialized"),), admin.clone());
}

pub fn progress_contract_updated(env: &Env, progress_contract: &Address) {
    env.events().publish(
        (Symbol::new(env, "progress_contract_updated"),),
        progress_contract.clone(),
    );
}

/// Emitted when a player disputes a milestone (issue #471)
pub fn milestone_disputed(
    env: &Env,
    player_id: u64,
    milestone_index: u32,
    reason: &String,
) {
    env.events().publish(
        (Symbol::new(env, "milestone_disputed"), player_id, milestone_index),
        (),
    );
}

// ---------------------------------------------------------------------------
// Config setter events (issue #1453)
// Each setter emits a typed event with old and new values for off-chain indexing.
// ---------------------------------------------------------------------------

/// Emitted when diversity_config is updated by admin.
pub fn diversity_config_updated(
    env: &Env,
    admin: &Address,
    old_min_unique_regions: u32,
    old_min_unique_validators: u32,
    new_min_unique_regions: u32,
    new_min_unique_validators: u32,
) {
    env.events().publish(
        (Symbol::new(env, "diversity_config_updated"), admin.clone()),
        (old_min_unique_regions, old_min_unique_validators, new_min_unique_regions, new_min_unique_validators),
    );
}

/// Emitted when min_region_quorum is updated by admin.
pub fn min_region_quorum_updated(
    env: &Env,
    admin: &Address,
    old_quorum: u32,
    new_quorum: u32,
) {
    env.events().publish(
        (Symbol::new(env, "min_region_quorum_updated"), admin.clone()),
        (old_quorum, new_quorum),
    );
}

/// Emitted when milestone_threshold is updated by admin.
pub fn milestone_threshold_updated(
    env: &Env,
    admin: &Address,
    old_min_votes: u32,
    old_approval_bps: u32,
    new_min_votes: u32,
    new_approval_bps: u32,
) {
    env.events().publish(
        (Symbol::new(env, "milestone_threshold_updated"), admin.clone()),
        (old_min_votes, old_approval_bps, new_min_votes, new_approval_bps),
    );
}

/// Emitted when voting_window_secs is updated by admin.
pub fn voting_window_secs_updated(
    env: &Env,
    admin: &Address,
    old_secs: u64,
    new_secs: u64,
) {
    env.events().publish(
        (Symbol::new(env, "voting_window_secs_updated"), admin.clone()),
        (old_secs, new_secs),
    );
}

/// Emitted when reg_cooldown is updated by admin.
pub fn reg_cooldown_updated(
    env: &Env,
    admin: &Address,
    old_secs: u64,
    new_secs: u64,
) {
    env.events().publish(
        (Symbol::new(env, "reg_cooldown_updated"), admin.clone()),
        (old_secs, new_secs),
    );
}

/// Emitted when jury_config is updated by admin.
pub fn jury_config_updated(
    env: &Env,
    admin: &Address,
    old_jury_size: u32,
    old_quorum: u32,
    new_jury_size: u32,
    new_quorum: u32,
) {
    env.events().publish(
        (Symbol::new(env, "jury_config_updated"), admin.clone()),
        (old_jury_size, old_quorum, new_jury_size, new_quorum),
    );
}
