# text_align

A crate that provides a single trait for various alignments of text

```rust
use text_align::TextAlign;

fn main() {
    assert_eq!(
        "          Hello my dearest friend!".left_align(),
        "Hello my dearest friend!"
    );

    assert_eq!(
        "Hello my dearest friend!".right_align(50),
        "                          Hello my dearest friend!"
    );

    assert_eq!(
        "Hello my dearest friend!".center_align(50),
        "             Hello my dearest friend!"
    );

    // justify -> Coming soon

    assert_eq!(
        "Hello     my       dearest    friend!".dejustify(2),
        "Hello my dearest friend!"
    );
}
```
