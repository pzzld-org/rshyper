/*
    Appellation: indicies <module>
    Created At: 2026.01.10:11:03:39
    Contrib: @FL03
*/
#![cfg(feature = "macros")]

#[macro_export]
macro_rules! vertex_id {
    ($idx:ty) => {
        $crate::idx::VertexId<$idx>
    };
    ($idx:expr) => {
        $crate::idx::VertexId::new($idx)
    };

}
