/*
    Appellation: index <module>
    Contrib: @FL03
*/

/// A generic [`IndexBase`] implementation used to represent various [_kinds_](GraphIndex) of
/// indices
#[derive(Clone, Copy, Hash, Ord, PartialOrd)]
// ids serialize as their raw value, so they work as JSON map keys (#126)
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct IndexBase<Idx = super::Udx, K = super::VertexIndex> {
    pub(crate) value: Idx,
    pub(crate) _type: core::marker::PhantomData<K>,
}
