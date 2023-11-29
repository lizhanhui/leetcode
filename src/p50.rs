struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // verify tests are fine
        // x.powi(n)
        if n < 0 {
            return 1_f64 / Self::my_pow(x, -n);
        }

        if n == 0 {
            return 1_f64;
        }

        let mut ans = 1_f64;
        let mut r = n;
        loop {
            ans *= x;
            r -= 1;
            if r == 0 {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let x = 2.00000;
        let n = 10;
        let ans = 1024.00000;
        assert!((ans - super::Solution::my_pow(x, n)).abs() < 1e-9);
    }
    #[test]
    fn case2() {
        let x = 2.10000;
        let n = 3;
        let ans = 9.26100;
        assert!((ans - super::Solution::my_pow(x, n)).abs() < 1e-9);
    }
    #[test]
    fn case3() {
        let x = 2.00000;
        let n = -2;
        let ans = 0.25000;
        assert!((ans - super::Solution::my_pow(x, n)).abs() < 1e-9);
    }
}
