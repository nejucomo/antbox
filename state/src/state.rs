use antbox_cellauto::ConwaysLife;
use derive_more::{From, Into};
use derive_new::new;
use mealy_machine::IntoNext;

/// The `antbox` functional, I/O-free [State]
#[derive(Debug, From, Into, new)]
pub struct State {
    /// The generation count
    pub gencnt: usize,
    /// The food grid
    pub food: ConwaysLife,
}

impl IntoNext for State {
    fn into_next(self) -> Self {
        State {
            gencnt: self.gencnt + 1,
            food: self.food.into_next(),
        }
    }
}
