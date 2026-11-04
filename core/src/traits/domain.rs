/*
    appellation: domain <module>
    authors: @FL03
*/
use crate::idx::{RawIndex, VertexId};

/// [`RawDomain`] is a trait that defines the behavior of a store that holds the vertices
/// associated with a hyperedge or hyperfacet. It is used to abstract over different
/// implementations of edge storage, such as arrays, vectors, or sets.
///
/// **note:** The trait is sealed to prevent external implementations, ensuring that only the
/// crate can define how edges are stored. This is to maintain consistency and prevent
/// misuse of the trait in different contexts.
pub trait RawDomain {
    type Key;

    private!();
    /// returns the number of vertices associated with the edge.
    fn len(&self) -> usize;
    /// returns true if there are no points.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// An [`Domain`] is a trait is a specialization of the [`RawDomain`] trait that represents
/// a store for edges, which are collections of vertices. It is used to define the behavior
pub trait Domain<Idx = usize>: RawDomain<Key = VertexId<Idx>>
where
    Idx: RawIndex,
{
}
/// The [`BinaryDomain`] trait extends the [`RawDomain`] trait to provide specific methods for
/// binary edges, which are edges that connect exactly two vertices.
pub trait BinaryDomain<Idx>: RawDomain<Key = VertexId<Idx>>
where
    Idx: RawIndex,
{
    /// returns the left-hand side vertex of the edge.
    fn src(&self) -> &Self::Key;
    /// returns the right-hand side vertex of the edge.
    fn tgt(&self) -> &Self::Key;
}
/// The [`IterDomain`] trait defines the base interface for creating an interator over a domain
/// of vertices.
pub trait IterDomain<Idx = usize>
where
    Idx: RawIndex,
{
    type Store<_T>: RawDomain<Key = VertexId<Idx>>;
    type Iter<'a, _T>: Iterator<Item = &'a _T>
    where
        _T: 'a,
        Self: 'a;
    /// returns an iterator over the vertices in the store.
    fn iter(&self) -> Self::Iter<'_, <Self::Store<Idx> as RawDomain>::Key>;
}

/*
 ************* Implementations *************
*/
impl<S, Idx> Domain<Idx> for S
where
    Idx: RawIndex,
    S: RawDomain<Key = VertexId<Idx>>,
{
}

macro_rules! impl_domain {
    (@impl $trait:ident for $t:ident<$T:ident>) => {
        impl<$T> $crate::$trait for $t<$crate::VertexId<$T>>
        where
            $T: $crate::idx::RawIndex,
        {
            type Key = $crate::VertexId<$T>;

            seal!();

            fn len(&self) -> usize {
                <$t<$crate::VertexId<$T>>>::len(self)
            }

            fn is_empty(&self) -> bool {
                <$t<$crate::VertexId<$T>>>::is_empty(self)
            }
        }
    };
    ($($t:ident<$T:ident>),* $(,)?) => {
        $(
            impl_domain!(@impl RawDomain for $t<$T>);
        )*
    };
}

#[cfg(feature = "hashbrown")]
mod impl_hb {
    use super::RawDomain;
    use crate::idx::{RawIndex, VertexId};
    use core::hash::BuildHasher;
    use hashbrown::{HashMap, HashSet};

    impl<Ix, V, S> RawDomain for HashMap<VertexId<Ix>, V, S>
    where
        Ix: RawIndex,
        S: BuildHasher,
    {
        type Key = VertexId<Ix>;

        seal!();

        fn len(&self) -> usize {
            self.len()
        }

        fn is_empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<I, S> RawDomain for HashSet<VertexId<I>, S>
    where
        I: RawIndex,
        S: BuildHasher,
    {
        type Key = VertexId<I>;

        seal!();

        fn len(&self) -> usize {
            self.len()
        }

        fn is_empty(&self) -> bool {
            self.is_empty()
        }
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use alloc::collections::{BTreeSet, VecDeque};
    use alloc::vec::Vec;

    impl_domain! {
        BTreeSet<I>,
        Vec<I>,
        VecDeque<I>
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::RawDomain;
    use crate::idx::{RawIndex, VertexId};
    use core::hash::BuildHasher;
    use std::collections::hash_set::HashSet;

    impl<I, S> RawDomain for HashSet<VertexId<I>, S>
    where
        I: RawIndex,
        S: BuildHasher,
    {
        type Key = VertexId<I>;

        seal!();

        fn len(&self) -> usize {
            <HashSet<VertexId<I>, S>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <HashSet<VertexId<I>, S>>::is_empty(self)
        }
    }
}

impl<I> RawDomain for &[VertexId<I>]
where
    I: RawIndex,
{
    type Key = VertexId<I>;

    seal!();

    fn len(&self) -> usize {
        <[VertexId<I>]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[VertexId<I>]>::is_empty(self)
    }
}

impl<I> RawDomain for [VertexId<I>]
where
    I: RawIndex,
{
    type Key = VertexId<I>;

    seal!();

    fn len(&self) -> usize {
        <[VertexId<I>]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[VertexId<I>]>::is_empty(self)
    }
}

impl<const N: usize, I> RawDomain for [VertexId<I>; N]
where
    I: RawIndex,
{
    type Key = VertexId<I>;

    seal!();

    fn len(&self) -> usize {
        <[Self::Key]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[Self::Key]>::is_empty(self)
    }
}

impl<I> BinaryDomain<I> for [VertexId<I>; 2]
where
    I: RawIndex,
{
    fn src(&self) -> &VertexId<I> {
        &self[0]
    }

    fn tgt(&self) -> &VertexId<I> {
        &self[1]
    }
}

impl<I> RawDomain for (VertexId<I>, VertexId<I>)
where
    I: RawIndex,
{
    type Key = VertexId<I>;

    seal!();

    fn len(&self) -> usize {
        2
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl<I> BinaryDomain<I> for (VertexId<I>, VertexId<I>)
where
    I: RawIndex,
{
    fn src(&self) -> &VertexId<I> {
        &self.0
    }

    fn tgt(&self) -> &VertexId<I> {
        &self.1
    }
}
