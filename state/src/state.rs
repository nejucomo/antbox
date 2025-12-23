use crate::Field;
use crate::gencount::GenerationCounter;

/// The full [State] is a generation counter and a [Field]
pub type State = GenerationCounter<Field>;

// /// The `antbox` functional, I/O-free [State]
// #[derive(Debug, From, Into, Deref)]
// #[deref(forward)]
// pub struct State(GenerationCounter<Field>);

// impl State {
//     /// Construct a new state from a [Grid]
//     pub fn new(grid: Grid<Spot>, clife: Grid<bool>) -> Self {
//         State(GenerationCounter::new(Field::new(grid, clife)))
//     }
// }

// impl<R> TakeIntoNext<&mut R> for State
// where
//     R: rand::Rng,
// {
//     type Next = Self;

//     fn take_into_next(self, input: &mut R) -> Self::Next {
//         State(self.0.take_into_next(input))
//     }
// }
