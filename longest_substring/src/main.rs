use std::collections::HashMap;

fn main() {

}

#[test]
fn test_1() {
    assert_eq!(
        3,
        length_of_longest_substring(
            "abcabcbb"
            .to_string()
        )
    );
}

#[test]
fn test_2() {
    assert_eq!(
        1,
        length_of_longest_substring(
            "bbbbb"
            .to_string()
        )
    );
}

#[test]
fn test_3() {
    assert_eq!(
        3,
        length_of_longest_substring(
            "pwwkew"
            .to_string()
        )
    );
}

pub fn length_of_longest_substring(s: String) -> i32 {
    // Initialize a HashMap to keep track of characters and their last seen index.
    let mut char_index: HashMap<char, usize> = HashMap::new();

    // `i` is a "pointer" representing the index of the last seen duplicate char
    let mut i = 0;

    // Represents the length of the longest non-repeating substring encountered
    let mut longest = 0;

    // Traversing the string as a char iterator, getting index pointer `j` and char value `c`...
    for (j, c) in s.chars().enumerate() {

        // if the HashMap contains an index for char `c`, assign that index as `k`...
        if let Some(&k) = char_index.get(&c) {
            // then update `i` to be the max of itself and `k + 1` (in order to exclude the repeated char)
            i = i.max(k + 1);
        }

        // Set longest to the max of itself and the range between `j` and `i`, our pointers to the substring range
        longest = longest.max(j - i + 1);

        // Then add the last seen index to the HashMap
        char_index.insert(c, j);
    }

    // After enumeration, return the length of the longest substring
    longest as i32
}
