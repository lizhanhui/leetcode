struct Solution;

///
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let mut s = 0;

        let mut m = std::collections::HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X', 10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);

        let len = chars.len();
        let mut i = 0;
        loop {
            if i > len - 1 {
                break;
            }

            let c = chars[i];
            if i < len - 1 {
                let n = chars[i + 1];
                if m[&c] < m[&n] {
                    s += m[&n] - m[&c];
                    i += 2;
                    continue;
                } else {
                    s += m[&c];
                }
            } else {
                s += m[&c];
            }
            
            i += 1;
        }
        s
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        let mut s = "III";
        let expected = 3;
        assert_eq!(super::Solution::roman_to_int(s.to_string()), expected);
    }

    #[test]
    fn case2() {
        let mut s = "LVIII";
        let expected = 58;
        assert_eq!(super::Solution::roman_to_int(s.to_string()), expected);
    }

    #[test]
    fn case3() {
        let mut s = "MCMXCIV";
        let expected = 1994;
        assert_eq!(super::Solution::roman_to_int(s.to_string()), expected);
    }
}
