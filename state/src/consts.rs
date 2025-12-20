//! Centralized magic tunable constants
//!
//! # TODO
//!
//! Pull magic constants into this module.

use crate::wcoin::WeightedCoin;

/// The top-level chance of a pod updating
pub(crate) const WCOIN_POD_UPDATES: WeightedCoin = WeightedCoin(1, 97);

/// The chance of a new pod appearing
pub(crate) const WCOIN_POD_APPEARS: WeightedCoin = WeightedCoin(1, 11);

/// The chance of a old pod disappearing
pub(crate) const WCOIN_POD_DISAPPEARS: WeightedCoin = WeightedCoin(1, 7);

/// The chance losing lifeforce
pub(crate) const WCOIN_LIFE_FORCE_LOSS: WeightedCoin = WeightedCoin(1, 31);

/// The denominator for a seed growth/death given numerator = neighbor_count
pub(crate) const SEED_CHANGE_DENOM: u32 = 10;

/// The denominator for a life growth/death given numerator = neighbor_count
pub(crate) const LIFE_CHANGE_DENOM: u32 = 13;

/// How much life-force is spent to spawn a new ant
pub(crate) const LIFE_FORCE_SPAWN_ANT: usize = 10;

/// How much life force is gained when an ant returns
pub(crate) const LIFE_FORCE_ANT_RETURNS: usize = LIFE_FORCE_SPAWN_ANT - 3;

/// How much life force is gained when food is returned
pub(crate) const LIFE_FORCE_FOOD_LIFE: usize = 10;

/// How much life force is gained when food is returned
pub(crate) const LIFE_FORCE_FOOD_SEED: usize = 1;
