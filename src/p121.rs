struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        // lowest price we know
        let mut l = prices[0];
        let mut max = 0;
        for i in 1..len {
            l = l.min(prices[i]);
            max = max.max(prices[i] - l);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn case2() {
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn test_wa() {
        let prices = vec![1];
        let expected = 0;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }

    #[test]
    fn test_wa2() {
        let prices = vec![1, 2];
        let expected = 1;
        assert_eq!(expected, super::Solution::max_profit(prices));
    }
}
