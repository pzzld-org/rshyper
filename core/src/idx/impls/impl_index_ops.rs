/*
    appellation: impl_index <module>
    authors: @FL03
*/
use crate::idx::{IndexBase, IndexType, RawIndex};
use crate::traits::StepWith;
use core::cmp::Ordering;
use num_traits::{Num, One, Zero};

impl<T, K> StepWith<T> for IndexBase<T, K>
where
    K: IndexType,
    T: RawIndex,
{
    type Output = IndexBase<T, K>;

    fn step_with<F>(&mut self, f: F) -> Self::Output
    where
        F: FnOnce(&T) -> T,
    {
        // compute the next value using the provided function
        let next = f(self.get());
        // replace the current value with the next one
        let prev = self.replace(next);
        // return the previous instance
        Self::new(prev)
    }
}

impl<T, K> Eq for IndexBase<T, K> where T: Eq {}

impl<T, K> PartialEq<T> for IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.value == other
    }
}

impl<'a, T, K> PartialEq<&'a T> for IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a T) -> bool {
        &self.value == *other
    }
}

impl<'a, T, K> PartialEq<&'a mut T> for IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut T) -> bool {
        &self.value == *other
    }
}

impl<T, K> PartialEq<IndexBase<T, K>> for IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &IndexBase<T, K>) -> bool {
        self.value == other.value
    }
}

impl<'a, T, K> PartialEq<&'a IndexBase<T, K>> for IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a IndexBase<T, K>) -> bool {
        self.value == other.value
    }
}

impl<'a, T, K> PartialEq<&'a mut IndexBase<T, K>> for IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut IndexBase<T, K>) -> bool {
        self.value == other.value
    }
}

impl<T, K> PartialEq<IndexBase<T, K>> for &IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &IndexBase<T, K>) -> bool {
        self.value == other.value
    }
}

impl<T, K> PartialEq<IndexBase<T, K>> for &mut IndexBase<T, K>
where
    T: PartialEq,
{
    fn eq(&self, other: &IndexBase<T, K>) -> bool {
        self.value == other.value
    }
}

impl<T, K> PartialOrd<T> for IndexBase<T, K>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<'a, T, K> PartialOrd<&'a T> for IndexBase<T, K>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a T) -> Option<Ordering> {
        self.value.partial_cmp(*other)
    }
}

impl<'a, T, K> PartialOrd<&'a mut T> for IndexBase<T, K>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut T) -> Option<Ordering> {
        self.value.partial_cmp(*other)
    }
}

impl<T, K> core::ops::Deref for IndexBase<T, K> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, K> core::ops::DerefMut for IndexBase<T, K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, K> core::ops::Not for IndexBase<T, K>
where
    T: core::ops::Not,
{
    type Output = IndexBase<T::Output, K>;

    fn not(self) -> Self::Output {
        IndexBase {
            value: !self.value,
            _type: core::marker::PhantomData::<K>,
        }
    }
}

impl<T, K> core::ops::Neg for IndexBase<T, K>
where
    T: core::ops::Neg,
{
    type Output = IndexBase<T::Output, K>;

    fn neg(self) -> Self::Output {
        IndexBase {
            value: -self.value,
            _type: core::marker::PhantomData::<K>,
        }
    }
}

impl<T, K> One for IndexBase<T, K>
where
    T: One,
{
    fn one() -> Self {
        Self {
            value: T::one(),
            _type: core::marker::PhantomData::<K>,
        }
    }
}

impl<T, K> Zero for IndexBase<T, K>
where
    T: Zero,
{
    fn zero() -> Self {
        Self {
            value: T::zero(),
            _type: core::marker::PhantomData::<K>,
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<T, K> Num for IndexBase<T, K>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(|value| IndexBase {
            value,
            _type: core::marker::PhantomData::<K>,
        })
    }
}

macro_rules! impl_binary_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B, C> ::core::ops::$trait<IndexBase<B, K>> for IndexBase<A, K>
        where
            A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: IndexBase<B, K>) -> Self::Output {
                let value = ::core::ops::$trait::$method(self.value, rhs.value);
                IndexBase {
                    value,
                    _type: core::marker::PhantomData::<K>,
                }
            }
        }

        impl<'a, K, A, B, C> ::core::ops::$trait<&'a IndexBase<B, K>> for IndexBase<A, K>
        where
            A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: &'a IndexBase<B, K>) -> Self::Output {
                let value = ::core::ops::$trait::$method(self.value, &rhs.value);
                IndexBase {
                    value,
                    _type: core::marker::PhantomData::<K>,
                }
            }
        }

        impl<'a, K, A, B, C> ::core::ops::$trait<&'a mut IndexBase<B, K>> for IndexBase<A, K>
        where
            A: ::core::ops::$trait<&'a mut B, Output = C>,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: &'a mut IndexBase<B, K>) -> Self::Output {
                let value = ::core::ops::$trait::$method(self.value, &mut rhs.value);
                IndexBase {
                    value,
                    _type: core::marker::PhantomData::<K>,
                }
            }
        }

        impl<'a, K, A, B, C> ::core::ops::$trait<IndexBase<B, K>> for &'a IndexBase<A, K>
        where
            &'a A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: IndexBase<B, K>) -> Self::Output {
                let value = ::core::ops::$trait::$method(&self.value, rhs.value);
                IndexBase {
                    value,
                    _type: core::marker::PhantomData::<K>,
                }
            }
        }

        impl<'a, K, A, B, C> ::core::ops::$trait<IndexBase<B, K>> for &'a mut IndexBase<A, K>
        where
            &'a mut A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: IndexBase<B, K>) -> Self::Output {
                let value = ::core::ops::$trait::$method(&mut self.value, rhs.value);
                IndexBase {
                    value,
                    _type: core::marker::PhantomData::<K>,
                }
            }
        }

        impl<'a, K, A, B, C> ::core::ops::$trait<&'a IndexBase<B, K>> for &'a IndexBase<A, K>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: &'a IndexBase<B, K>) -> Self::Output {
                let value = ::core::ops::$trait::$method(&self.value, &rhs.value);
                IndexBase {
                    value,
                    _type: core::marker::PhantomData::<K>,
                }
            }
        }

        impl<'a, K, A, B, C> ::core::ops::$trait<&'a mut IndexBase<B, K>> for &'a mut IndexBase<A, K>
        where
            &'a mut A: ::core::ops::$trait<&'a mut B, Output = C>,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: &'a mut IndexBase<B, K>) -> Self::Output {
                let value = ::core::ops::$trait::$method(&mut self.value, &mut rhs.value);
                IndexBase {
                    value,
                    _type: core::marker::PhantomData::<K>,
                }
            }
        }
    };
    (@mut $trait:ident::$method:ident) => {
        paste::paste! {
            impl_assign_op!(@impl [<$trait Assign>]::[<$method _assign>]);
        }
    };
    ($($trait:ident::$method:ident),* $(,)?) => {
        $(
            impl_binary_op!(@impl $trait::$method);
            impl_binary_op!(@mut $trait::$method);
        )*
    };
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B> ::core::ops::$trait<B> for IndexBase<A, K>
        where
            A: ::core::ops::$trait<B>,
        {
            fn $method(&mut self, rhs: B) {
                ::core::ops::$trait::$method(&mut self.value, rhs)
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {
        $(
            impl_assign_op!(@impl $trait::$method);
        )*
    };
}

impl_binary_op! {
    Add::add,
    Sub::sub,
    Mul::mul,
    Div::div,
    Rem::rem,
    BitAnd::bitand,
    BitOr::bitor,
    BitXor::bitxor,
    Shl::shl,
    Shr::shr,
}
