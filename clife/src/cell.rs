/// A [Grid](antbox_geom::Grid) cell type which provides an `is_alive`/`set_alive` interface for Conway's Life
pub trait ConwayCell {
    /// Whether or not the cell is alive according to Conway's Life
    fn is_alive(&self) -> bool;
    /// Set the life status according to Conway's Life
    fn set_alive(&mut self, alive: bool);
}

impl ConwayCell for bool {
    fn is_alive(&self) -> bool {
        *self
    }

    fn set_alive(&mut self, alive: bool) {
        *self = alive;
    }
}
