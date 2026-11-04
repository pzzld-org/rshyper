/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module focuses on the [`HyperEdge`] implementation, providing additional types, traits, and
//! representations for edges in a hypergraph.
#[doc(inline)]
pub use self::{hyper_edge::HyperEdge, link::Link, traits::*, types::*, utils::*};

/// the [`hyper_edge`] is responsible for defining the [`HyperEdge`] struct
pub mod hyper_edge;
pub mod link;

mod impls {
    mod impl_hyper_edge;

    mod impl_link;
    mod impl_link_ext;
    mod impl_link_repr;

    #[doc(hidden)]
    mod impl_link_deprecated;
}

mod traits {
    #[doc(inline)]
    pub use self::{layout::*, surface::*};

    mod layout;
    mod surface;
}

mod types {
    #[doc(inline)]
    pub use self::aliases::*;

    mod aliases;
}

mod utils {
    //! this module implements various utility functions for working with edges in a hypergraph
    #[doc(inline)]
    pub use self::base::*;
    #[doc(inline)]
    #[cfg(all(feature = "alloc", feature = "rand"))]
    pub use self::rand::*;

    mod base;
    #[cfg(all(feature = "alloc", feature = "rand"))]
    mod rand;
}

pub(crate) mod prelude {
    pub use super::hyper_edge::*;
    pub use super::link::*;
    pub use super::traits::*;
    pub use super::types::*;
    pub use super::utils::*;
}
