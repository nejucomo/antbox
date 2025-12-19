use crate::{Ant, SteppedUpon};

/// Yum!
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Food;

impl SteppedUpon for Food {
    type NewState = Ant;

    fn stepped_upon_by(self, ant: Ant) -> Option<Ant> {
        use Ant::WithFood;

        match ant {
            // `ant` can only hold one: Bonk!
            WithFood => None,
            // `ant` picks me up!
            _ => Some(WithFood),
        }
    }
}
