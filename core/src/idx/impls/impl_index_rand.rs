/*
    Appellation: impl_index_rand <module>
    Created At: 2026.01.10:11:21:50
    Contrib: @FL03
*/
#![cfg(feature = "rand")]
use crate::idx::{IndexBase, IndexType, RawIndex};
use rand::RngCore;
use rand_distr::uniform::{SampleRange, SampleUniform};
use rand_distr::{Distribution, StandardNormal, StandardUniform};

/// generic implementations for the [`IndexBase<T, K>`] enabled by the `rand` feature
impl<K> IndexBase<usize, K>
where
    K: IndexType,
{
    /// generate a random index from a value of type `T`
    pub fn rand() -> Self {
        let rid = rand::random::<u128>();
        Self::new(rid as usize)
    }
}

/// generic implementations for the [`IndexBase<T, K>`] enabled by the `rand` feature
impl<T, K> IndexBase<T, K>
where
    T: RawIndex,
    K: IndexType,
{
    /// replaces the current value with a randomly generated value and returns a new instance
    /// of [`IndexBase`] with the previous value.
    pub fn rand_step(&mut self) -> Self
    where
        StandardUniform: Distribution<T>,
    {
        // generate a random value from the standard uniform distribution
        let mut id = Self::random();
        // swap out the values
        self.swap(&mut id);
        // return the previous value
        id
    }
    /// generate a random index from a value of type `T`
    pub fn random() -> Self
    where
        StandardUniform: Distribution<T>,
    {
        Self::new(rand::random())
    }
    /// returns a new index randomly generated within the provided range
    pub fn random_between<R>(range: R) -> Self
    where
        R: SampleRange<T>,
        T: SampleUniform,
    {
        // generate a random value from the standard uniform distribution within the range
        let value = rand::random_range(range);
        // initialize a new `IndexBase` with the generated value
        IndexBase::new(value)
    }
    /// generate a random index from a value of type `T` using the provided [`Rng`](rand::Rng)
    pub fn random_with<R, Dist>(rng: &mut R, distr: Dist) -> Self
    where
        R: ?Sized + RngCore,
        Dist: Distribution<T>,
    {
        use rand::Rng;
        // generate a random u128 and cast it to usize
        let rid = rng.sample(distr);
        // cast the random u128 to usize
        Self::new(rid)
    }
}

impl<T, K> Distribution<IndexBase<T, K>> for StandardUniform
where
    K: IndexType,
    T: RawIndex,
    StandardUniform: Distribution<T>,
{
    fn sample<R>(&self, rng: &mut R) -> IndexBase<T, K>
    where
        R: ?Sized + rand::Rng,
    {
        // generate a random value from the standard normal distribution
        let value = self.sample(rng);
        // initialize a new `IndexBase` with the generated value
        IndexBase::new(value)
    }
}

impl<T, K> Distribution<IndexBase<T, K>> for StandardNormal
where
    K: IndexType,
    T: RawIndex,
    StandardNormal: Distribution<T>,
{
    fn sample<R>(&self, rng: &mut R) -> IndexBase<T, K>
    where
        R: ?Sized + rand::RngCore,
    {
        // generate a random value from the standard normal distribution
        let value = self.sample(rng);
        // initialize a new `IndexBase` with the generated value
        IndexBase::new(value)
    }
}
