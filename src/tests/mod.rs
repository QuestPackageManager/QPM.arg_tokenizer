use std::vec;

use crate::arg::{self, Expression};

#[test]

fn replace_singles() {
    let text: String = "$0 $1 $2".to_string();
    let arg_strings = vec!["Oh", "my", "god"];

    let expression = Expression::parse(&text);
    let result = expression
        .replace(arg_strings.as_slice())
        .expect("Failed to replace argument");

    assert_eq!(result, arg_strings.join(" "));
}

#[test]
fn replace_range() {
    let text: String = "$0:2".to_string();
    let arg_strings = vec!["Oh", "my", "god"];

    let expression = Expression::parse(&text);
    let result = expression
        .replace(arg_strings.as_slice())
        .expect("Failed to replace argument");

    assert_eq!(result, arg_strings.join(" "));
}


#[test]
fn replace_range_reverse() {
    let text: String = "$2:0".to_string();
    let mut arg_strings = vec!["Oh", "my", "god"];

    arg_strings.reverse();

    let expression = Expression::parse(&text);
    let result = expression
        .replace(arg_strings.as_slice())
        .expect("Failed to replace argument");

    arg_strings.reverse();

    assert_eq!(result, arg_strings.join(" "));
}

#[test]
fn replace_range_end() {
    let text: String = "$0:".to_string();
    let arg_strings = vec!["Oh", "my", "god"];

    let expression = Expression::parse(&text);
    let result = expression
        .replace(arg_strings.as_slice())
        .expect("Failed to replace argument");

    assert_eq!(result, arg_strings.join(" "));
}


#[test]
fn replace_all() {
    let text: String = "$0 $1:2 $3:".to_string();
    let arg_strings = vec!["Oh", "my", "god", "it works so well!"];

    let expression = Expression::parse(&text);
    let result = expression
        .replace(arg_strings.as_slice())
        .expect("Failed to replace argument");

    assert_eq!(result, arg_strings.join(" "));
}
