//! ```
//! use text_align::TextAlign;
//! assert_eq!(
//!     "                          Hello my dearest friend!".left_align(50),
//!     "Hello my dearest friend!                          "
//! );
//!
//! assert_eq!(
//!     "Hello my dearest friend!".right_align(50),
//!     "                          Hello my dearest friend!"
//! );
//!
//! assert_eq!(
//!     "Hello my dearest friend!".center_align(50),
//!     "             Hello my dearest friend!             "
//! );
//!
//! assert_eq!(
//!     "Hello my dearest friend!".justify(50),
//!     "Hello          my          dearest         friend!"
//! );
//!
//! assert_eq!(
//!     "Hello          my          dearest         friend!".dejustify(2),
//!     "Hello my dearest friend!"
//! );
//! ```

mod helper_functions;

use helper_functions::{get_index_spread, replace_matches};
use std::fmt;

pub trait TextAlign {
    fn center_align(&self, width: usize) -> String;
    fn left_align(&self, width: usize) -> String;
    fn right_align(&self, width: usize) -> String;
    fn justify(&self, width: usize) -> String;
    fn dejustify(&self, spaces_after_punctuation: usize) -> String;
}

// TODO: Any way to make this return either `&str` or `String`?
impl<T: AsRef<str> + fmt::Display> TextAlign for T {
    fn center_align(&self, mut width: usize) -> String {
        let mut str_ref = self.as_ref().trim_start();
        let text_length = str_ref.len();
        let has_newline = str_ref.ends_with('\n');
        str_ref = str_ref.trim_end();

        if has_newline {
            width += 1;
        }

        if width <= text_length {
            return self.to_string();
        }

        let spaces = width - text_length;
        let left_padding_length = spaces / 2;
        let right_padding_length = spaces - left_padding_length;
        let last_character = if has_newline { "\n" } else { "" };

        format!(
            "{}{}{}{}",
            " ".repeat(left_padding_length),
            str_ref,
            " ".repeat(right_padding_length),
            last_character
        )
    }

    fn left_align(&self, mut width: usize) -> String {
        let mut str_ref = self.as_ref().trim_start();
        let has_newline = str_ref.ends_with('\n');
        let text_length = str_ref.len();
        str_ref = str_ref.trim_end();

        if has_newline {
            width += 1;
        }

        if width <= text_length {
            return self.to_string();
        }

        let padding_length = width - text_length;
        let last_character = if has_newline { "\n" } else { "" };

        format!("{}{}{}", str_ref, " ".repeat(padding_length), last_character)
    }

    fn right_align(&self, mut width: usize) -> String {
        let mut str_ref = self.as_ref().trim_start();
        let has_newline = str_ref.ends_with('\n');
        let text_length = str_ref.len();
        str_ref = str_ref.trim_end();

        if has_newline {
            width += 1;
        }

        if width <= text_length {
            return self.to_string();
        }

        let padding_length = width - text_length;
        let last_character = if has_newline { "\n" } else { "" };

        format!("{}{}{}", " ".repeat(padding_length), str_ref, last_character)
    }

    fn justify(&self, width: usize) -> String {
        let mut str_ref = self.as_ref();
        let has_newline = str_ref.ends_with('\n');

        if has_newline {
            str_ref = str_ref.trim_end();
        }

        if width <= str_ref.len() {
            return self.to_string();
        }

        let words: Vec<&str> = str_ref.split_ascii_whitespace().collect();
        let length_of_words: usize = words.iter().map(|word| word.len()).sum();
        let spaces_required = width - length_of_words;
        let space_blocks_required = words.len() - 1;
        let spaces_per_block = spaces_required / space_blocks_required;
        let remaining_spaces = spaces_required % space_blocks_required;

        let mut space_counts = vec![spaces_per_block; space_blocks_required];

        if let Some(indices) = get_index_spread(remaining_spaces, space_counts.len()) {
            for index in indices {
                space_counts[index] += 1
            }
        }

        // We must have an equal number of words and space blocks to prevent zip() from
        // short-circuiting
        // We currently have 1 less space block
        // We can push a 0 to the end, which will ultimately result in the last word having an empty
        // string appended to it
        space_counts.push(0);
        assert_eq!(words.len(), space_counts.len());

        let last_character = if has_newline { "\n" } else { "" };

        let text: String = words
            .iter()
            .zip(space_counts.iter())
            .map(|(word, space_count)| format!("{}{}", word, " ".repeat(*space_count)))
            .collect::<Vec<String>>()
            .join("");

        format!("{}{}", text, last_character)
    }

