/*
    appellation: heuristic <module>
    authors: @FL03
*/
use rshyper::idx::{self, VertexId};
/// [`Heuristic`] defines a common interface for heuristic functions compatible with the [`A*`](crate::AStarSearch)
/// search implementation
pub trait Heuristic<T = idx::Udx> {
    type Output;

    fn compute(&self, start: VertexId<T>, goal: VertexId<T>) -> Self::Output;
}

/*
 ************* Implementations *************
*/

impl<U, F, Idx> Heuristic<Idx> for F
where
    Idx: idx::RawIndex,
    F: Fn(VertexId<Idx>, VertexId<Idx>) -> U,
{
    type Output = U;

    fn compute(&self, start: VertexId<Idx>, goal: VertexId<Idx>) -> Self::Output {
        self(start, goal)
    }
}
