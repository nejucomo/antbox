/// Given if a cell is `already_alive` and the `neighbor_count` return whether it would be alive in the next iteration according to Conway's Life rule
pub fn conways_rule(already_alive: bool, neighbor_count: usize) -> bool {
    match neighbor_count {
        // underpopulation or over-exposure:
        0 | 1 | 4..=8 => false,

        // With 2 neighbors, we can stay alive only if we were previously alive:
        2 => already_alive,

        // birth or persistence
        3 => true,

        // Only up to 8 cells are possible:
        n => panic!("incoherent neighbor count: {n:?}"),
    }
}
