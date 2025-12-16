/// The type of [Object]s which can be in a [Spot](crate::Spot) in the [State](crate::State)
#[derive(Debug)]
pub enum Object {
    /// A food particle
    Food,
    /// An ant
    Ant,
    /// An ant hole
    AntHole,
}
