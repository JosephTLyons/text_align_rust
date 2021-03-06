# text_align

[A Rust crate that provides a single trait for various alignments of text](https://crates.io/crates/text_align)

```rust
use text_align::TextAlign;

fn main() {
    assert_eq!(
        "                          Hello my dearest friend!".left_align(50),
        "Hello my dearest friend!                          "
    );

    assert_eq!(
        "Hello my dearest friend!".right_align(50),
        "                          Hello my dearest friend!"
    );

    assert_eq!(
        "Hello my dearest friend!".center_align(50),
        "             Hello my dearest friend!             "
    );

    assert_eq!(
        "Hello my dearest friend!".justify(50),
        "Hello          my          dearest         friend!"
    );

    assert_eq!(
        "Hello          my          dearest         friend!".dejustify(2),
        "Hello my dearest friend!"
    );
}
```
