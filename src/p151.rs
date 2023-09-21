struct Solution;

/// Given an input string s, reverse the order of the words.
///
/// A word is defined as a sequence of non-space characters.
///  The words in s will be separated by at least one space.
///
/// Return a string of the words in reverse order concatenated
/// by a single space.
///
/// Note that s may contain leading or trailing spaces
/// or multiple spaces between two words. The returned string should only
/// have a single space separating the words. Do not include any extra spaces.
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = String::new();

        let chars = s.chars().rev().collect::<Vec<_>>();

        let mut i = 0;
        loop {
            if i >= chars.len() {
                break;
            }

            if chars[i] == ' ' {
                i += 1;
                continue;
            }

            let mut j = 0;
            loop {
                if i + j >= chars.len() {
                    break;
                }

                if chars[i + j] == ' ' {
                    break;
                }

                j += 1;
            }

            if !res.is_empty() {
                res.push(' ');
            }

            for k in (0..j).rev() {
                res.push(chars[i + k]);
            }
            i += j;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let s = "the sky is blue";
        let expected = "blue is sky the";
        assert_eq!(expected, &super::Solution::reverse_words(s.to_string()));
    }

    #[test]
    fn case2() {
        let s = "  hello world  ";
        let expected = "world hello";
        assert_eq!(expected, &super::Solution::reverse_words(s.to_string()));
    }
}
