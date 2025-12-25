//! [std::iter::Iterator] support

use crate::{Halting, Stout, TakeIntoNext};

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
