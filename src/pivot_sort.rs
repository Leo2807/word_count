
use std::cmp;

pub trait Sortable {
    fn weight(&self) -> i64;
}

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
            cmp::Ordering::Less => lower.push(item),
            cmp::Ordering::Equal => equal.push(item),
            cmp::Ordering::Greater => higher.push(item),
        }
    }

    pivot_sort_high_to_low(higher);
    pivot_sort_high_to_low(lower);

    items.append(higher);
    items.append(equal);
    items.append(lower);
}
