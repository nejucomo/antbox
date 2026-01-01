use crate::NNF;

/// Raise to nonnegative
pub trait PowUnsigned {
    /// Raise to the `pow` [NNF] power
    fn pow_nnf(self, pow: NNF) -> Self;

    /// Raise to the `pow` [u32] power
    fn pow_u32(self, pow: u32) -> Self;
}
