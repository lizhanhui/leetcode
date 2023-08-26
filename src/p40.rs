struct Solution;

impl Solution {
    fn subtract(all: &Vec<i32>, used: &Vec<i32>) -> Vec<i32> {
        let mut flag = vec![];
        flag.resize(used.len(), false);
        let mut diff = vec![];
        for e in all {
            let mut found_used = false;
            for (pos, n) in used.iter().enumerate() {
                if e == n && !flag[pos] {
                    flag[pos] = true;
                    found_used = true;
                    break;
                }
            }
            if !found_used {
                diff.push(*e);
            }
        }
        diff
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut result = vec![];

        let mut search = vec![];

        // Initiate search
        for n in candidates.iter() {
            if *n > target {
                break;
            }

            let opt = vec![*n];

            if *n == target {
                if !result.contains(&opt) {
                    result.push(opt);
                }
                continue;
            }

            if !search.contains(&opt) {
                search.push(opt);
            }
        }

        loop {
            if search.is_empty() {
                break;
            }

            if let Some(current) = search.pop() {
                let diff = Self::subtract(&candidates, &current);
                if diff.is_empty() {
                    continue;
                }

                let s = current.iter().sum::<i32>();
                for e in diff {
                    if s + e > target {
                        break;
                    }

                    let mut opt = current.clone();
                    let pos = opt.iter().position(|v| *v >= e);
                    match pos {
                        Some(pos) => opt.insert(pos, e),
                        None => {
                            opt.push(e);
                        }
                    }

                    if s + e == target {
                        if !result.contains(&opt) {
                            result.push(opt);
                        }
                    } else {
                        if !search.contains(&opt) {
                            search.push(opt);
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {

    macro_rules! accept {
        ($left: expr, $right: expr) => {
            for e in $left.iter() {
                if $right.iter().find(|e1| *e1 == e).is_none() {
                    panic!("{e:?} is not found in the right");
                }
            }

            for e in $right.iter() {
                if $left.iter().find(|e1| *e1 == e).is_none() {
                    panic!("{e:?} is not found in the left");
                }
            }
        };
    }

    #[test]
    fn test_case1() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let results = super::Solution::combination_sum2(candidates, target);
        accept!(
            results,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_case2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let results = super::Solution::combination_sum2(candidates, target);
        accept!(results, vec![vec![1, 2, 2], vec![5]]);
    }
}
