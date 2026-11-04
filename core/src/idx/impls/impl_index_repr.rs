/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::idx::{EdgeIndex, IndexBase, IndexType, RawIndex, VertexIndex};
use core::sync::atomic::{AtomicUsize, Ordering};

/// a global static counter used to generate unique indices
pub(crate) static INDEX_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl<K> IndexBase<usize, K>
where
    K: IndexType,
{
    /// returns a new index generated using an [`AtomicUsize`]
    /// This method is useful in that it is `no_std` compatible, thread-safe, and capable of
    /// generating unique indices in a concurrent environment.
    pub fn atomic() -> Self {
        Self::new(INDEX_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    /// a helper method that dynamically initializes the index based on the enabled features:
    ///
    /// - `rand`: if enabled, use random number generation to generate the index.
    /// - `fallback`: otherwise, use an atomic counter to generate the index.
    pub fn create() -> Self {
        #[cfg(feature = "rand")]
        {
            Self::rand()
        }
        #[cfg(not(feature = "rand"))]
        {
            Self::atomic()
        }
    }
    /// atomically generates a the next index, replacing the current value with the generated
    /// one and returning the previous value.
    pub fn atomic_next(&mut self) -> Self {
        // generate the next id atomically
        let mut id = Self::atomic();
        // swap values, replacing the current instances value with the new one
        self.swap(&mut id);
        // return the previous instance
        id
    }
    /// the [`generate`](IndexBase::generate) is useful for instances where the type `Idx` of
    /// the [`IndexBase`] is not generalized, automatically using random number generation
    /// if the `rand` feature is enabled, or an atomic counter otherwise. The method is also
    /// useful in that it generates [`usize`] indices, whcih are the most common instances, yet
    /// aren't direct implementors of [`StandardUniform`](https://docs.rs/rand/latest/rand/distr/struct.StandardUniform.html).
    pub fn generate(&mut self) -> Self {
        // declare the prev param
        let prev: usize;
        // use random number generation if the `rand` feature is enabled
        #[cfg(feature = "rand")]
        {
            // generate a random value from the standard uniform distribution
            let next = rand::random::<u128>();
            // replace the current value with the next one
            prev = self.replace(next as usize);
        }
        // otherwise use the atomic counter
        #[cfg(not(feature = "rand"))]
        {
            prev = self.replace(INDEX_COUNTER.fetch_add(1, Ordering::Relaxed));
        }
        // return a new instance with the previous value
        Self::new(prev)
    }
}

impl<T> IndexBase<T, EdgeIndex>
where
    T: RawIndex,
{
    /// initializes a new index for an edge
    pub const fn edge(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> IndexBase<T, VertexIndex>
where
    T: RawIndex,
{
    /// initializes a new index for a vertex
    pub const fn vertex(value: T) -> Self {
        Self::new(value)
    }
}
