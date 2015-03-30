#![stable]
//! This module contains the implementation of a pivot-based sorting algorithm.
//!
//! This is used to sort the output of the indexing algorithm, making the final output of the
//! program more readable to humans.

use std::cmp;

/// Trait to mark a sortable implementation.
///
/// Intended for use with [`pivot_sort_high_to_low`](fn.pivot_sort_high_to_low.html)
///
/// # Examples
///
/// ```
/// use lib_word_count::pivot_sort::Sortable;
///
/// struct Something {
///     x: i64,
/// }
///
/// impl Sortable for Something {
///     fn weight(&self) -> i64 {
///         self.x
///     }
/// }
///
/// let something = Something{
///     x: 2
/// };
///
/// assert_eq!(something.weight(), 2i64);
/// ```
#[stable]
pub trait Sortable {
    /// The value to sort by.
    #[stable]
    fn weight(&self) -> i64;
}

/// Sort a given vector from highest 'weight' to lowest 'weight'.
///
/// Order is derived from the return-value of `weight()`.
///
/// # Arguments
///
/// * `items`    A reference to a vector containing items that implement the Sortable trait. The items
///             will be sorted on the vector itself.
///
/// # Examples
///
/// ```
/// use lib_word_count::pivot_sort;
/// # use std::cmp;

/// # #[derive(Debug, PartialEq)]
/// # struct Something {
/// #       weight: i64,
/// #       id: i64
/// # }

/// # impl pivot_sort::Sortable for Something {
/// #       fn weight(&self) -> i64 {
/// #           self.weight
/// #       }
/// # }

/// // We use id to distinguish the order of objects with eual weight.
/// let mut sort_me = vec![
///     Something{weight: 5, id: 1},
///     Something{weight: 2, id: 1},
///     Something{weight: 1, id: 1},
///     Something{weight: 3, id: 1},
///     Something{weight: 2, id: 2},
///     Something{weight: 5, id: 2},
///     Something{weight: 4, id: 1},
/// ];
///
/// pivot_sort::pivot_sort_high_to_low(&mut sort_me);
///
/// assert_eq!(sort_me, vec![
///     Something{weight: 5, id: 1},
///     Something{weight: 5, id: 2},
///     Something{weight: 4, id: 1},
///     Something{weight: 3, id: 1},
///     Something{weight: 2, id: 1},
///     Something{weight: 2, id: 2},
///     Something{weight: 1, id: 1},
/// ]);
/// ```
#[stable]
pub fn pivot_sort_high_to_low<'a, T: Sortable>(items: &mut Vec<T>) {
    if items.len() == 0 {
        return;
    }

    let pivot = match items.pop() {
        Some(valid_pivot) => valid_pivot,
        None => unreachable!(),
    };
    let pivot_weight = pivot.weight();

    let lower: &mut Vec<T> = &mut Vec::new();
    let higher: &mut Vec<T> = &mut Vec::new();
    let equal: &mut Vec<T> = &mut vec![pivot];

    while let Some(item) = items.pop() {
        match item.weight().cmp(&pivot_weight) {
            cmp::Ordering::Less => lower.insert(0, item),
            cmp::Ordering::Equal => equal.insert(0, item),
            cmp::Ordering::Greater => higher.insert(0, item),
        }
    }

    pivot_sort_high_to_low(higher);
    pivot_sort_high_to_low(lower);

    items.append(higher);
    items.append(equal);
    items.append(lower);
}
