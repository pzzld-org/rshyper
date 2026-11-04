/*
    appellation: attrs <module>
    authors: @FL03
*/
use crate::idx::RawIndex;
use crate::{Directed, GraphType, Mode, Undirected};
use core::marker::PhantomData;

/// [`Attrs`] is a generic implementation of the [`GraphProps`](super::GraphProps) trait,
/// enabling the definition of hypergraphs with different index types and graph kinds
/// (i.e., [`Directed`] or [`Undirected`]).
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct Attrs<I = usize, K = Undirected> {
    /// the inner type of index used by the graph
    pub(crate) index: PhantomData<I>,
    /// the kind of graph, either directed or undirected
    pub(crate) kind: PhantomData<K>,
}

impl<I, K> Attrs<I, K> {
    /// returns a new instance of [`Attrs`] initialized with the given index and kind.
    pub const fn new() -> Self
    where
        I: RawIndex,
        K: GraphType,
    {
        Attrs {
            index: PhantomData::<I>,
            kind: PhantomData::<K>,
        }
    }
    /// consumes the current instance to create another with the given kind
    pub fn with_kind<K2>(self) -> Attrs<I, K2>
    where
        K2: GraphType,
    {
        Attrs {
            index: self.index,
            kind: PhantomData::<K2>,
        }
    }
    /// consumes the current instance to create another with the given index type
    pub fn with_index<I2>(self) -> Attrs<I2, K>
    where
        I2: RawIndex,
    {
        Attrs {
            index: PhantomData::<I2>,
            kind: self.kind,
        }
    }
    /// returns true if the current kind `K` is the same as the given kind `K2`
    pub fn is_kind<K2: 'static>(&self) -> bool
    where
        K: GraphType,
    {
        use core::any::TypeId;
        TypeId::of::<K2>() == TypeId::of::<K>()
    }
    /// returns true if the current index type `I` is the same as the given index type `I2`
    pub fn is_index<I2: 'static>(&self) -> bool
    where
        I: RawIndex,
    {
        use core::any::TypeId;
        TypeId::of::<I2>() == TypeId::of::<I>()
    }
    /// returns true if the current kind is [`Directed`]
    pub fn is_directed(&self) -> bool
    where
        K: GraphType,
    {
        self.is_kind::<Directed>()
    }
    /// returns true if the current kind is [`Undirected`]
    pub fn is_undirected(&self) -> bool
    where
        K: GraphType,
    {
        self.is_kind::<Undirected>()
    }
    /// returns the _kind_ of the graph as an enum variant of [`Mode`], or `None` when `K`
    /// fixes no direction (a `Mode`-kinded graph stores no runtime mode)
    pub fn mode(&self) -> Option<Mode>
    where
        K: GraphType,
    {
        Mode::from_type::<K>()
    }
}

impl<I> Attrs<I, Directed>
where
    I: RawIndex,
{
    /// initializes a new instance of [`Attrs`] with the kind set to [`Directed`].
    pub const fn directed() -> Self {
        Attrs {
            index: PhantomData::<I>,
            kind: PhantomData::<Directed>,
        }
    }
}

impl<I> Attrs<I, Undirected>
where
    I: RawIndex,
{
    /// initializes a new instance of [`Attrs`] with the kind set to [`Undirected`].
    pub const fn undirected() -> Self {
        Attrs {
            index: PhantomData::<I>,
            kind: PhantomData::<Undirected>,
        }
    }
}

impl<I, K> Clone for Attrs<I, K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I, K> Copy for Attrs<I, K> {}

impl<I, K> Default for Attrs<I, K>
where
    I: RawIndex,
    K: GraphType,
{
    fn default() -> Self {
        Attrs::new()
    }
}

unsafe impl<I, K> Send for Attrs<I, K> {}

unsafe impl<I, K> Sync for Attrs<I, K> {}

impl<I, K> core::fmt::Debug for Attrs<I, K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Attrs")
            .field(&self.index)
            .field(&self.kind)
            .finish()
    }
}

impl<I, K> core::fmt::Display for Attrs<I, K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "({}, {})",
            core::any::type_name::<I>(),
            core::any::type_name::<K>()
        )
    }
}
