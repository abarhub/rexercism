use std::ops::Add;
use unicode_segmentation::UnicodeSegmentation;

fn hello_world() -> String {
    "Hello World".to_string()
}

pub fn reverse(input: &str) -> String {
    let mut res: String = "".to_string();
    for c in input.graphemes(true).rev() {
        res = res.add(c);
    }
    res
}

fn main() {
    // hello
    //println!("{}", hello_world());
    println!("{}", reverse("Hello World"));
}

#[test]
//#[ignore]
fn test_hello_world() {
    let output = hello_world();
    let expected = "Hello World";
    assert_eq!(output, expected);
}

#[test]
//#[ignore]
fn a_word() {
    let input = "robot";
    let output = reverse(input);
    let expected = "tobor";
    assert_eq!(output, expected);
}

#[test]
//#[ignore]
//#[cfg(feature = "grapheme")]
fn grapheme_cluster_with_pre_combined_form() {
    let input = "Würstchenstand";
    let output = reverse(input);
    let expected = "dnatsnehctsrüW";
    assert_eq!(output, expected);
}

#[test]
//#[ignore]
//#[cfg(feature = "grapheme")]
fn grapheme_clusters() {
    let input = "ผู้เขียนโปรแกรม";
    let output = reverse(input);
    let expected = "มรกแรปโนยขีเผู้";
    assert_eq!(output, expected);
}
