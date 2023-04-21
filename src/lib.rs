use once_cell::sync::Lazy;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

type EmojiMap = HashMap<&'static str, &'static str>;

pub fn remove_emoji(text: &str) -> String {
    text.graphemes(true)
        .filter(|grapheme| !ONCE_CELL.contains_key(*grapheme))
        .collect()
}

static ONCE_CELL: Lazy<EmojiMap> = include!("emoji_map.in");
