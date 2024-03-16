//! Code for computing an upper bound for BTreeMapRange for a string prefix.
//!
//! While this crate doesn't need std, it does need alloc.
//!
//! Taken from <https://www.thecodedmessage.com/posts/prefix-ranges/> (by Jimmy Hartzell)
//! Extended with documentation.
//!
//! Licensed under: MIT

#![no_std]

extern crate alloc;
use alloc::collections::BTreeSet;
use alloc::{format, string::String};

/// Find the key to use as an upper bound for range query
///
/// ```
/// # use std::collections::BTreeMap;
/// # use std::ops::Bound;
/// let mut data: BTreeMap<String, i32> = BTreeMap::new();
/// data.insert("bbabb".to_owned(), 1);
/// data.insert("bbbcc".to_owned(), 2);
/// data.insert("bbbab".to_owned(), 3);
/// data.insert("bbb".to_owned(), 4);
/// data.insert("bbc".to_owned(), 5);
///
/// let lower = Bound::Included("bbb".to_owned());
/// let upper = Bound::Excluded(prefix_range::upper_bound_from_prefix("bbb").unwrap());
/// let in_prefix: Vec<_> = data.range((lower, upper)).map(|(_, v)| *v).collect();
/// assert_eq!(in_prefix, vec![4, 3, 2]);
/// ```
pub fn upper_bound_from_prefix(prefix: &str) -> Option<String> {
    for i in (0..prefix.len()).rev() {
        if let Some(last_char_str) = prefix.get(i..) {
            let rest_of_prefix = {
                debug_assert!(prefix.is_char_boundary(i));
                &prefix[0..i]
            };

            let last_char = last_char_str
                .chars()
                .next()
                .expect("last_char_str will contain at least one char");
            let Some(last_char_incr) = (last_char..=char::MAX).nth(1) else {
                // Last character is highest possible code point.
                // Go to second-to-last character instead.
                continue;
            };

            let new_string = format!("{rest_of_prefix}{last_char_incr}");

            return Some(new_string);
        }
    }

    None
}

/// Create a new BTreeSet that contains all the keys with the given prefix
pub fn prefixed_set(mut set: BTreeSet<String>, prefix: &str) -> BTreeSet<String> {
    let mut set = set.split_off(prefix);

    if let Some(not_in_prefix) = upper_bound_from_prefix(prefix) {
        set.split_off(&not_in_prefix);
    }

    set
}

#[cfg(test)]
mod tests {

    use alloc::string::ToString;

    use super::*;

    #[test]
    fn it_works() {
        let set = {
            let mut set = BTreeSet::new();
            set.insert("Hi".to_string());
            set.insert("Hey".to_string());
            set.insert("Hello".to_string());
            set.insert("heyyy".to_string());
            set.insert("".to_string());
            set.insert("H".to_string());
            set
        };
        let set = prefixed_set(set, "H");
        assert_eq!(set.len(), 4);
        assert!(!set.contains("heyyy"));
    }

    #[test]
    fn maxicode() {
        let set = {
            let mut set = BTreeSet::new();
            set.insert("Hi".to_string());
            set.insert("Hey".to_string());
            set.insert("Hello".to_string());
            set.insert("heyyy".to_string());
            set.insert("H\u{10FFFF}eyyy".to_string());
            set.insert("H\u{10FFFF}".to_string());
            set.insert("I".to_string());
            set.insert("".to_string());
            set.insert("H".to_string());
            set
        };
        let set = prefixed_set(set, "H\u{10FFFF}");
        assert_eq!(set.len(), 2);
        assert!(!set.contains("I"));
    }
}
