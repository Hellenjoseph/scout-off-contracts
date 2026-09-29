pub use scoutchain_shared_types::ContractHealth;
use soroban_sdk::{contracttype, Address, String, Vec};

/// Richer validator status — distinguishes unregistered from revoked.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ValidatorStatus {
    NotRegistered,
    Active,
    Revoked,
}

/// A single verified milestone record
#[contracttype]
#[derive(Clone, Debug)]
pub struct Milestone {
    pub player_id: u64,
    pub validator: Address,
    pub description: String,
    /// IPFS/Arweave CID of supporting evidence (video clip, stat sheet, etc.)
    pub evidence_hash: String,
    pub approved_at: u64,
    /// Stellar ledger sequence at time of approval for tamper-proof auditability
    pub ledger_sequence: u32,
}

/// Validator entry in the trusted registry
#[contracttype]
#[derive(Clone, Debug)]
pub struct Validator {
    pub wallet: Address,
    /// Human-readable credential label (e.g. "UEFA B License", "Academy Director")
    pub credentials: String,
    pub registered_at: u64,
    pub active: bool,
}

/// Entry in the global milestone index for on-chain auditability.
#[contracttype]
#[derive(Clone, Debug)]
pub struct GlobalMilestoneEntry {
    pub player_id: u64,
    pub milestone_index: u32,
}

/// Paginated response for global milestone index queries.
#[contracttype]
#[derive(Clone, Debug)]
pub struct GlobalMilestoneIndexPage {
    pub entries: Vec<GlobalMilestoneEntry>,
    pub total: u32,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Initialized,
    Paused,
    ProgressContract,
    ProgressContractSet,
    Validator(Address),
    MilestoneCounter(u64),
    Milestone(u64, u32),
    ValidatorMilestoneCount(Address),
    ValidatorPlayerMilestoneCount(Address, u64),
    ValidatorVector,
    TotalMilestoneCount,
    GlobalMilestoneIndex,
    /// Stores a MilestoneDispute record keyed by (player_id, milestone_index).
    MilestoneDispute(u64, u32),
    /// Configuration keys for security-relevant parameters (issue #1453).
    DiversityConfig,
    MinRegionQuorum,
    MilestoneThreshold,
    VotingWindowSecs,
    RegCooldown,
    JuryConfig,
}

// ---------------------------------------------------------------------------
// Configuration types (issue #1453)
// ---------------------------------------------------------------------------

/// Controls diversity requirements for milestone approval (e.g., minimum unique regions).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DiversityConfig {
    /// Minimum number of distinct validator regions required for an approval.
    pub min_unique_regions: u32,
    /// Minimum number of distinct validators required for an approval.
    pub min_unique_validators: u32,
}

/// Controls the minimum regional quorum needed for dispute resolution.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MinRegionQuorum {
    pub quorum: u32,
}

/// Threshold for milestone approval votes.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MilestoneThreshold {
    /// Minimum approval votes needed (absolute count).
    pub min_votes: u32,
    /// Minimum approval ratio in basis points (0–10000 = 0%–100%).
    pub approval_bps: u32,
}

/// Voting window duration for disputes, in seconds.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct VotingWindowSecs {
    pub secs: u64,
}

/// Cooldown period between player registrations, in seconds.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RegCooldown {
    pub secs: u64,
}

/// Jury configuration for dispute resolution.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct JuryConfig {
    /// Number of validators drawn as jurors for each dispute.
    pub jury_size: u32,
    /// Minimum votes required from the jury to reach a verdict.
    pub quorum: u32,
}
