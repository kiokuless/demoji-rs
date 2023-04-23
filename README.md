# demoji_rs
`demoji_rs` is a Rust port of the popular Python library, demoji. Demoji is one of the great Python libraries that accurately find or remove emojis from a blob of text using data from the Unicode Consortium's emoji code repository. This Rust implementation brings the power and ease of use of the original Python library to the Rust ecosystem.

## Features
Find and remove emojis from text using the latest data from the Unicode Consortium's emoji code repository.
Fast and efficient emoji detection and removal for Rust applications.
Simple and easy-to-use API for developers.
Usage
To use `demoji_rs`, add it as a dependency in your Cargo.toml file and then import it in your project.

```toml
[dependencies]
demoji_rs = "0.1.0"
```

Then, you can use the `find_emoji` and `remove_emoji` functions in your Rust code.

```rust
use demoji_rs::{find_emoji, remove_emoji};

fn main() {
    let text_with_emoji = "Hello, world! 🌍😃";
    
    // Find emojis
    let emojis = find_emoji(text_with_emoji);
    println!("Emojis found: {:?}", emojis);

    // Remove emojis
    let text_without_emoji = remove_emoji(text_with_emoji);
    println!("Text without emojis: {}", text_without_emoji);
}
```

## Contributing
We welcome your contributions to `demoji_rs`! Feel free to submit issues, feature requests, or pull requests to help us improve the library and keep it up to date with the latest emoji standards.

## License
`demoji_rs` is released under the MIT License.