    // TODO: Gross and probably inefficient
    fn dejustify(&self, spaces_after_punctuation: usize) -> String {
        // Normalize all space groupings in between words to a single space
        let mut text = replace_matches(&self, "[' ']{2,}", " ");

        if spaces_after_punctuation > 1 {
            let padding_string = " ".repeat(spaces_after_punctuation);

            let regular_expressions_and_replacements = [
                (r"\.[' ']", format!(".{}", padding_string)),
                (r"\?[' ']", format!("?{}", padding_string)),
                (r"![' ']", format!("!{}", padding_string)),
            ];

            // Adjust (the now) single spaces after punctuation to be whatever the user requests
            for (regular_expression, replacement) in regular_expressions_and_replacements {
                text = replace_matches(&text, regular_expression, &replacement);
            }
        }

        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_align_even_length_text() {
        // Non-newline tests
        assert_eq!("hi".center_align(3), "hi ");
        assert_eq!("hi".center_align(5), " hi  ");
        assert_eq!("hi".center_align(8), "   hi   ");

        // Newline tests
        assert_eq!("hi\n".center_align(3), "hi \n");
        assert_eq!("hi\n".center_align(5), " hi  \n");
        assert_eq!("hi\n".center_align(8), "   hi   \n");
    }

    #[test]
    fn test_center_align_odd_length_text() {
        // Non-newline tests
        assert_eq!("doggy".center_align(3), "doggy");
        assert_eq!("doggy".center_align(8), " doggy  ");
        assert_eq!("doggy".center_align(9), "  doggy  ");

        // Newline tests
        assert_eq!("doggy\n".center_align(3), "doggy\n");
        assert_eq!("doggy\n".center_align(8), " doggy  \n");
        assert_eq!("doggy\n".center_align(9), "  doggy  \n");
    }

    #[test]
    fn test_right_align() {
        // Non-newline tests
        assert_eq!("hi".right_align(1), "hi");
        assert_eq!("hi".right_align(3), " hi");
        assert_eq!("hi".right_align(5), "   hi");

        // Newline tests
        assert_eq!("hi\n".right_align(1), "hi\n");
        assert_eq!("hi\n".right_align(3), " hi\n");
        assert_eq!("hi\n".right_align(5), "   hi\n");
    }

    #[test]
    fn test_left_align() {
        // Non-newline tests
        assert_eq!("hi".left_align(1), "hi");
        assert_eq!("hi".left_align(3), "hi ");
        assert_eq!(" hi".left_align(5), "hi   ");

        // Newline tests
        assert_eq!("hi\n".left_align(1), "hi\n");
        assert_eq!(" hi\n".left_align(3), "hi \n");
        assert_eq!(" hi\n".left_align(5), "hi   \n");
    }

    #[test]
    fn test_justify_sentence() {
        // Non-newline tests
        assert_eq!("Good dog".justify(1), "Good dog");
        assert_eq!("Good dog".justify(8), "Good dog");
        assert_eq!("Good dog".justify(9), "Good  dog");
        assert_eq!("Good dog".justify(10), "Good   dog");
        assert_eq!("Really good dog".justify(16), "Really good  dog");
        assert_eq!("Really good dog".justify(17), "Really  good  dog");
        assert_eq!("Really good dog".justify(18), "Really  good   dog");

        // Newline tests
        assert_eq!("Good dog\n".justify(1), "Good dog\n");
        assert_eq!("Good dog\n".justify(8), "Good dog\n");
        assert_eq!("Good dog\n".justify(9), "Good  dog\n");
        assert_eq!("Good dog\n".justify(10), "Good   dog\n");
        assert_eq!("Really good dog\n".justify(16), "Really good  dog\n");
        assert_eq!("Really good dog\n".justify(17), "Really  good  dog\n");
        assert_eq!("Really good dog\n".justify(18), "Really  good   dog\n");
    }

    #[test]
    fn test_dejustify() {
        // Non-newline tests
        assert_eq!(
            "Hi    bud.    How    are    you?".dejustify(1),
            "Hi bud. How are you?"
        );
        assert_eq!(
            "Hi    bud.    How    are    you?".dejustify(2),
            "Hi bud.  How are you?"
        );
        assert_eq!(
            "Hi!    Hey?    Hello.    Bud.".dejustify(2),
            "Hi!  Hey?  Hello.  Bud."
        );
        assert_eq!(
            "Hi!    Hey?\nHello.    Bud.".dejustify(1),
            "Hi! Hey?\nHello. Bud."
        );

        // Newline tests
        assert_eq!(
            "Hi    bud.    How    are    you?\n".dejustify(1),
            "Hi bud. How are you?\n"
        );
        assert_eq!(
            "Hi    bud.    How    are    you?\n".dejustify(2),
            "Hi bud.  How are you?\n"
        );
        assert_eq!(
            "Hi!    Hey?    Hello.    Bud.\n".dejustify(2),
            "Hi!  Hey?  Hello.  Bud.\n"
        );
        assert_eq!(
            "Hi!    Hey?\nHello.    Bud.\n".dejustify(1),
            "Hi! Hey?\nHello. Bud.\n"
        );
    }
}
