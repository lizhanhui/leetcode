struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<Vec<_>>();

        let n = chars.len();
        for i in 0..(n / 2) {
            if chars[i] != chars[n - i - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let s = "A man, a plan, a canal: Panama";
        assert_eq!(true, super::Solution::is_palindrome(s.to_string()));
    }

    #[test]
    fn case2() {
        let s = "race a car";
        assert_eq!(false, super::Solution::is_palindrome(s.to_string()));
    }

    #[test]
    fn case3() {
        let s = " ";
        assert_eq!(true, super::Solution::is_palindrome(s.to_string()));
    }

    #[test]
    fn case4() {
        let s = "a";
        assert_eq!(true, super::Solution::is_palindrome(s.to_string()));
    }

    #[test]
    fn case5() {
        let s = "0P";
        assert_eq!(false, super::Solution::is_palindrome(s.to_string()));
    }
}
