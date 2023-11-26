struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        dp.resize(nums.len(), 0);
        // dp[k]: an increasing sub-sequence ends with nums[k]
        // dp[k] = max {
        //            dp[l] + 1 if nums[k] > nums[l],
        //            dp[m] if nums[k] <= nums[m]
        // }
        dp[0] = 1;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                } else {
                    dp[i] = dp[i].max(dp[j]);
                }
            }
        }
        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(4, super::Solution::length_of_lis(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(4, super::Solution::length_of_lis(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![7,7,7,7,7,7,7];
        assert_eq!(1, super::Solution::length_of_lis(nums));
    }

}
