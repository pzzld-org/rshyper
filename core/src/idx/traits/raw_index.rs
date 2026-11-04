/*
    appellation: index <module>
    authors: @FL03
*/

/// a simple trait for denoting types compatible with to be used as indices in a hypergraph.
/// **note:** the trait is sealed to prevent external implementations.
pub trait RawIndex: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private! {}
    /// returns the value one step after `self`, or [`None`] when that value would overflow
    /// (or, for floats, would not be a new finite value). Id allocation uses it so an
    /// exhausted index space is an error rather than a panic or a wrap-around that reissues
    /// old ids.
    #[doc(hidden)]
    fn checked_step(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

pub trait Idx: RawIndex
where
    Self: Clone + Copy + Eq + PartialOrd + core::hash::Hash,
{
    type Kind: ?Sized;

    /// converts the index to a string representation.
    #[cfg(feature = "alloc")]
    fn to_string(&self) -> alloc::string::String {
        alloc::format!("{self}")
    }
}
/// The [`HashIndex`] trait extends the [`RawIndex`] trait to include additional operations and
/// behaviours commonly expected from indices in a hypergraph.
///
/// **note:** this trait is automatically implemented for all [`RawIndex`] implementors that
/// also implements the [`Eq`] and [`Hash`](core::hash::Hash) traits and is sealed to prevent
/// any external implementations.
pub trait HashIndex: RawIndex
where
    Self: Eq + core::hash::Hash,
{
    private!();
}
/// The [`NumIndex`] trait extends the [`RawIndex`] trait to include additional operations and
/// behaviours expected from numerical indices in a hypergraph.
///
/// **note:** the trait is automatically implemented for all types that implement [`HashIndex`]
/// alongside additional traits
pub trait NumIndex: RawIndex
where
    Self: Copy
        + PartialEq
        + PartialOrd
        + crate::AddStep<Output = Self>
        + core::ops::Add<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Shl<Output = Self>
        + core::ops::Shr<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::ShlAssign
        + core::ops::ShrAssign
        + core::ops::SubAssign
        + core::ops::Not
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToBytes
        + num_traits::Num
        + num_traits::NumRef
        + num_traits::NumAssignRef,
{
    private!();
}

/// The [`HyperIndex`] trait extends the [`NumIndex`] to define contraints for the standard
/// index type for the crate; implementors must also implement following traits:
///
/// - [`NumIndex`]
/// - [`Default`]
/// - [`Eq`]
/// - [`Hash`](core::hash::Hash)
/// - [`Ord`]
pub trait HyperIndex: NumIndex
where
    Self: Default + Eq + Ord + core::hash::Hash,
{
    private!();
}
/*
 ************* Implementations *************
*/

impl<T> HashIndex for T
where
    T: RawIndex + Eq + core::hash::Hash,
{
    seal!();
}

impl<T> NumIndex for T
where
    T: RawIndex
        + Copy
        + Default
        + PartialEq
        + PartialOrd
        + crate::AddStep<Output = Self>
        + core::ops::Add<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Shl<Output = Self>
        + core::ops::Shr<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::ShlAssign
        + core::ops::ShrAssign
        + core::ops::SubAssign
        + core::ops::Not
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToBytes
        + num_traits::NumRef
        + num_traits::NumAssignRef,
{
    seal!();
}

impl<T> HyperIndex for T
where
    T: NumIndex + Default + Eq + Ord + core::hash::Hash,
{
    seal!();
}

macro_rules! impl_raw_index {
    ($($T:ty),* $(,)?) => {
        $(
            impl_raw_index!(@impl RawIndex for $T);
        )*
    };
    (@impl $trait:ident for $t:ty) => {
        impl $crate::idx::$trait for $t {
            seal! {}
        }
    };
    (@int $($t:ty),* $(,)?) => {
        $(
            impl $crate::idx::RawIndex for $t {
                seal! {}

                fn checked_step(&self) -> Option<Self> {
                    self.checked_add(1)
                }
            }
        )*
    };
    (@float $($t:ty),* $(,)?) => {
        $(
            impl $crate::idx::RawIndex for $t {
                seal! {}

                fn checked_step(&self) -> Option<Self> {
                    let next = *self + 1.0;
                    // past 2^mantissa `x + 1.0 == x`, which would reissue the same id
                    (next.is_finite() && next > *self).then_some(next)
                }
            }
        )*
    };
}

impl_raw_index! {
    @int usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
}

impl_raw_index! {
    @float f32, f64
}

impl_raw_index! {
    char, str
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::RawIndex;
    use alloc::boxed::Box;

    impl_raw_index! {
        alloc::string::String,
    }

    impl RawIndex for Box<dyn RawIndex> {
        seal! {}
    }

    impl RawIndex for Box<dyn RawIndex + Send + Sync + 'static> {
        seal! {}
    }
}
