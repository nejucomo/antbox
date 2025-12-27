use rand::Rng;
use rand::distr::Distribution;

pub(crate) struct WeightedCoin(pub(crate) u32, pub(crate) u32);

impl Distribution<bool> for WeightedCoin {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        rng.random_ratio(self.0, self.1)
    }
}
