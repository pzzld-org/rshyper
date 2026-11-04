/*
    appellation: impl_weight_repr <module>
    authors: @FL03
*/
use crate::weight::{Weight, Weightless};

impl<T> Weightless<T> {
    /// initializes a new instance of the [`Weight`] containing a [`PhantomData`](core::marker::PhantomData) marker.
    pub const fn weightless() -> Self {
        Weight::new(core::marker::PhantomData::<T>)
    }
    /// returns a new [`Weight`] of type `T` initialized using the provided closure `f`
    pub fn init_with<F>(self, f: F) -> Weight<T>
    where
        F: FnOnce() -> T,
    {
        Weight::new(f())
    }
    /// returns a new [`Weight`] initialized using the logical default for type `T`
    pub fn init_default(self) -> Weight<T>
    where
        T: Default,
    {
        self.init_with(Default::default)
    }
}

impl<T> Weight<&T> {
    /// returns a new [`Weight`] using a clone of the current inner value.
    #[inline]
    pub fn cloned(&self) -> Weight<T>
    where
        T: Clone,
    {
        Weight::new(self.0.clone())
    }
    /// returns a new instance of the [`Weight`] using a copy of the current inner value.
    #[inline]
    pub fn copied(&self) -> Weight<T>
    where
        T: Copy,
    {
        Weight::new(*self.0)
    }
}

impl<T> Weight<&mut T> {
    /// returns a new [`Weight`] using a clone of the current inner value.
    #[inline]
    pub fn cloned(&self) -> Weight<T>
    where
        T: Clone,
    {
        Weight::new(self.0.clone())
    }
    /// returns a new instance of the [`Weight`] using a copy of the current inner value.
    #[inline]
    pub const fn copied(&self) -> Weight<T>
    where
        T: Copy,
    {
        Weight::new(*self.0)
    }
}

impl<T> Weight<Option<T>> {
    /// returns a new [`Weight`] with [`Some`] inner value of type `T`
    pub const fn some(value: T) -> Self {
        Weight(Some(value))
    }
    /// returns a new [`Weight`] with [`None`] inner value.
    pub const fn none() -> Self {
        Weight(None)
    }
    /// returns true if the inner value is `Some`, false otherwise.
    pub fn is_some(&self) -> bool {
        self.get().is_some()
    }
    /// returns true if the inner value is `None`, false otherwise.
    pub fn is_none(&self) -> bool {
        self.get().is_none()
    }
}
