/*
    appellation: contains <module>
    authors: @FL03
*/
/// [`Contains`] defines a common interface for types able to verify if they contain a given
/// key or index; the trait strives to emulate the behavior of the `contains` method found in
/// standard collections such as `HashSet` or `BTreeSet`.
pub trait Contains<Q> {
    type Key;
    /// checks if the container contains the given index
    fn contains(&self, key: &Q) -> bool
    where
        Self::Key: core::borrow::Borrow<Q>;
}

/*
 ************* Implementations *************
*/
use crate::idx::{RawIndex, VertexId};
use crate::{Domain, GraphType};
use crate::{HyperEdge, Link};

impl<S, K, Idx, Q> Contains<Q> for Link<S, K, Idx>
where
    Q: PartialEq,
    S: Domain<Idx>,
    K: GraphType,
    Idx: RawIndex + PartialEq,
    for<'a> &'a S: IntoIterator<Item = &'a VertexId<Idx>>,
{
    type Key = VertexId<Idx>;

    fn contains(&self, query: &Q) -> bool
    where
        Self::Key: core::borrow::Borrow<Q>,
    {
        self.contains(query)
    }
}

impl<T, S, K, Idx, Q> Contains<Q> for HyperEdge<T, S, K, Idx>
where
    Q: PartialEq,
    S: Domain<Idx>,
    K: GraphType,
    Idx: RawIndex + PartialEq,
    for<'a> &'a S: IntoIterator<Item = &'a VertexId<Idx>>,
{
    type Key = VertexId<Idx>;

    fn contains(&self, query: &Q) -> bool
    where
        Self::Key: core::borrow::Borrow<Q>,
    {
        self.contains(query)
    }
}
