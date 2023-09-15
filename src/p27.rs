struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;

        loop {
            if j >= nums.len() {
                break;
            }

            if nums[j] != val {
                if j != i {
                    nums.swap(i, j);
                }
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let result = 2;
        assert_eq!(result, super::Solution::remove_element(&mut nums, val));
        assert!(&nums[..(result as usize)].iter().all(|n| *n != val));
    }

    #[test]
    fn case2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let result = 5;
        assert_eq!(result, super::Solution::remove_element(&mut nums, val));
        assert!(&nums[..(result as usize)].iter().all(|n| *n != val));
    }
}
