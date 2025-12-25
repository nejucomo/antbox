// Use sub-mods to ensure `use` namespaces are uncontaminated;

mod providers {
    use crate::{State, TakeIntoNext};

    /// I don't trigger any blanket extensions (unless `T = ()`)
    #[derive(Default)]
    pub struct NonExtended;

    impl<T> TakeIntoNext<T> for NonExtended {
        type Next = ();

        fn take_into_next(self, _: T) -> Self::Next {}
    }

    #[derive(Default)]
    pub struct Accumulator(usize);

    impl TakeIntoNext<usize> for Accumulator {
        type Next = State<Self>;

        fn take_into_next(self, input: usize) -> Self::Next {
            Accumulator(self.0 + input).into()
        }
    }
}

#[allow(unused_imports, dead_code)] // Temporary while we build out cases
mod consumers {
    // This `use` namespace is used via `*` in test cases, so it must only include a precise non-polluting set. In particular, it must not include any of the crate's traits to ensure we're checking blanket extension existence correctly.
    use super::providers::{Accumulator, NonExtended};

    mod is_into_halting_state {
        use super::*;
        use test_case::test_case;

        use crate::IntoHaltingState;

        fn test<T: IntoHaltingState>(_: T) {}
    }

    mod is_into_halting_stout {
        use super::*;
        use test_case::test_case;

        use crate::IntoHaltingStout;

        fn test<T: IntoHaltingStout<O>, O>(_: T) {}
    }

    mod is_take_into_halting_state {
        use super::*;
        use test_case::test_case;

        use crate::TakeIntoHaltingState;

        #[test_case(Accumulator::default(), 2)]
        fn test<T: TakeIntoHaltingState<I>, I>(_: T, _: I) {}
    }

    mod is_take_into_halting_stout {
        use super::*;
        use test_case::test_case;

        use crate::TakeIntoHaltingStout;

        fn test<T: TakeIntoHaltingStout<I, O>, I, O>(_: T, _: I) {}
    }

    mod is_into_next {
        use super::*;
        use test_case::test_case;

        use crate::IntoNext;

        #[test_case(NonExtended)]
        fn test<T: IntoNext>(_: T) {}
    }

    mod is_into_state {
        use super::*;
        use test_case::test_case;

        use crate::IntoState;

        fn test<T: IntoState>(_: T) {}
    }

    mod is_take_into_state {
        use super::*;
        use test_case::test_case;

        use crate::TakeIntoState;

        #[test_case(Accumulator::default(), 5)]
        fn test<T: TakeIntoState<I>, I>(_: T, _: I) {}
    }

    mod is_into_stout {
        use super::*;
        use test_case::test_case;

        use crate::IntoStout;

        fn test<T: IntoStout<O>, O>(_: T) {}
    }

    mod is_take_into_stout {
        use super::*;
        use test_case::test_case;

        use crate::TakeIntoStout;

        fn test<T: TakeIntoStout<I, O>, I, O>(_: T, _: I) {}
    }

    mod is_take_into_next {
        use super::*;
        use test_case::test_case;

        use crate::TakeIntoNext;

        #[test_case(NonExtended, ())]
        #[test_case(NonExtended, 2)]
        #[test_case(Accumulator::default(), 11)]
        fn test<T: TakeIntoNext<I>, I>(_: T, _: I) {}
    }
}
