use std::collections::HashMap;

fn main() {
}


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::<i32, usize>::new();

    for i in 0..nums.len() {
        let complement = target - nums[i];
        if let Some(comp) = hash.get(&complement).map(|&x| x as i32) {
            return vec![comp, i as i32];
        }
        hash.insert(nums[i], i);
    }
    vec![]
}

#[test]
fn test_1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
}

#[test]
fn test_2() {
    assert_eq!(two_sum(vec![3, 2, 4], 6), [1, 2]);
}

#[test]
fn test_3() {
    assert_eq!(two_sum(vec![3, 3], 6), [0, 1]);
}
