use crate::{Ant, SteppedUpon};

/// How much life-force is spent to spawn a new ant
const LF_SPAWN_ANT: usize = 10;

/// How much life force is gained when an ant returns
const LF_ANT_RETURNS: usize = LF_SPAWN_ANT - 3;

/// How much life force is gained when food is returned
const LF_FOOD_RETURNS: usize = 10;

/// An [AntHole] collects food for its lifeforce and uses that to spawn ants
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AntHole {
    lifeforce: usize,
}

impl SteppedUpon for AntHole {
    type NewState = Self;

    fn stepped_upon_by(self, ant: Ant) -> Option<Self> {
        Some(AntHole {
            lifeforce: self.lifeforce
                + LF_ANT_RETURNS
                + if matches!(ant, Ant::WithFood) {
                    // Good job, drone!
                    LF_FOOD_RETURNS
                } else {
                    // Whoops
                    0
                },
        })
    }
}

impl Default for AntHole {
    fn default() -> Self {
        Self { lifeforce: 30 }
    }
}
