/*
    appellation: attributes <module>
    authors: @FL03
*/
//! this module implements the [`GraphProps`] trait and provides a concrete implementation
//! with the [`Attrs`] struct. These objects are used to define the _attributes_ of a
//! hypergraph such as the type of index used to identify vertices and edges as well as the
//! type of graph (directed or undirected).
#[doc(inline)]
pub use self::attributes::Attrs;

/// this module provides the physical [`Attrs`] implementation for hypergraphs.
pub mod attributes;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::attributes::Attrs;
    #[doc(inline)]
    pub use super::{DiAttrs, GraphProps, UnAttrs};
}
use crate::{Directed, GraphType, Mode, RawIndex, Undirected};

/// a type alias for graph [`Attrs`] configured with a [`Directed`] graph type.
pub type DiAttrs<Idx> = Attrs<Idx, Directed>;
/// a type alias for graph [`Attrs`] configured with an [`Undirected`] graph type.
pub type UnAttrs<Idx> = Attrs<Idx, Undirected>;

/// The [`GraphProps`] trait abstracts several generic types used to define a hyper graph
/// into a single entity.
pub trait GraphProps: 'static + Copy + Send + Sync {
    type Ix: RawIndex;
    type Kind: GraphType;

    private!();

    /// returns a new instance of the graph attributes.
    fn new() -> Self;
    /// returns a [`PhantomData`] instance of the graph attributes.
    fn phantom() -> PhantomData<(Self::Kind, Self::Ix)> {
        PhantomData::<(Self::Kind, Self::Ix)>
    }
    /// convert the current attributes into a [`PhantomData`] instance.
    fn into_phantom(self) -> PhantomData<(Self::Kind, Self::Ix)> {
        PhantomData::<(Self::Kind, Self::Ix)>
    }
    /// returns true if the attributes are directed.
    fn is_directed(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<Self::Kind>() == TypeId::of::<Directed>()
    }
    /// returns true if the attributes are undirected.
    fn is_undirected(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<Self::Kind>() == TypeId::of::<Undirected>()
    }
    /// returns the [`Mode`] of the graph, or `None` when the kind fixes no direction (a
    /// `Mode`-kinded graph stores no runtime mode)
    fn mode(&self) -> Option<Mode> {
        Mode::from_type::<Self::Kind>()
    }
}

/*
 ************* Implementations *************
*/
use core::marker::PhantomData;

impl<I, K> GraphProps for Attrs<I, K>
where
    I: RawIndex,
    K: GraphType,
{
    type Ix = I;
    type Kind = K;

    seal!();

    fn new() -> Self {
        Attrs::new()
    }
}

impl<I, K> GraphProps for PhantomData<(K, I)>
where
    I: RawIndex,
    K: GraphType,
{
    type Ix = I;
    type Kind = K;

    seal!();

    fn new() -> Self {
        PhantomData::<(K, I)>
    }
}

impl<I, K> GraphProps for (PhantomData<I>, PhantomData<K>)
where
    I: RawIndex,
    K: GraphType,
{
    type Ix = I;
    type Kind = K;

    seal!();

    fn new() -> Self {
        (PhantomData::<I>, PhantomData::<K>)
    }
}
