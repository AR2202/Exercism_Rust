use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let s: String = input.graphemes(true).rev().collect();
    s
}
