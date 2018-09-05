#![macro_use]

use std::iter::Peekable;

macro_rules! log_debug {
    ($fmt:expr) => (
        #[cfg(debug_assertions)]
        println!($fmt));
    ($fmt:expr, $($arg:tt)*) => (
        #[cfg(debug_assertions)]
        println!($fmt, $($arg)*));
}

/// Macro to create String -> T hash map from list of static string and values
macro_rules! string_hash_map {
    [$(($key: expr, $value: expr)), *,] => ({
        let mut result = HashMap::new();
        $(
            result.insert($key.to_string(), $value)
         ); *;
        result
     })
}

/// Iterator to advance iterator until next value meets requirements
pub struct ExclusiveTakeWhile<'a, T, P>
where
    T: 'a,
    T: Iterator,
{
    iter: &'a mut Peekable<T>,
    pred: P,
}

impl<'a, T: Iterator, P> Iterator for ExclusiveTakeWhile<'a, T, P>
where
    P: FnMut(&T::Item) -> bool,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if if let Some(val) = self.iter.peek() {
            (self.pred)(val)
        } else {
            false
        } {
            self.iter.next()
        } else {
            None
        }
    }
}

// Add function to Peekable
pub trait AsExclusiveTakeWhile<'a, T>
where
    T: Iterator,
{
    fn take_while_exclusive<P>(self, predicate: P) -> ExclusiveTakeWhile<'a, T, P>
    where
        P: FnMut(&T::Item) -> bool;
}

impl<'a, T> AsExclusiveTakeWhile<'a, T> for &'a mut Peekable<T>
where
    T: Iterator,
{
    fn take_while_exclusive<P>(self, predicate: P) -> ExclusiveTakeWhile<'a, T, P> {
        ExclusiveTakeWhile::<T, P> {
            iter: self,
            pred: predicate,
        }
    }
}
