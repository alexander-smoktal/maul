#![macro_use]

use std::iter::Peekable;

/// Macro to create String -> T hash map from list of static string and values
macro_rules! string_hash_map {
    [$(($key: expr, $value: expr)), *] => ({
        let mut result = HashMap::new();
        $(
            result.insert($key.to_string(), $value)
         ); *;
        result
     })
}


/// Iterator to advance iterator until next value meets requirements
pub struct ExclusiveTakeWhile<'a, T, P>
    where T: 'a,
          T: Iterator
{
    iter: &'a mut Peekable<T>,
    pred: P,
}

impl<'a, T: Iterator, P> Iterator for ExclusiveTakeWhile<'a, T, P>
    where P: FnMut(&T::Item) -> bool
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut good = false;

        if let Some(val) = self.iter.peek() {
            good = (self.pred)(val);
        }

        if good {
            self.iter.next()
        } else {
            None
        }
    }
}

pub fn take_while_exclusive<'a, T, P>(iter: &'a mut Peekable<T>,
                                      predicate: P)
                                      -> ExclusiveTakeWhile<'a, T, P>
    where P: FnMut(&T::Item) -> bool,
          T: Iterator
{
    ExclusiveTakeWhile::<T, P> {
        iter: iter,
        pred: predicate,
    }
}