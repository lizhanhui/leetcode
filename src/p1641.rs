struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        // 'a', 'e', 'i', 'o', 'u'
        let mut s = vec![];
        s.push(5);
        for _ in 1..n {
            let mut t = vec![];
            while let Some(x) = s.pop() {
                for i in 1..=x {
                    t.push(i)
                }
            }
            s = t;
        }
        s.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowel_strings() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
        assert_eq!(Solution::count_vowel_strings(2), 15);
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}
