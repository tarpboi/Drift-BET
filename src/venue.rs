//! Drift BET venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Drift BET";

/// Venue category.
pub const VENUE_TYPE: &str = "Solana";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "BTC 5m / 15m / 1hr Arbitrage",
    "Orderbook Imbalance",
    "Market Making",
    "On-Chain Whale Signal",
];
