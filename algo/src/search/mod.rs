/*
    Appellation: search <module>
    Created At: 2026.01.06:19:08:10
    Contrib: @FL03
*/
//! search alogithms for hypergraphs
#[doc(inline)]
pub use self::{
    astar::AStarSearch, breadth_first::BreadthFirstTraversal, depth_first::DepthFirstTraversal,
    dijkstra::Dijkstra, traits::*, walks::*,
};

mod impls {
    mod impl_astar;
    mod impl_breadth_first;
    mod impl_depth_first;
    mod impl_dijkstra;
}

pub mod astar;
pub mod breadth_first;
pub mod depth_first;
pub mod dijkstra;
pub mod walks;

mod traits {
    #[doc(inline)]
    pub use self::{heuristic::*, search::*};

    mod heuristic;
    mod search;
}

#[doc(hidden)]
pub mod prelude {
    pub use super::traits::*;

    pub use super::astar::AStarSearch;
    pub use super::breadth_first::BreadthFirstTraversal;
    pub use super::depth_first::DepthFirstTraversal;
    pub use super::dijkstra::Dijkstra;
    pub use super::walks::{PathLimits, Walk, Walks};
}
