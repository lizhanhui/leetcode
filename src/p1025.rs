struct Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        match n {
            0 | 1 => {
                return false;
            }
            2 => {
                return true;
            }
            3 => {
                return false;
            }
            _ => {}
        }

        let mut dp = vec![-1; n as usize];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 0;

        Self::play(n, &mut dp);
        dp[(n - 1) as usize] == 1
    }

    fn play(n: i32, dp: &mut [i32]) -> i32 {
        if dp[(n - 1) as usize] != -1 {
            return dp[(n - 1) as usize];
        }

        let divisors = (1..n)
            .into_iter()
            .filter(|x| n % x == 0)
            .rev()
            .collect::<Vec<_>>();
        let mut may_win = false;
        for d in divisors {
            // Next player will lose
            if Self::play(n - d, dp) == 0 {
                dp[(n - 1) as usize] = 1;
                may_win = true;
                break;
            }
        }
        if !may_win {
            dp[(n - 1) as usize] = 0;
        }
        dp[(n - 1) as usize]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        assert_eq!(super::Solution::divisor_game(2), true);
    }

    #[test]
    fn case2() {
        assert_eq!(super::Solution::divisor_game(3), false);
    }

    #[test]
    fn wa() {
        assert_eq!(super::Solution::divisor_game(8), true);
    }
}