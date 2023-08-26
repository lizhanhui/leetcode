struct Solution;

impl Solution {
    /// 1 <= candidates.length <= 30
    /// 2 <= candidates[i] <= 40
    /// All elements of candidates are distinct.
    /// 1 <= target <= 40
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        let mut result = vec![];

        let mut search = vec![];
        for i in candidates.iter() {
            if *i < target {
                search.push(vec![*i]);
            } else if *i == target {
                result.push(vec![*i]);
            }
        }

        // Brute force searching
        loop {
            if search.is_empty() {
                break;
            }

            if let Some(current) = search.pop() {
                let s = current.iter().map(|i| *i).sum::<i32>();
                for i in candidates.iter() {
                    if s + *i > target {
                        break;
                    }
                    let mut opt = current.clone();
                    let pos = opt.iter().position(|v| *v >= *i);
                    match pos {
                        Some(pos) => {
                            opt.insert(pos, *i);
                        }
                        None => {
                            opt.push(*i);
                        }
                    }
                    if s + *i < target {
                        search.push(opt);
                    } else if s + *i == target {
                        if result.iter().find(|path| **path == opt).is_none() {
                            result.push(opt);
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
        let candidates = vec![2, 3, 6, 7];
        let target = 7;

        let result = super::Solution::combination_sum(candidates, target);
        accept!(result, vec![vec![2, 2, 3], vec![7]]);
    }

    fn test_case2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let result = super::Solution::combination_sum(candidates, target);
        accept!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    fn test_case3() {
        let candidates = vec![2];
        let target = 1;
        let result = super::Solution::combination_sum(candidates, target);
        accept!(result, Vec::<Vec<i32>>::new());
    }
}
