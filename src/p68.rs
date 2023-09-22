struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];

        let word_cnt = words.len();

        let mut i = 0;
        let mut lines = vec![];
        loop {
            if i >= word_cnt {
                break;
            }
            let mut line = vec![];
            let mut len = 0;
            loop {
                if i >= word_cnt {
                    break;
                }

                if 0 == len {
                    line.push(&words[i]);
                    len += words[i].len();
                    i += 1;
                } else {
                    // add space separator
                    len += 1;
                    if len + words[i].len() > max_width as usize {
                        break;
                    }
                    line.push(&words[i]);
                    len += words[i].len();
                    i += 1;
                }
            }
            lines.push(line);
        }

        for i in 0..lines.len() {
            if i < lines.len() - 1 {
                let line = &lines[i];
                let c_cnt = line.iter().map(|w| w.len()).sum::<usize>();
                let wc = line.iter().count();
                let space = max_width as usize - c_cnt;
                let slots = (wc - 1).max(1);
                let r = space % slots;
                let avg = space / slots;
                let mut s = String::new();
                s.push_str(&line[0]);
                for j in 0..slots {
                    for _ in 0..avg {
                        s.push(' ');
                    }
                    if j < r {
                        s.push(' ');
                    }
                    if j + 1 < wc {
                        s.push_str(&line[j + 1]);
                    }
                }
                res.push(s);
            } else {
                // justify left
                let mut s = String::new();
                for w in &lines[i] {
                    if s.is_empty() {
                        s.push_str(w);
                    } else {
                        s.push(' ');
                        s.push_str(w);
                    }
                }
                loop {
                    if s.len() >= max_width as usize {
                        break;
                    }
                    s.push(' ');
                }
                res.push(s);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let words = vec![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let max_width = 16;
        let expected = vec!["This    is    an", "example  of text", "justification.  "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        assert_eq!(expected, super::Solution::full_justify(words, max_width));
    }

    #[test]
    fn case2() {
        let words = vec!["What", "must", "be", "acknowledgment", "shall", "be"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let max_width = 16;
        let expected = vec!["What   must   be", "acknowledgment  ", "shall be        "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        assert_eq!(expected, super::Solution::full_justify(words, max_width));
    }

    #[test]
    fn case3() {
        let words = vec![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let max_width = 20;
        let expected = vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        assert_eq!(expected, super::Solution::full_justify(words, max_width));
    }
}
