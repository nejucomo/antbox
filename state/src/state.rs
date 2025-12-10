use antbox_cellauto::ConwaysLife;
use antbox_geom::Bounds;
use derive_more::{From, Into};
use mealy_machine::IntoNext;

/// The `antbox` functional, I/O-free [State]
#[derive(Debug, From, Into)]
pub struct State {
    /// The generation count
    pub gencnt: usize,
    /// The grid bounds
    pub bounds: Bounds,
    /// The food grid
    pub food: ConwaysLife,
}

impl State {
    /// Initialize the state from a `food` grid
    pub fn new(food: ConwaysLife) -> Self {
        State {
            gencnt: 0,
            bounds: food.bounds(),
            food,
        }
    }
}

impl IntoNext for State {
    fn into_next(self) -> Self {
        State {
            gencnt: self.gencnt + 1,
            bounds: self.bounds,
            food: self.food.into_next(),
        }
    }
}
