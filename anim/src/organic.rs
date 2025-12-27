//! Utilities to make rendering look more "wild" and "organic" using [Rng]

use derive_new::new;
use rand::Rng;
use rand_distr::Distribution;

#[derive(Copy, Clone, Debug, new)]
pub(crate) struct OrganicScale {
    lo: f32,
    hi: f32,
    n: usize,
}

impl Default for OrganicScale {
    fn default() -> Self {
        Self {
            lo: 0.8,
            hi: 1.1,
            n: 2,
        }
    }
}

impl Distribution<f32> for OrganicScale {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        // This is probably a known, existing, more efficiently implemented distribution:
        let mut lo = self.lo;
        let mut hi = self.hi;

        for _ in 0..self.n {
            lo = rng.random_range(lo..1.0);
            hi = rng.random_range(1.0..hi);
        }

        rng.random_range(lo..=hi)
    }
}
