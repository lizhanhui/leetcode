struct Solution;


impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = [0; 3];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;

        match n {
            0..=2 => {
                return dp[n as usize];
            }
            _ => {}
        }

        for i in 3..=(n as usize) {
            let next = dp[0] + dp[1] + dp[2];
            dp[0] = dp[1];
            dp[1] = dp[2];
            dp[2] = next;
        }
        dp[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribonacci() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}