/*
    Appellation: index <module>
    Contrib: @FL03
*/

/// A generic [`IndexBase`] implementation used to represent various [_kinds_](super::IndexType) of
/// indices
#[derive(Clone, Copy, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IndexBase<Idx = super::Udx, K = super::VertexIndex> {
    pub(crate) value: Idx,
    pub(crate) _type: core::marker::PhantomData<K>,
}
