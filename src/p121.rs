struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len <= 1 {
            return 0;
        }

        let mut min = prices[0];
        let mut max = prices[len - 1];
        let mut s_p = vec![];
        let mut b_p = vec![];

        for i in 0..len - 1 {
            if prices[i] < min {
                min = prices[i];
            }
            b_p.push(min);

            if prices[len - 1 - i] > max {
                max = prices[len - 1 - i];
            }
            s_p.push(max);
        }

        s_p.reverse();

        let mut max = 0;
        for i in 0..len - 1 {
            let profit = s_p[i] - b_p[i];
            if profit > max {
                max = profit;
            }
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
