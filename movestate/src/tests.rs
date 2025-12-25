// Use sub-mods to ensure `use` namespaces are uncontaminated;

mod providers {
    use crate::{Halting, State, Stout, TakeIntoNext};

    /// I don't trigger any blanket extensions (unless `T = ()`)
    #[derive(Default)]
    pub struct NonExtended;

    impl<T> TakeIntoNext<T> for NonExtended {
        type Next = ();

        fn take_into_next(self, _: T) -> Self::Next {}
    }

    /// `S -> [S]`: `IntoHaltingState`
    #[derive(Default)]
    pub struct UpToTen(u8);

    impl TakeIntoNext<()> for UpToTen {
        type Next = Halting<State<Self>>;

        fn take_into_next(self, (): ()) -> Self::Next {
            let n = self.0 + 1;
            if n >= 10 { None } else { Some(UpToTen(n)) }.into()
        }
    }

    /// `S -> [S, O]`: `IntoHaltingStout`
    #[derive(Default)]
    pub struct Countdown(pub usize);

    impl TakeIntoNext<()> for Countdown {
        type Next = Halting<Stout<Self, usize>>;

        fn take_into_next(self, (): ()) -> Self::Next {
            let n = self.0 - 1;
            if n > 0 {
                Some((Countdown(n), self.0))
            } else {
                None
            }
            .into()
        }
    }

    /// `(S, I) -> [S]`: `TakeIntoHaltingState`
    #[derive(Default)]
    pub struct OverflowAccumulator(u8);

    impl TakeIntoNext<u8> for OverflowAccumulator {
        type Next = Halting<State<Self>>;

        fn take_into_next(self, input: u8) -> Self::Next {
            self.0.checked_add(input).map(OverflowAccumulator).into()
        }
    }

    /// `(S, I) -> [S, O]`: `TakeIntoHaltingStout`
    #[derive(Default)]
    pub struct OverflowingSuccessCounter {
        successes: u8,
        failures: u8,
    }

    impl TakeIntoNext<bool> for OverflowingSuccessCounter {
        type Next = Halting<Stout<Self, usize>>;

        fn take_into_next(mut self, success: bool) -> Self::Next {
            use Halting::*;

            let ctr = if success {
                &mut self.successes
            } else {
                &mut self.failures
            };
            if let Some(v) = ctr.checked_add(1) {
                *ctr = v;
                let total = self.successes as usize + self.failures as usize;
                Continue(Stout::new(self, total))
            } else {
                Halt
            }
        }
    }

    /// `(S, I) -> S`: `TakeIntoState`
    #[derive(Default)]
    pub struct Accumulator(usize);

    impl TakeIntoNext<usize> for Accumulator {
        type Next = State<Self>;

        fn take_into_next(self, input: usize) -> Self::Next {
            Accumulator(self.0 + input).into()
        }
    }

    /// `S -> S`: `IntoState`
    #[derive(Default)]
    pub struct Incrementor(usize);

    impl TakeIntoNext<()> for Incrementor {
        type Next = State<Self>;

        fn take_into_next(self, (): ()) -> Self::Next {
            Incrementor(self.0 + 1).into()
        }
    }

    /// `S -> (S, O)`: `IntoStout`
    #[derive(Default)]
    pub struct Naturals(usize);

    impl TakeIntoNext<()> for Naturals {
        type Next = Stout<Self, usize>;

        fn take_into_next(self, (): ()) -> Self::Next {
            let n = self.0 + 1;
            Stout::new(Self(n), n)
        }
    }

    /// `(S, I) -> (S, O)`: `TakeIntoStout`
    #[derive(Default)]
    pub struct OverwriteSlot<T>(Option<T>);

    impl<T> TakeIntoNext<T> for OverwriteSlot<T> {
        type Next = Stout<Self, Option<T>>;

        fn take_into_next(mut self, new: T) -> Self::Next {
            let prev = self.0.replace(new);
            Stout::new(self, prev)
        }
    }
}

mod consumers {
    mod is_into_halting_state {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::IntoHaltingState;

        #[test_case(Accumulator::default())]
        #[test_case(Countdown::default())]
        #[test_case(Incrementor::default())]
        #[test_case(Naturals::default())]
        #[test_case(NonExtended)]
        #[test_case(OverflowAccumulator::default())]
        #[test_case(OverflowingSuccessCounter::default())]
        #[test_case(OverwriteSlot::default())]
        #[test_case(UpToTen::default())]
        fn test<T: IntoHaltingState>(_: T) {}
    }

    mod is_into_halting_stout {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::IntoHaltingStout;

        #[test_case(Accumulator::default())]
        #[test_case(Countdown(10))]
        #[test_case(Countdown::default())]
        #[test_case(Incrementor::default())]
        #[test_case(Naturals::default())]
        #[test_case(NonExtended)]
        #[test_case(OverflowAccumulator::default())]
        #[test_case(OverflowingSuccessCounter::default())]
        #[test_case(OverwriteSlot::default())]
        #[test_case(UpToTen::default())]
        fn test<T: IntoHaltingStout<O>, O>(_: T) {}
    }

