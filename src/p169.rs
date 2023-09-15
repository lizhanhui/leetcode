use std::collections::HashMap;

/// Given an array nums of size n, return the majority element.
/// The majority element is the element that appears more than ⌊n / 2⌋ times.
///  You may assume that the majority element always exists in the array.
struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut x = None;
        let mut count = 0;
        nums.into_iter().for_each(|n| match x {
            None => {
                x = Some(n);
                count = 1;
            }
            Some(ref mut m) => {
                if count == 0 {
                    *m = n;
                    count = 1;
                } else {
                    if *m == n {
                        count += 1;
                    } else {
                        count -= 1;
                    }
                }
            }
        });
        x.unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let nums = vec![3, 2, 3];
        let actual = super::Solution::majority_element(nums);
        let expected = 3;
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let actual = super::Solution::majority_element(nums);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
