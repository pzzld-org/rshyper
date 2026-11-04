/*
    Appellation: impl_index_ext <module>
    Created At: 2026.01.10:11:19:50
    Contrib: @FL03
*/
use crate::idx::{IndexBase, IndexType, RawIndex};
use crate::traits::AddStep;

macro_rules! impl_fmt {
    (
        $s:ident(
            $($trait:ident),* $(,)?
        )
    ) => {
        $(
            impl_fmt!(@impl $s($trait));
        )*
    };
    (@impl $s:ident($trait:ident)) => {
        impl<T, K> ::core::fmt::$trait for $s<T, K>
        where
            K: IndexType,
            T: ::core::fmt::$trait,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.value, f)
            }
        }
    };
}

impl_fmt! {
    IndexBase(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        Pointer,
        UpperExp,
        UpperHex,
    )
}

impl<T, K> Default for IndexBase<T, K>
where
    T: Default,
    K: IndexType,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            _type: core::marker::PhantomData::<K>,
        }
    }
}

impl<T, K> AsRef<T> for IndexBase<T, K>
where
    T: RawIndex,
    K: IndexType,
{
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T, K> AsMut<T> for IndexBase<T, K>
where
    T: RawIndex,
    K: IndexType,
{
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T, K> core::borrow::Borrow<T> for IndexBase<T, K>
where
    T: RawIndex,
    K: IndexType,
{
    fn borrow(&self) -> &T {
        self.get()
    }
}
impl<T, K> core::borrow::BorrowMut<T> for IndexBase<T, K>
where
    T: RawIndex,
    K: IndexType,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T, K> From<T> for IndexBase<T, K>
where
    T: RawIndex,
    K: IndexType,
{
    fn from(index: T) -> Self {
        Self::new(index)
    }
}

impl<T, K> Iterator for IndexBase<T, K>
where
    T: RawIndex + AddStep<Output = T>,
    K: IndexType,
{
    type Item = IndexBase<T, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.step().ok()
    }
}
