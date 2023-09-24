struct Solution;

/// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 < numbers.length.
/// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
/// The tests are generated such that there is exactly one solution. You may not use the same element twice.
/// Your solution must use only constant extra space.
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![];

        let n = numbers.len();
        for i in 0..n {
            let r = target - numbers[i];
            if let Ok(idx) = &numbers[(i + 1)..n].binary_search(&r) {
                res.push(i as i32 + 1);
                res.push(*idx as i32 + 1 + i as i32 + 1);
                return res;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![1, 2];
        assert_eq!(expected, super::Solution::two_sum(numbers, target));
    }

    #[test]
    fn case2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let expected = vec![1, 3];
        assert_eq!(expected, super::Solution::two_sum(numbers, target));
    }

    #[test]
    fn case3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let expected = vec![1, 2];
        assert_eq!(expected, super::Solution::two_sum(numbers, target));
    }

    #[test]
    fn wa() {
        let numbers = vec![5, 25, 75];
        let target = 100;
        let expected = vec![2, 3];
        assert_eq!(expected, super::Solution::two_sum(numbers, target));
    }
}
