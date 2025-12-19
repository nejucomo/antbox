use rand::seq::SliceRandom as _;

pub(crate) trait ShuffleIntoVec: Iterator {
    fn shuffle_into_vec<R>(self, rng: &mut R) -> Vec<Self::Item>
    where
        R: rand::Rng;
}

impl<I> ShuffleIntoVec for I
where
    I: Iterator,
{
    fn shuffle_into_vec<R>(self, rng: &mut R) -> Vec<Self::Item>
    where
        R: rand::Rng,
    {
        let mut v: Vec<_> = self.collect();
        v.shuffle(rng);
        v
    }
}
