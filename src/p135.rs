struct Solution;

impl Solution {
    fn left(ratings: &[i32], res: &mut Vec<i32>) {
        let len = ratings.len();
        for i in 1..len {
            if ratings[i] > ratings[i - 1] {
                res[i] = res[i].max(res[i - 1] + 1);
            }
        }
    }

    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        let mut res = Vec::with_capacity(len);
        res.resize(len, 1);

        Self::left(&ratings, &mut res);
        let rev = ratings.iter().cloned().rev().collect::<Vec<_>>();
        let mut res = res.into_iter().rev().collect();
        Self::left(&rev, &mut res);
        dbg!(res).iter().sum()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let ratings = vec![1, 0, 2];
        let expected = 5;
        assert_eq!(super::Solution::candy(ratings), expected);
    }

    #[test]
    fn case2() {
        let ratings = vec![1, 2, 2];
        let expected = 4;
        assert_eq!(super::Solution::candy(ratings), expected);
    }

    #[test]
    fn case3() {
        let ratings = vec![1, 2, 2, 1];
        let expected = 6;
        assert_eq!(super::Solution::candy(ratings), expected);
    }

    #[test]
    fn wa1() {
        let ratings = vec![1, 2, 87, 87, 87, 2, 1];
        let expected = 13;
        assert_eq!(expected, super::Solution::candy(ratings));
    }

    #[test]
    fn wa2() {
        // 1, 2, 3, 4, 1
        let ratings = vec![1, 3, 4, 5, 2];
        let expected = 11;
        assert_eq!(expected, super::Solution::candy(ratings));
    }
}
