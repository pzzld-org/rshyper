/*
    appellation: impl_weight <module>
    authors: @FL03
*/
use crate::weight::{Weight, Weightless};

/// the base implemenation of [`Weight`] that is generic over type `T`
impl<T> Weight<T> {
    /// returns a new instance of the [`Weight`] created from the given value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }
    /// generates a new instance of the [`Weight`] using the provided function
    pub fn init<F>(value: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Self::new(value())
    }
    /// returns a new instance of the [`Weight`] with the inner value set to `1`
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Self::init(T::one)
    }
    /// returns a new instance of the [`Weight`] with a value of `0`
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self::init(T::zero)
    }
    /// returns an immutable reference to the inner value.
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    #[inline]
    /// consumes the current instance to return the inner value
    pub fn value(self) -> T {
        self.0
    }
    /// applies the provided function onto the inner value and returns a new [`Weight`] with
    /// the result.
    #[inline]
    pub fn map<U, F>(self, f: F) -> Weight<U>
    where
        F: FnOnce(T) -> U,
    {
        Weight::new(f(self.value()))
    }
    /// apply the function onto a mutable reference to the inner value and return a
    /// mutable reference to the current instanc storing the updating weight.
    #[inline]
    pub fn map_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        // apply the function onto the inner value
        f(self.get_mut());
        self
    }
    /// [`replace`](core::mem::replace) the inner value and return the previous value.
    pub const fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }
    /// updates the inner value with the provided value and returns a mutable reference to the
    /// current instance.
    #[inline]
    pub fn set(&mut self, value: T) -> &mut Self {
        *self.get_mut() = value;
        self
    }
    /// [`swap`](core::mem::swap) the inner value with another [`Weight`].
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    /// [`take`](core::mem::take) the inner value, leaving the logical default in its place.
    #[inline]
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// returns a constant pointer to the inner value; see [`core::ptr::addr_of!`] for more
    /// information
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::addr_of!(self.0)
    }
    /// returns a mutable pointer to the inner value; see [`core::ptr::addr_of_mut!`] for more
    /// information
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::addr_of_mut!(self.0)
    }
    /// returns a _view_ of the weight whose inner value is a reference to the original.
    pub const fn view(&self) -> Weight<&T> {
        Weight::new(self.get())
    }
    /// returns a _view_ of the weight whose inner value is a mutable reference to the original
    pub const fn view_mut(&mut self) -> Weight<&mut T> {
        Weight::new(self.get_mut())
    }
    /// consumes the current instance to create another with the given value
    #[inline]
    pub fn with<U>(self, value: U) -> Weight<U> {
        self.map(|_| value)
    }
    /// returns true if:
    ///
    /// - the weight is not of type [`Weightless`]
    /// - the inner value is not the default value of the type `T`
    ///
    /// rather than simply checking if the weight is not of type [`Weightless`], this method
    /// includes additional assertions for quickly checking the state of a weight.
    pub fn is_weighted(&self) -> bool
    where
        T: 'static + Default + PartialEq,
    {
        !self.is_weightless() && self.get() != &T::default()
    }
    /// returns true if the weight is of type [`Weightless`]
    pub fn is_weightless(&self) -> bool
    where
        T: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<Self>() != TypeId::of::<Weightless<T>>()
    }
}

impl<T> AsRef<T> for Weight<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Weight<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Weight<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Weight<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Weight<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Weight<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> From<T> for Weight<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> PartialEq<T> for Weight<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<'a, T> PartialEq<&'a T> for Weight<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a T) -> bool {
        self.get() == *other
    }
}

impl<'a, T> PartialEq<&'a mut T> for Weight<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut T) -> bool {
        self.get() == *other
    }
}

impl<T> PartialOrd<T> for Weight<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}
