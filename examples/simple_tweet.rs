use demoji_rs::{find_emoji, remove_emoji};

fn main() {
    let text_with_emoji = "Hello, world🌏!😃";
    let text_without_emoji = remove_emoji(text_with_emoji);
    let found_emojis = find_emoji(&text_with_emoji);
    println!("Before demoji: {}", text_with_emoji);
    println!("After demoji: {}", text_without_emoji);
    println!("Found emoji: {:?}", found_emojis);
}
