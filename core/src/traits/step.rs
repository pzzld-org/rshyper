/*
    appellation: step <module>
    authors: @FL03
*/

/// [`StepWith`] is a trait defining an interface that can be best described as a more flexible
/// [`take`](core::mem::take) method, however, instead of leaving the default value in place
/// of the previous one, it allows for a generator function to be provided.
pub trait StepWith<T> {
    type Output;

    fn step_with<F>(&mut self, f: F) -> Self::Output
    where
        F: FnOnce(&T) -> T;
}
/// [`StepSelf`] is a trait establishing a common interface for entities that may be progressed,
/// producing some [`Output`](StepSelf::Output) as a result. The trait is typically used to define
/// generators for indices within the hypergraph.
pub trait StepSelf {
    /// the expected output type of the step function
    type Output;

    /// progress the current state by a single step, producing some output
    fn step(&mut self) -> Self::Output;
}
/// [`AddStep`] is a trait that defines a method to add a step to the current value, replacing
pub trait AddStep<T = Self>: StepWith<T> {
    /// computes the next value by adding a step to the current one, replacing and returning it
    fn add_step(&mut self) -> Self::Output;
}

/*
 ************* Implementations *************
*/
use num_traits::One;

impl<T> AddStep<T> for T
where
    T: One + StepWith<T>,
    for<'a> &'a T: core::ops::Add<T, Output = T>,
{
    fn add_step(&mut self) -> Self::Output {
        // compute the next value by incrementing the current one
        self.step_with(|x| x + T::one())
    }
}

macro_rules! impl_step_add {
    ($($T:ty),* $(,)?) => {
        $(
            impl_step_add!(@std $T);
        )*
    };
    (@saturating $t:ty) => {
        impl StepSelf for $t {
            type Output = $t;

            /// Creates the next value by incrementing the current one.
            fn step(&mut self) -> Self::Output {
                // compute the next value by incrementing the current one
                let next = self.saturating_add(<$t>::one());
                // replace the current value with the next one
                core::mem::replace(self, next)
            }
        }

    };
    (@std $t:ty) => {
        impl StepWith<$t> for $t {
            type Output = $t;

            /// Creates the next value by incrementing the current one using a generator function.
            fn step_with<F>(&mut self, f: F) -> Self::Output
            where
                F: FnOnce(&$t) -> $t,
            {
                // compute the next value by applying the generator function to the current one
                let next = f(self);
                // replace the current value with the next one
                core::mem::replace(self, next)
            }
        }

        impl StepSelf for $t {
            type Output = $t;

            /// Creates the next value by incrementing the current one.
            fn step(&mut self) -> Self::Output {
                use num_traits::One;
                // compute the next value by incrementing the current one
                self.step_with(|x| x + <$t>::one())
            }
        }
    };
}

impl_step_add! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
}
