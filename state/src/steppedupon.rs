use crate::Ant;

/// Types which can react to being stepped upon
pub trait SteppedUpon: Sized + Copy {
    /// Getting stepped upon may produce a different value in the state type tree
    ///
    /// Example: `Food` getting stepped upon by an `Ant` produces a new `Ant` state
    type NewState;

    /// If ant successfully steps on us, return our new state
    ///
    /// If a new state is produced, `ant` is "consumed"/"dropped" from the game state. Otherwise `ant` persists
    fn stepped_upon_by(self, ant: Ant) -> Option<Self::NewState>;
}
