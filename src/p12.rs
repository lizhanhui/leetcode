struct Solution;

/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
/// For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
///
/// I can be placed before V (5) and X (10) to make 4 and 9.
/// X can be placed before L (50) and C (100) to make 40 and 90.
/// C can be placed before D (500) and M (1000) to make 400 and 900.
/// Given an integer, convert it to a roman numeral.
impl Solution {
    fn append<'a>(sym: &[&'a str], r: i32, s: &mut Vec<&'a str>) {
        match r {
            1..=3 => {
                for _ in 0..r {
                    s.push(sym[0]);
                }
            }
            4 => {
                s.push(sym[1]);
            }
            5 => {
                s.push(sym[2]);
            }
            6 | 7 | 8 => {
                for _ in 0..(r - 5) {
                    s.push(sym[0]);
                }
                s.push(sym[2]);
            }
            9 => {
                s.push(sym[3]);
            }
            _ => {}
        }
    }

    pub fn int_to_roman(num: i32) -> String {
        let sym = vec![
            vec!["I", "IV", "V", "IX"],
            vec!["X", "XL", "L", "XC"],
            vec!["C", "CD", "D", "CM"],
            vec!["M"],
        ];
        let mut s = String::new();

        let mut sig = 0;
        let mut r = num;
        let mut v = vec![];
        loop {
            let r1 = r % 10;
            r = r / 10;
            let symbol = &sym[sig];
            Self::append(symbol, r1, &mut v);
            sig += 1;
            if 0 == r {
                break;
            }
        }

        for e in v.into_iter().rev() {
            s.push_str(e);
        }

        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let num = 3;
        let expected = "III";
        assert_eq!(&super::Solution::int_to_roman(num), expected);
    }

    #[test]
    fn case2() {
        let num = 58;
        let expected = "LVIII";
        assert_eq!(&super::Solution::int_to_roman(num), expected);
    }

    #[test]
    fn case3() {
        let num = 1994;
        let expected = "MCMXCIV";
        assert_eq!(&super::Solution::int_to_roman(num), expected);
    }
}
