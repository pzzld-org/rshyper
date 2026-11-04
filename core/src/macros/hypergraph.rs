/*
    appellation: hypergraph <module>
    authors: @FL03
*/
#![cfg(feature = "macros")]

/// the [`hypergraph`](crate::hypergraph) macro works to aide the in the creation of
/// hypergraphs by allowing users to define nodes and edges in a hypergraph in a more
/// declarative way.
///
/// ## Example
///
/// Runnable examples require a concrete hypergraph, such as the `HyperMap`, and are
/// maintained within the [`rshyper` crate docs][rshyper-macros].
///
/// [rshyper-macros]: https://docs.rs/rshyper/latest/rshyper/#example-2-macros
#[macro_export]
macro_rules! hypergraph {
    (
        let mut $graph:ident: $H:ty {
            nodes: {$($nodes:tt)*};
            edges: {$($edges:tt)*};
        }
    ) => {
        // initialize a new hypergraph with the given type
        let mut $graph: $H = <$H>::new();
        // insert nodes into the graph
        $crate::hypernode!($graph {$($nodes)*});
        // insert edges into the graph
        $crate::hyperedge!($graph {$($edges)*});
    };
    (
        $graph:ident {
            nodes: {$($nodes:tt)*};
            edges: {$($edges:tt)*};
        }
    ) => {
        // insert nodes into the graph
        $crate::hypernode!($graph {$($nodes)*});
        // insert edges into the graph
        $crate::hyperedge!($graph {$($edges)*});
    };
}
/// The `hyperedge` macro streamlines the definition of hyperedges in a hypergraph.
///
/// ## Usage
///
/// The macro requires that you to pass a mutable reference to some hypergraph by first
/// defining the ident of the associated graph. Once declared, hyperedges are defined as `let`
/// statement within a block, where each statement defines a hyperedge by its name and the
/// vertices it connects. Instead of specifying the type of hyperedge, a slice of node indices
/// is used to define the edge consituents. Optionally, a weight may be specified for the
/// hyperedge by appending `= <weight>` immediately after the desired edge statement.
///
/// ```ignore
/// hyperedge! {
///     ${graph} {
///         let ${edge_name}: [${vertex1}, ${vertex2}, ...] $(= ${weight})?;
///     }
/// }
/// ```
///
/// ## Basic Usage
///
/// Runnable examples require a concrete hypergraph, such as the `HyperMap`, and are
/// maintained within the [`rshyper` crate docs][rshyper-macros].
///
/// [rshyper-macros]: https://docs.rs/rshyper/latest/rshyper/#example-2-macros
#[macro_export]
macro_rules! hyperedge {
    ($src:ident { $(let $edge:ident: [$($var:ident),*] $(= $w:expr)?);* $(;)? }) => {
        $(
            $crate::hyperedge!(@impl let $src.$edge: [$($var),*] $(= $w)?);
        )*
    };
    (@impl let $src:ident.$edge:ident: [$($var:ident),*] $(= $w:expr)?) => {
        $crate::hyperedge!(@new let $src.$edge = [$($var),*] $(=> $w)?);
    };
    (@new let $src:ident.$edge:ident = [$($var:ident),*]) => {
        let $edge = $src.add_link([$($var),*]).expect("Failed to insert edge");
    };
    (@new let $src:ident.$edge:ident = [$($var:ident),*] => $w:expr) => {
        let $edge = $src.add_edge([$($var),*], $crate::Weight($w)).expect("Failed to insert edge");
    };
}
/// the [`hypernode`](crate::hypernode) macro streamlines the process of inserting nodes into a
/// hypergraph.
///
/// ## Usage
///
/// The macro requires that you to pass a mutable reference to some hypergraph by first
/// defining the ident of the associated graph. Once declared, hypernodes are defined as `let`
/// statement within a block, where each statement defines the identifier of a node.
/// Optionally, a weight may be specified for the hypernode by appending `= <weight>`.
///
/// ```ignore
/// hypernode! {
///     $graph: {
///         let $var:ident $(= $w:expr)?;
///     }
/// }
/// ```
///
/// ### Example
///
/// Runnable examples require a concrete hypergraph, such as the `HyperMap`, and are
/// maintained within the [`rshyper` crate docs][rshyper-macros].
///
/// [rshyper-macros]: https://docs.rs/rshyper/latest/rshyper/#example-2-macros
#[macro_export]
macro_rules! hypernode {
    ($src:ident { $(let $var:ident $(= $w:expr)?);* $(;)? }) => {
        $($crate::hypernode!(@impl $src.$var $(= $w)?);)*
    };
    (@impl $src:ident.$var:ident $(= $w:expr)?) => {
        $crate::hypernode!(@new $src.$var $(= $w)?);
    };
    (@new $src:ident.$var:ident = $w:expr) => {
        let $var = $src.add_node($crate::Weight($w)).expect("Failed to insert node");
    };
    (@new $src:ident.$var:ident) => {
        let $var = $src.add_vertex().expect("Failed to insert node");
    };
}
