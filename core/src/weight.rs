/*
    appellation: weight <module>
    authors: @FL03
*/
//! this module implements a generic [`Weight`] wrapper type for representing the weights of
//! entries within the hypergraph. Additionally, the module provides the [`Weightless`] type
//! alias for cases where there is no associated weight.
#[doc(inline)]
pub use self::kinds::*;

mod impl_weight;
mod impl_weight_ops;
mod impl_weight_repr;
mod kinds;

#[doc(hidden)]
#[allow(deprecated)]
mod impl_weight_deprecated;

/// a trait for converting a type into a valid [`Weight`]
pub trait AsWeight<T> {
    fn as_weight(&self) -> Weight<T>;
}
/// a trait for converting a type into a valid [`Weight`]
pub trait IntoWeight<T> {
    fn into_weight(self) -> Weight<T>;
}

/// The [`Weight`] type is a wrapper around a generic type `T` that provides additional
/// functionality for working with weights in a graph context.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent, rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct Weight<T>(pub T);

/// An [`Weightless`] types is a type alias for a [`Weight`] that uses the
/// [`PhantomData`](core::marker::PhantomData) marker type to indicate that it has no weight.
pub type Weightless<T> = Weight<core::marker::PhantomData<T>>;

/*
 ************* Implementations *************
*/

impl<T> IntoWeight<T> for T {
    fn into_weight(self) -> Weight<T> {
        Weight::new(self)
    }
}

impl<T> AsWeight<T> for T
where
    T: Clone + IntoWeight<T>,
{
    fn as_weight(&self) -> Weight<T> {
        self.clone().into_weight()
    }
}
