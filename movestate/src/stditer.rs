//! [std::iter::Iterator] support

use std::marker::PhantomData;

use derive_new::new;
use moveslot::{MapInPlace as _, MoveSlot};

use crate::{Halting, IntoHaltingStout, Stout, TakeIntoNext};

#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct IHSIter<S, O> {
    #[new(into)]
    mslot: MoveSlot<S>,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<S, O> Iterator for IHSIter<S, O>
where
    S: IntoHaltingStout<O>,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        self.mslot.mip_out_opt(IntoHaltingStout::into_opt_self_out)
    }
}

/// Blanket impl such that every [Iterator] is an [IntoHaltingStout](crate::IntoHaltingStout)
impl<S> TakeIntoNext<()> for S
where
    S: Iterator,
{
    type Next = Halting<Stout<S, S::Item>>;

    fn take_into_next(mut self, (): ()) -> Self::Next {
        self.next().map(|x| (self, x)).into()
    }
}

#[test]
fn test_iterator_impl() {
    use crate::IntoHaltingStout;

    let s0 = 0..3;

    let (s1, n0) = s0.into_opt_self_out().unwrap();
    assert_eq!(n0, 0);
    let (s2, n1) = s1.into_opt_self_out().unwrap();
    assert_eq!(n1, 1);
    let (s3, n2) = s2.into_opt_self_out().unwrap();
    assert_eq!(n2, 2);

    assert!(s3.into_opt_self_out().is_none());
}

#[test]
fn test_into_iterator() {
    struct SliceIHS(&'static [usize]);

    impl TakeIntoNext<()> for SliceIHS {
        type Next = Halting<Stout<Self, usize>>;

        fn take_into_next(self, (): ()) -> Self::Next {
            self.0.split_first().map(|(&x, s)| (SliceIHS(s), x)).into()
        }
    }

    let sum: usize = SliceIHS(&[2, 3, 5]).into_iterator().sum();
    assert_eq!(sum, 10);
}
