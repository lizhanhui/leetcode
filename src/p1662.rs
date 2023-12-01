struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut lhs = word1
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .flatten();
        let mut rhs = word2
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .flatten();

        loop {
            match (lhs.next(), rhs.next()) {
                (Some(l), Some(r)) => {
                    if l != r {
                        return false;
                    }
                }
                (None, None) => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let word1 = vec!["ab".to_string(), "c".to_string()];
        let word2 = vec!["a".to_string(), "bc".to_string()];
        assert_eq!(true, super::Solution::array_strings_are_equal(word1, word2))
    }

    #[test]
    fn case2() {
        let word1 = vec!["a".to_string(), "cb".to_string()];
        let word2 = vec!["ab".to_string(), "c".to_string()];
        assert_eq!(false, super::Solution::array_strings_are_equal(word1, word2))
    }
}
