struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0;
        let mut j = 1;
        loop {
            if j >= nums.len() {
                break;
            }

            if nums[i] != nums[j] {
                if i + 1 != j {
                    nums.swap(i + 1, j);
                }
                i += 1;
            }
            j += 1;
        }

        i as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let mut nums = vec![1, 1, 2];
        let expected = 2;
        assert_eq!(expected, super::Solution::remove_duplicates(&mut nums));
    }

    #[test]
    fn case2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected = 5;
        assert_eq!(expected, super::Solution::remove_duplicates(&mut nums));
    }
}