    mod is_take_into_halting_state {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::TakeIntoHaltingState;

        #[test_case(Accumulator::default(), 11)]
        #[test_case(Countdown::default(), ())]
        #[test_case(Incrementor::default(), ())]
        #[test_case(Naturals::default(), ())]
        #[test_case(NonExtended, ())]
        #[test_case(NonExtended, 2)]
        #[test_case(OverflowAccumulator::default(), 2)]
        #[test_case(OverflowingSuccessCounter::default(), false)]
        #[test_case(OverwriteSlot::default(), ())]
        #[test_case(UpToTen::default(), ())]
        fn test<T: TakeIntoHaltingState<I>, I>(_: T, _: I) {}
    }

    mod is_take_into_halting_stout {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::TakeIntoHaltingStout;

        #[test_case(Accumulator::default(), 23)]
        #[test_case(Countdown::default(), ())]
        #[test_case(Incrementor::default(), ())]
        #[test_case(Naturals::default(), ())]
        #[test_case(NonExtended, ())]
        #[test_case(NonExtended, 2)]
        #[test_case(OverflowAccumulator::default(), 41)]
        #[test_case(OverflowingSuccessCounter::default(), true)]
        #[test_case(OverwriteSlot::default(), ())]
        #[test_case(UpToTen::default(), ())]
        fn test<T: TakeIntoHaltingStout<I, O>, I, O>(_: T, _: I) {}
    }

    mod is_into_next {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::IntoNext;

        #[test_case(Accumulator::default())]
        #[test_case(Countdown::default())]
        #[test_case(Incrementor::default())]
        #[test_case(Naturals::default())]
        #[test_case(NonExtended)]
        #[test_case(OverflowAccumulator::default())]
        #[test_case(OverflowingSuccessCounter::default())]
        #[test_case(OverwriteSlot::default())]
        #[test_case(UpToTen::default())]
        fn test<T: IntoNext>(_: T) {}
    }

    mod is_into_state {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::IntoState;

        #[test_case(Accumulator::default())]
        #[test_case(Countdown::default())]
        #[test_case(Incrementor::default())]
        #[test_case(Naturals::default())]
        #[test_case(NonExtended)]
        #[test_case(OverflowAccumulator::default())]
        #[test_case(OverflowingSuccessCounter::default())]
        #[test_case(OverwriteSlot::default())]
        #[test_case(UpToTen::default())]
        fn test<T: IntoState>(_: T) {}
    }

    mod is_take_into_state {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::TakeIntoState;

        #[test_case(Accumulator::default(), 5)]
        #[test_case(Countdown::default(), ())]
        #[test_case(Incrementor::default(), ())]
        #[test_case(Naturals::default(), ())]
        #[test_case(NonExtended, ())]
        #[test_case(NonExtended, 2)]
        #[test_case(OverflowAccumulator::default(), 41)]
        #[test_case(OverflowingSuccessCounter::default(), false)]
        #[test_case(OverwriteSlot::default(), ())]
        #[test_case(UpToTen::default(), ())]
        fn test<T: TakeIntoState<I>, I>(_: T, _: I) {}
    }

    mod is_into_stout {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::IntoStout;

        #[test_case(Accumulator::default())]
        #[test_case(Countdown::default())]
        #[test_case(Incrementor::default())]
        #[test_case(Naturals::default())]
        #[test_case(NonExtended)]
        #[test_case(OverflowAccumulator::default())]
        #[test_case(OverflowingSuccessCounter::default())]
        #[test_case(OverwriteSlot::default())]
        #[test_case(UpToTen::default())]
        fn test<T: IntoStout<O>, O>(_: T) {}
    }

    mod is_take_into_stout {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::TakeIntoStout;

        #[test_case(Accumulator::default(), 11)]
        #[test_case(Countdown::default(), ())]
        #[test_case(Incrementor::default(), ())]
        #[test_case(Naturals::default(), ())]
        #[test_case(NonExtended, ())]
        #[test_case(NonExtended, 2)]
        #[test_case(OverflowAccumulator::default(), 41)]
        #[test_case(OverflowingSuccessCounter::default(), false)]
        #[test_case(OverwriteSlot::default(), "foo")]
        #[test_case(OverwriteSlot::default(), ())]
        #[test_case(UpToTen::default(), ())]
        fn test<T: TakeIntoStout<I, O>, I, O>(_: T, _: I) {}
    }

    mod is_take_into_next {
        use crate::tests::providers::*;
        use test_case::test_case;

        use crate::TakeIntoNext;

        #[test_case(Accumulator::default(), 23)]
        #[test_case(Countdown::default(), ())]
        #[test_case(Incrementor::default(), ())]
        #[test_case(Naturals::default(), ())]
        #[test_case(NonExtended, ())]
        #[test_case(NonExtended, 2)]
        #[test_case(OverflowAccumulator::default(), 41)]
        #[test_case(OverflowingSuccessCounter::default(), false)]
        #[test_case(OverwriteSlot::default(), ())]
        #[test_case(UpToTen::default(), ())]
        fn test<T: TakeIntoNext<I>, I>(_: T, _: I) {}
    }
}
