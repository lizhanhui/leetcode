struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        // Greedy
        let mut s = 0;
        let mut prev = prices[0];
        for i in 1..prices.len() {
            let cur = prices[i];
            if cur > prev {
                s += cur - prev;
                prev = cur;
            }
            prev = cur;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 7;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn case2() {
        let prices = vec![1, 2, 3, 4, 5];
        let expected = 4;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn case3() {
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn case4() {
        let prices = vec![];
        let expected = 0;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn case5() {
        let prices = vec![1];
        let expected = 0;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn case6() {
        let prices = vec![7, 1, 2, 5, 3, 6, 4];
        let expected = 7;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

}
