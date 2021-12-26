pub fn get_evenly_seleted_indices(items_count: usize, distribution_value: usize) -> Vec<usize> {
    let none_number = items_count - distribution_value;
    let mut indices: Vec<Option<usize>> = (0..items_count).map(Some).collect();



    let indices: Vec<usize> = indices.into_iter().flatten().collect();

    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_get_evenly_seleted_indices() {
        // [0, 0, 0, 0, 0, 0, 0, 0];
        let output = get_evenly_seleted_indices(8, 3);
        assert_eq!(output, [0, 3, 6]);

        // let output = get_evenly_seleted_indices(&items, 5);
        // assert_eq!(output, [1, 0, 0, 1, 0, 0, 1, 0])
    }
}

// [1, 0, 1, 0, 1, 0, 0, 0]
// 3 -> 8 / 3 = 2

// [1, 0, 1, 0, 1, 0, 0, 0]fjdjdfjfjfjdjdjfj
// 3 -> 8 / 3 = 2

// [1, 0, 1, 0, 1, 0, 1, 0]
// 5 -> 4


// [Some(0), None, Some(2), None, Some(4), Some(5), Some(6), Some(7), None]
// [0, 2, 4, 5, 6, 7]
