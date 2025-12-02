use antbox_cellauto::Generation;
use derive_more::{From, Into};
use derive_new::new;

/// A [Notification] from the engine to the app
#[derive(Debug, From, new)]
pub enum Notification {
    /// There's a new food-layer generation
    NewFoodGeneration(NewFoodGeneration),
}

/// A [Notification] about the new food-layer generation
#[derive(Debug, From, Into, new)]
pub struct NewFoodGeneration {
    /// The incrementing age of the generation
    pub age: usize,
    /// The generation's grid
    pub grid: Generation,
}
