struct Solution;

/// Given a string s consisting of words and spaces,
/// return the length of the last word in the string.
/// A word is a maximal substring consisting of non-space
///  characters only.
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut cnt = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                if 0 == cnt {
                    continue;
                } else {
                    break;
                }
            } else {
                cnt += 1;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let s = "Hello World";
        let expected = 5;
        assert_eq!(
            expected,
            super::Solution::length_of_last_word(s.to_string())
        );
    }

    #[test]
    fn case2() {
        let s = "   fly me   to   the moon  ";
        let expected = 4;
        assert_eq!(
            expected,
            super::Solution::length_of_last_word(s.to_string())
        );
    }

    #[test]
    fn case3() {
        let s = "luffy is still joyboy";
        let expected = 6;
        assert_eq!(
            expected,
            super::Solution::length_of_last_word(s.to_string())
        );
    }
}
