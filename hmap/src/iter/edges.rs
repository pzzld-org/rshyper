/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module provides various iterators over the edges of a hypergraph, allowing users to
//! traverse sequentially, directionally, or even in parallel.
#[doc(inline)]
pub use self::prelude::*;

mod directional;
mod iter;
#[cfg(feature = "rayon")]
mod parallel;
mod sequential;

pub(crate) mod prelude {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::directional::*;
    #[doc(inline)]
    pub use super::iter::*;
    #[doc(inline)]
    #[cfg(feature = "rayon")]
    pub use super::parallel::*;
    #[doc(inline)]
    pub use super::sequential::*;
}
