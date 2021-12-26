mod helper_functions;

use helper_functions::{get_evenly_seleted_indices, replace_matches};
use std::fmt;

pub trait TextAlign {
    fn center_align(&self, width: usize, should_include_trailing_whitespace: bool) -> String;
    fn left_align(&self) -> String;
    fn right_align(&self, width: usize) -> String;
    fn justify(&self, width: usize) -> String;
    fn dejustify(&self, spaces_after_punctuation: usize) -> String;
}

// TODO: Any way to make this return either `&str` or `String`?
impl<T: AsRef<str> + fmt::Display> TextAlign for T {
    fn center_align(&self, mut width: usize, should_include_trailing_whitespace: bool) -> String {
        let mut str_ref = self.as_ref();
        let text_length = str_ref.len();
        let has_newline = str_ref.ends_with("\n");

        if has_newline {
            str_ref = str_ref.trim_end();
            width += 1;
        }

        if width <= text_length {
            return self.to_string();
        }

        let spaces = width - text_length;
        let left_padding_length = spaces / 2;

        let right_padding_length = if should_include_trailing_whitespace {
            spaces - left_padding_length
        } else {
            0
        };

        let last_character = if has_newline { "\n" } else { "" };

        format!(
            "{}{}{}{}",
            " ".repeat(left_padding_length),
            str_ref,
            " ".repeat(right_padding_length),
            last_character
        )
    }

    // Because left alignment will only ever shorten the length of a line, we dont need to worry
    // about staying within a width, so we don't even consider it
    fn left_align(&self) -> String {
        self.as_ref().trim_start().to_string()
    }

    fn right_align(&self, mut width: usize) -> String {
        let str_ref = self.as_ref();
        let text_length = str_ref.len();

        if str_ref.ends_with("\n") {
            width += 1;
        }

        if width <= text_length {
            return self.to_string();
        }

        let padding_length = width - text_length;
        let padding_string = " ".repeat(padding_length);

        format!("{}{}", padding_string, self)
    }

    fn justify(&self, width: usize) -> String {
        let str_ref = self.as_ref();

        if width <= str_ref.len() {
            return self.to_string();
        }

        let words: Vec<&str> = str_ref.split_ascii_whitespace().collect();
        let length_of_words: usize = words.iter().map(|word| word.len()).sum();
        let spaces_required = width - length_of_words;
        let space_blocks_required = words.len() - 1;
        let spaces_per_block = spaces_required / space_blocks_required;
        let remaining_spaces = spaces_required % space_blocks_required;

        // dbg!(&words);
        // dbg!(&length_of_words);
        // dbg!(&spaces_required);
        // dbg!(&space_blocks_required);
        // dbg!(&spaces_per_block);
        // dbg!(&remaining_spaces);

        let mut space_counts = vec![spaces_per_block; space_blocks_required];
        let indices_to_increment = get_evenly_seleted_indices(space_counts.len(), remaining_spaces);

        // for index in indices_to_increment {
        //     space_counts[index] += 1
        // }

        // We must have an equal number of words and space blocks to prevent zip() from
        // short-circuiting
        // We currently have 1 less space block
        // We can push a 0 to the end, which will ultimately result in the last word having an empty
        // string appended to it
        space_counts.push(0);
        assert_eq!(words.len(), space_counts.len());

        let text: String = words
            .iter()
            .zip(space_counts.iter())
            .map(|(word, space_count)| format!("{}{}", word, " ".repeat(*space_count)))
            .collect::<Vec<String>>()
            .join("");

        text
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
        assert_eq!("hi".center_align(3, false), "hi");
        assert_eq!("hi".center_align(3, true), "hi ");

        assert_eq!("hi".center_align(5, false), " hi");
        assert_eq!("hi".center_align(5, true), " hi  ");

        assert_eq!("hi".center_align(8, false), "   hi");
        assert_eq!("hi".center_align(8, true), "   hi   ");

        assert_eq!("hi\n".center_align(3, false), "hi\n");
        assert_eq!("hi\n".center_align(3, true), "hi \n");

        assert_eq!("hi\n".center_align(5, false), " hi\n");
        assert_eq!("hi\n".center_align(5, true), " hi  \n");

        assert_eq!("hi\n".center_align(8, false), "   hi\n");
        assert_eq!("hi\n".center_align(8, true), "   hi   \n");
    }

    #[test]
    fn test_center_align_odd_length_text() {
        assert_eq!("doggy".center_align(3, false), "doggy");
        assert_eq!("doggy".center_align(3, true), "doggy");

        assert_eq!("doggy".center_align(8, false), " doggy");
        assert_eq!("doggy".center_align(8, true), " doggy  ");

        assert_eq!("doggy".center_align(9, false), "  doggy");
        assert_eq!("doggy".center_align(9, true), "  doggy  ");

        assert_eq!("doggy\n".center_align(3, false), "doggy\n");
        assert_eq!("doggy\n".center_align(3, true), "doggy\n");

        assert_eq!("doggy\n".center_align(8, false), " doggy\n");
        assert_eq!("doggy\n".center_align(8, true), " doggy  \n");

        assert_eq!("doggy\n".center_align(9, false), "  doggy\n");
        assert_eq!("doggy\n".center_align(9, true), "  doggy  \n");
    }

    #[test]
    fn test_right_align() {
        assert_eq!("hi".right_align(1), "hi");
        assert_eq!("hi".right_align(3), " hi");
        assert_eq!("hi".right_align(5), "   hi");

        assert_eq!("hi\n".right_align(1), "hi\n");
        assert_eq!("hi\n".right_align(3), " hi\n");
        assert_eq!("hi\n".right_align(5), "   hi\n");
    }

    #[test]
    fn test_left_align() {
        assert_eq!("hi".left_align(), "hi");
        assert_eq!(" hi".left_align(), "hi");

        assert_eq!("hi\n".left_align(), "hi\n");
        assert_eq!(" hi\n".left_align(), "hi\n");
    }

    #[test]
    fn test_justify_sentence() {
        assert_eq!("Good dog".justify(1), "Good dog");
        assert_eq!("Good dog".justify(8), "Good dog");
        assert_eq!("Good dog".justify(9), "Good  dog");
        assert_eq!("Good dog".justify(10), "Good   dog");
        // assert_eq!("Really good dog".justify(16), "Really  good dog");
        // assert_eq!("Really good dog".justify(17), "Really  good  dog");
        // assert_eq!("Really good dog".justify(18), "Really   good  dog");

        assert_eq!("Good dog\n".justify(1), "Good dog\n");
        assert_eq!("Good dog\n".justify(8), "Good dog\n");
        assert_eq!("Good dog\n".justify(9), "Good  dog\n");
        assert_eq!("Good dog\n".justify(10), "Good   dog\n");
        // assert_eq!("Really good dog".justify(16), "Really  good dog");
        // assert_eq!("Really good dog".justify(17), "Really  good  dog");
        // assert_eq!("Really good dog".justify(18), "Really   good  dog");
    }

    #[test]
    fn test_dejustify() {
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

// TODO: Add tests for including whitespace, refactor that funtion again
