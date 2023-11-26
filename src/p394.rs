struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let chars = s.chars().collect::<Vec<_>>();
        Self::decode_str(&chars)
    }

    fn decode_str(s: &[char]) -> String {
        let mut ans = String::new();

        let mut repeat = 0;
        let mut i = 0;
        loop {
            match s[i] {
                'a'..='z' => {
                    ans.push(s[i]);
                    i += 1;
                }
                '0'..='9' => {
                    repeat = repeat * 10 + (s[i] as u32 - '0' as u32);
                    i += 1;
                }
                '[' => {
                    let mut occurance = 1;
                    let mut j = i + 1;
                    loop {
                        match s[j] {
                            '[' => {
                                occurance += 1;
                            }
                            ']' => {
                                occurance -= 1;
                            }
                            _ => {}
                        }
                        if 0 == occurance {
                            break;
                        }
                        j += 1;
                    }
                    let nested = Self::decode_str(&s[i + 1..j]);
                    for _ in 0..repeat {
                        ans.push_str(&nested);
                    }
                    repeat = 0;
                    i = j + 1;
                }
                ']' => {
                    i += 1;
                }
                _ => {
                    unreachable!()
                }
            }

            if i >= s.len() {
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
        let s = "3[a]2[bc]";
        let ans = "aaabcbc";
        assert_eq!(ans, &super::Solution::decode_string(s.to_string()));
    }

    #[test]
    fn case2() {
        let s = "3[a2[c]]";
        let ans = "accaccacc";
        assert_eq!(ans, &super::Solution::decode_string(s.to_string()));
    }
    #[test]
    fn case3() {
        let s = "2[abc]3[cd]ef";
        let ans = "abcabccdcdcdef";
        assert_eq!(ans, &super::Solution::decode_string(s.to_string()));
    }
    #[test]
    fn case4() {
        let s = "3[a]2[bc]";
        let ans = "aaabcbc";
        assert_eq!(ans, &super::Solution::decode_string(s.to_string()));
    }
}
