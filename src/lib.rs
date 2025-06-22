/*! Provider of [`Upget`].

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

The [`Upget`] trate can be used for refactoring as follows.

Before refactoring.
```
fn clone_with_sort(vec: &Vec<String>) -> Vec<String> {
    let mut result = vec.clone();
    result.sort();
    result
}
```

After refactoring.
```
# use crate::upget::Upget;
fn clone_with_sort(vec: &Vec<String>) -> Vec<String> {
    vec.clone().upget(|x| x.sort())
}
```
*/

#![no_std]
#![warn(missing_docs)]

pub mod prelude;

/// Trait that handles common patterns of updating and getting value.
pub trait Upget {
    /// Update self in the specified way and get the result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::upget::Upget;
    /// assert_eq!([3, 2, 1].upget(|x| x.sort()), [1, 2, 3]);
    /// ```
    fn upget<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}

impl<T> Upget for T {}
