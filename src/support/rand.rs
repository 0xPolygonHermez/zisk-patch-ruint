//! Support for the [`rand`](https://crates.io/crates/rand) crate.

#![cfg(feature = "rand")]
#![cfg_attr(docsrs, doc(cfg(feature = "rand")))]

// FEATURE: Implement the Uniform distribution.

use crate::Uint;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl<const BITS: usize, const LIMBS: usize> Distribution<Uint<BITS, LIMBS>> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Uint<BITS, LIMBS> {
        <Uint<BITS, LIMBS>>::random_with(rng)
    }
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Creates a new [`Uint`] with the default cryptographic random number
    /// generator.
    ///
    /// This is currently [`rand::thread_rng`].
    #[inline]
    #[must_use]
    #[cfg(feature = "std")]
    pub fn random() -> Self {
        // SAFETY: `uint` is only accessible after random initialization.
        #[allow(clippy::uninit_assumed_init)]
        let mut uint = unsafe { core::mem::MaybeUninit::<Self>::uninit().assume_init() };
        uint.randomize();
        uint
    }

    /// Creates a new [`Uint`] with the given random number generator.
    #[inline]
    #[doc(alias = "random_using")]
    #[must_use]
    pub fn random_with<R: rand::RngCore + ?Sized>(rng: &mut R) -> Self {
        // SAFETY: `uint` is only accessible after random initialization.
        #[allow(clippy::uninit_assumed_init)]
        let mut uint = unsafe { core::mem::MaybeUninit::<Self>::uninit().assume_init() };
        uint.randomize_with(rng);
        uint
    }

    /// Fills this [`Uint`] with the default cryptographic random number
    /// generator.
    ///
    /// See [`random`](Self::random) for more details.
    #[inline]
    #[cfg(feature = "std")]
    pub fn randomize(&mut self) {
        self.randomize_with(&mut rand::thread_rng());
    }

    /// Fills this [`Uint`] with the given random number generator.
    #[inline]
    #[doc(alias = "randomize_using")]
    pub fn randomize_with<R: rand::RngCore + ?Sized>(&mut self, rng: &mut R) {
        rng.fill(&mut self.limbs[..]);
        self.apply_mask();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};

    #[test]
    fn test_rand() {
        let mut rng = rand::thread_rng();
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            for _ in 0..1000 {
                let _: Uint<BITS, LIMBS> = rng.gen();
            }
        });
    }
}
