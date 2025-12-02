use antbox_cellauto::{ConwaysLife, Evolvable};
use derive_more::{From, Into};
use derive_new::new;

/// The `antbox` functional, I/O-free [State]
#[derive(Debug, From, Into, new)]
pub struct State {
    /// The generation count
    pub gencnt: usize,
    /// The food grid
    pub food: ConwaysLife,
}

impl Evolvable for State {
    fn evolve(&self) -> Self {
        State {
            gencnt: self.gencnt + 1,
            food: self.food.evolve(),
        }
    }
}
