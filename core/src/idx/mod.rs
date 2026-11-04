/*
    appellation: index <module>
    authors: @FL03
*/
//! the [`idx`](crate::idx) module provides the [`IndexBase`], a generic index type used to
//! establish a solid foundation for all indices used by the hypergraph. Type aliases, such as
//! [`EdgeId`] and [`VertexId`], are provided for convenience, reducing the need to continually
//! specify the index type when working with hypergraphs.
#[doc(inline)]
#[cfg(feature = "alloc")]
pub use self::tracker::IndexTracker;
#[doc(inline)]
pub use self::{error::*, frame::*, index::*, iter::*, traits::*, types::*};

/// this module defines the [`IndexError`] type, establishing the various errors encountered by
/// indices in a hypergraph.
pub mod error;
/// this module implements the [`IndexFrame`] type, which is used to track the current edge
/// and vertex indices in a hypergraph.
pub mod frame;
/// this module provides the [`IndexBase`] type, which is a generic index type used to
/// represent various kinds of indices in a hypergraph.
mod index;
#[cfg(feature = "alloc")]
/// this module provides the [`IndexTracker`] for retaining a history of created indices
pub mod tracker;

#[doc(hidden)]
mod impls {
    mod impl_index;
    mod impl_index_ext;
    mod impl_index_ops;
    mod impl_index_rand;
    mod impl_index_repr;
}

pub mod iter {
    //! this module provides various iterators for indices, such as [`Counter`] and
    //! [`Stepper`].
    #[doc(inline)]
    pub use self::prelude::*;

    mod counter;
    mod stepper;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::counter::*;
        #[doc(inline)]
        pub use super::stepper::*;
    }
}

mod traits {
    //! this module defines the [`RawIndex`] trait along with its related traits and
    //! implementations.
    #[doc(inline)]
    pub use self::prelude::*;

    /// this module defines various conversion routines for converting types into valid indices
    mod convert;
    /// this module provides the [`Indexed`] trait for defining various representations of a
    /// type that has knowledge of its index.
    mod graph_index;
    /// this module provides the [`RawIndex`] trait
    mod raw_index;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::graph_index::*;
        #[doc(inline)]
        pub use super::raw_index::*;
    }
}

mod types {
    //! this module provides various types in support of the [`IndexBase`](super::IndexBase)
    //! type
    //!
    #[doc(inline)]
    pub use self::prelude::*;

    mod aliases;
    mod kinds;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::kinds::*;
    }
}

pub(crate) mod prelude {
    pub use super::frame::*;
    pub use super::index::IndexBase;
    pub use super::traits::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() -> crate::Result<()> {
        let mut idx = IndexBase::<usize, VertexIndex>::new(1);
        assert_eq!(idx.get(), &1);
        idx.set(2);
        assert_eq!(idx.get(), &2);
        Ok(())
    }

    #[test]
    fn test_edge_id() -> crate::Result<()> {
        let mut edge_id = EdgeId::<usize>::default();
        let e0 = edge_id.step()?;
        let e1 = edge_id.step()?;
        let e2 = edge_id.step()?;
        assert_eq!(e0.get(), &0);
        assert_eq!(e1.get(), &1);
        assert_eq!(e2.get(), &2);
        Ok(())
    }

    #[test]
    fn test_vertex_id() -> crate::Result<()> {
        let vertex_id = VertexId::new(1);
        assert_eq!(vertex_id.get(), &1);
        Ok(())
    }

    #[test]
    fn test_position() -> crate::Result<()> {
        let mut index = IndexFrame::<usize>::zero();
        // create some edge indices
        let e0 = index.next_edge()?;
        let e1 = index.next_edge()?;
        let e2 = index.next_edge()?;
        // check the edge indices
        assert_eq!(e0, &0);
        assert_eq!(e1, &1);
        assert_eq!(e2, &2);
        // create some vertex indices
        let v0 = index.next_node()?;
        let v1 = index.next_node()?;
        let v2 = index.next_node()?;
        // check the vertex indices
        assert_eq!(e0.get(), v0.get());
        assert_eq!(e1.get(), v1.get());
        assert_eq!(e2.get(), v2.get());
        Ok(())
    }

    #[test]
    fn test_tracker() -> crate::Result<()> {
        let mut history = IndexTracker::<usize>::zero();
        // create some edge indices
        let e0 = history.next_edge()?;
        let e1 = history.next_edge()?;
        let e2 = history.next_edge()?;
        // veryify the edge indices
        assert_eq!(history.edges(), &[e0, e1, e2]);
        // create some vertex indices
        let v0 = history.next_vertex()?;
        let v1 = history.next_vertex()?;
        let v2 = history.next_vertex()?;
        // verify the vertex indices
        assert_eq!(history.nodes(), &[v0, v1, v2]);
        Ok(())
    }

    #[test]
    fn test_tracker_sees_raw_history_edits() {
        let mut history = IndexTracker::<usize>::zero();
        history.next_vertex().unwrap();
        history.next_vertex().unwrap();
        // an id pushed past the cursor through the raw accessor must still be rejected
        history.nodes_mut().push(VertexId::new(4));
        assert_eq!(history.next_vertex().unwrap(), 2);
        assert_eq!(history.next_vertex().unwrap(), 3);
        let before = history.clone();
        assert!(matches!(
            history.next_vertex(),
            Err(IndexError::DuplicateIndex)
        ));
        assert_eq!(history.nodes(), before.nodes());
        assert_eq!(history.cursor(), before.cursor());
        // so must one added by `add_node` or `with_nodes`
        let mut history = IndexTracker::<usize>::zero().with_nodes(alloc::vec![VertexId::new(0)]);
        assert!(matches!(
            history.next_vertex(),
            Err(IndexError::DuplicateIndex)
        ));
        history.nodes_mut().clear();
        history.add_node(VertexId::new(0));
        assert!(matches!(
            history.next_vertex(),
            Err(IndexError::DuplicateIndex)
        ));
        // `clear` resets both the history and the cursor
        history.clear();
        assert_eq!(history.next_vertex().unwrap(), 0);
    }

    #[test]
    fn test_step_overflow_is_an_error() {
        let mut id = VertexId::new(u8::MAX - 1);
        assert_eq!(id.step().unwrap(), u8::MAX - 1);
        assert!(matches!(id.step(), Err(IndexError::IndexOutOfBounds)));
        assert_eq!(id.get(), &u8::MAX);
    }
}
