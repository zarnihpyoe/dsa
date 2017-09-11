extern crate dsa;

use dsa::selection;

#[test]
fn selection_sort_integers_with_duplicates() {
    let mut xs = vec![5, 5, 4, 2, 0, 3, 1, 2];
    let e = vec![0, 1, 2, 2, 3, 4, 5, 5];
    assert_eq!(e, selection::sort(&mut xs));
}
#[test]
fn selection_sort_chars_with_duplicates() {
    let mut xs = vec!['f', 'f', 'e', 'c', 'a', 'd', 'b', 'c'];
    let e = vec!['a', 'b', 'c', 'c', 'd', 'e', 'f', 'f'];
    assert_eq!(e, selection::sort(&mut xs));
}
#[test]
fn selection_sort_strs_lowercase_only() {
    let mut xs = vec!["me", "you", "i", "they", "them", "the"];
    let e = vec!["i", "me", "the", "them", "they", "you"];
    assert_eq!(e, selection::sort(&mut xs));
}
#[test]
fn selection_sort_strs_mix_lowercase_uppercase() {
    let mut xs = vec!["me", "You", "i", "theY", "them", "the"];
    let e = vec!["You", "i", "me", "the", "theY", "them"];
    assert_eq!(e, selection::sort(&mut xs));
}
