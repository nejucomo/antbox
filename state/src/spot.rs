use antbox_clife::ConwayCell;

use crate::Object;

/// Every [Spot] in the [State](crate::State) can contain up to one [Object]
#[derive(Default)]
pub struct Spot(Option<Object>);

impl Spot {
    /// If this spot is unoccupied
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn as_ref(&self) -> Option<&Object> {
        self.0.as_ref()
    }
}

impl ConwayCell for Spot {
    fn is_alive(&self) -> bool {
        self.as_ref()
            .map(|obj| matches!(obj, Object::Food))
            .unwrap_or(false)
    }

    fn set_alive(&mut self, alive: bool) {
        if alive && self.is_empty() {
            // We only set food life for empty spots; if it already contains food, this is a no-op, but if it contains an ant or anthole, those aren't overwritten.
            self.0 = Some(Object::Food);
        } else if !alive && self.is_alive() {
            // It was `Food` so set it to nothing:
            self.0 = None;
        }
    }
}

impl std::fmt::Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Object::*;

        write!(
            f,
            "{}",
            match self.0 {
                Some(Food) => '*',
                Some(Ant) => '8',
                Some(AntHole) => 'O',
                None => '_',
            }
        )
    }
}
