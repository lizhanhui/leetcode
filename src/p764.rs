struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp = vec![0; len];
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..len {
            dp[i] = dp[i - 1].min(dp[i-2]) + cost[i];
        }
        dp[len - 1].min(dp[len - 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![10, 15, 20]),
            15
        );
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}