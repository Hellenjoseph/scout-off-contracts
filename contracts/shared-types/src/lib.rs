#![no_std]
use soroban_sdk::{contracttype, Address, String, Vec};

/// Four-tier progress level for a player profile
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ProgressLevel {
    /// Level 0 — profile created, no verification yet
    Unverified,
    /// Level 1 — identity confirmed by academy or KYC
    VerifiedIdentity,
    /// Level 2 — performance milestones verified by approved third party
    PerformanceMilestones,
    /// Level 3 — scout feedback or trial offer logged
    EliteTier,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ContractHealth {
    pub initialized: bool,
    pub paused: bool,
}

impl ProgressLevel {
    /// Returns the next valid level, or None if already at the top.
    pub fn next(&self) -> Option<ProgressLevel> {
        match self {
            ProgressLevel::Unverified => Some(ProgressLevel::VerifiedIdentity),
            ProgressLevel::VerifiedIdentity => Some(ProgressLevel::PerformanceMilestones),
            ProgressLevel::PerformanceMilestones => Some(ProgressLevel::EliteTier),
            ProgressLevel::EliteTier => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Cross-contract shared player and scout types (issue #1455)
// Single authoritative definitions used by registration and its consumers
// (verification, scout_access) to avoid silent drift from mirror types.
// ---------------------------------------------------------------------------

/// Basic player vitals stored on-chain.
/// Shared across registration (producer) and verification/scout_access (consumers).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerVitals {
    pub age: u32,
    pub position: String,
    pub region: String,
    pub nationality: String,
}

/// Full on-chain player profile returned to callers.
/// `level` is derived from the progress contract at read time — it is NOT
/// persisted here.  `progress::get_level` is the single source of truth.
#[contracttype]
#[derive(Clone, Debug)]
pub struct PlayerProfile {
    pub player_id: u64,
    pub wallet: Address,
    pub vitals: PlayerVitals,
    /// IPFS/Arweave CIDs for highlight reels and photos
    pub ipfs_hashes: Vec<String>,
    pub level: ProgressLevel,
    pub registered_at: u64,
    pub updated_at: u64,
}

/// Lightweight player view for scout discovery (no IPFS hashes or wallet).
#[contracttype]
#[derive(Clone, Debug)]
pub struct PlayerSummary {
    pub player_id: u64,
    pub vitals: PlayerVitals,
    pub level: ProgressLevel,
    pub updated_at: u64,
}

/// Internal on-chain player profile (no level — progress contract is the source of truth).
/// Used by the registration contract for storage; not exposed directly to consumers.
#[contracttype]
#[derive(Clone, Debug)]
pub struct StoredPlayerProfile {
    pub player_id: u64,
    pub wallet: Address,
    pub vitals: PlayerVitals,
    /// IPFS/Arweave CIDs for highlight reels and photos
    pub ipfs_hashes: Vec<String>,
    pub registered_at: u64,
    pub updated_at: u64,
}

/// Paginated response from filter_players.
/// `next_cursor` is `0` when there are no more results.
#[contracttype]
#[derive(Clone, Debug)]
pub struct FilterResult {
    pub profiles: Vec<PlayerProfile>,
    /// Pass this value as `cursor` in the next call to continue pagination.
    /// A value of `0` means there are no further results.
    pub next_cursor: u64,
}

/// Scout profile stored on-chain.
/// Shared across registration (producer) and scout_access (consumer).
#[contracttype]
#[derive(Clone, Debug)]
pub struct ScoutProfile {
    pub scout_id: u64,
    pub wallet: Address,
    pub region: String,
    pub verified: bool,
    pub registered_at: u64,
}

/// Validate that a string is a plausible IPFS/Arweave CID.
///
/// Rules:
/// - CIDv0: starts with "Qm", exactly 46 characters, base58btc charset
///   (no 0, O, I, l characters).
/// - CIDv1 (base32): starts with "bafy", 59–128 characters.
pub fn validate_cid(hash: &String) -> Result<(), &'static str> {
    let hash_len = hash.len();
    let bytes = hash.to_bytes();

    let starts_with_qm = bytes.get(0) == Some(b'Q') && bytes.get(1) == Some(b'm');
    let starts_with_bafy = hash_len >= 4
        && bytes.get(0) == Some(b'b')
        && bytes.get(1) == Some(b'a')
        && bytes.get(2) == Some(b'f')
        && bytes.get(3) == Some(b'y');

    if starts_with_qm {
        // CIDv0: exactly 46 chars
        if hash_len != 46 {
            return Err("invalid cid: CIDv0 must be exactly 46 characters");
        }
        // Base58btc: no 0, O, I, l
        for i in 0..hash_len {
            match bytes.get(i) {
                Some(b'0') | Some(b'O') | Some(b'I') | Some(b'l') => {
                    return Err("invalid cid: CIDv0 contains invalid base58btc character");
                }
                _ => {}
            }
        }
        Ok(())
    } else if starts_with_bafy {
        // CIDv1 (base32): 59–128 chars
        if !(59..=128).contains(&hash_len) {
            return Err("invalid cid: CIDv1 must be 59–128 characters");
        }
        Ok(())
    } else {
        Err("invalid cid: must start with 'Qm' (CIDv0) or 'bafy' (CIDv1)")
    }
}
