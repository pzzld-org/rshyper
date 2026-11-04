/*
    appellation: impl_weight_deprecated <module>
    authors: @FL03
*/
use crate::weight::Weight;

impl<T> Weight<T> {
    #[inline]
    #[deprecated(since = "0.2.0", note = "use `value` instead")]
    pub fn into_inner(self) -> T {
        self.0
    }
}
