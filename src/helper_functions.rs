use regex::Regex;
use std::fmt;

// A modified version of: https://stackoverflow.com/a/9873804
pub fn get_index_spread(n: usize, size: usize) -> Option<Vec<usize>> {
    // TODO: Should either or both of these cases be an error?
    if size == 0 || n == 0 {
        return None;
    }

    let indices: Vec<usize> = if n >= size {
        // TODO: Should this case be an error?
        (0..size).collect()
    } else {
        (0..n)
            .map(|i| {
                let a = (i * size) / n;
                let b = size / (2 * n);
                a + b
            })
            .collect()
    };

    Some(indices)
}

pub fn replace_matches<T: AsRef<str> + fmt::Display>(
    text: &T,
    regular_expression: &str,
    replacement: &str,
) -> String {
    Regex::new(regular_expression)
        .unwrap()
        .replace_all(text.as_ref(), replacement)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index_spread_special_cases() {
        let output = get_index_spread(0, 1);
        assert_eq!(output, None);

        let output = get_index_spread(1, 0);
        assert_eq!(output, None);

        let output = get_index_spread(100, 8);
        assert_eq!(output.unwrap(), [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_get_index_spread_even_size() {
        // 0 -> [o, o, o, o, o, o, o, o]
        // 1 -> [o, o, o, o, |, o, o, o]
        // 2 -> [o, o, |, o, o, o, |, o]
        // 3 -> [o, |, o, |, o, o, |, o]
        // 4 -> [o, |, o, |, o, |, o, |]
        // 5 -> [|, |, o, |, |, o, |, o]
        // 6 -> [|, |, |, o, |, |, |, o]
        // 7 -> [|, |, |, |, |, |, |, o]
        // 8 -> [|, |, |, |, |, |, |, |]

        let output = get_index_spread(1, 8);
        assert_eq!(output.unwrap(), [4]);

        let output = get_index_spread(2, 8);
        assert_eq!(output.unwrap(), [2, 6]);

        let output = get_index_spread(3, 8);
        assert_eq!(output.unwrap(), [1, 3, 6]);

        let output = get_index_spread(4, 8);
        assert_eq!(output.unwrap(), [1, 3, 5, 7]);

        let output = get_index_spread(5, 8);
        assert_eq!(output.unwrap(), [0, 1, 3, 4, 6]);

        let output = get_index_spread(6, 8);
        assert_eq!(output.unwrap(), [0, 1, 2, 4, 5, 6]);

        let output = get_index_spread(7, 8);
        assert_eq!(output.unwrap(), [0, 1, 2, 3, 4, 5, 6]);

        let output = get_index_spread(8, 8);
        assert_eq!(output.unwrap(), [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_get_index_spread_odd_size() {
        // 0 -> [o, o, o, o, o]
        // 1 -> [o, o, |, o, o]
        // 2 -> [o, |, o, |, o]
        // 3 -> [|, |, o, |, o]
        // 4 -> [|, |, |, |, o]
        // 5 -> [|, |, |, |, |]

        let output = get_index_spread(1, 5);
        assert_eq!(output.unwrap(), [2]);

        let output = get_index_spread(2, 5);
        assert_eq!(output.unwrap(), [1, 3]);

        let output = get_index_spread(3, 5);
        assert_eq!(output.unwrap(), [0, 1, 3]);

        let output = get_index_spread(4, 5);
        assert_eq!(output.unwrap(), [0, 1, 2, 3]);

        let output = get_index_spread(5, 5);
        assert_eq!(output.unwrap(), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_replace_matches() {
        assert_eq!(
            replace_matches(&"Hello      Sir", "[' ']{1,}", " "),
            "Hello Sir"
        )
    }
}
