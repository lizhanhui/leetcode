struct Solution {}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = std::collections::BTreeMap::new();
        nums.iter().enumerate().rev().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, v)| {
                m.entry(i + j)
                    .and_modify(|entry: &mut Vec<i32>| entry.push(*v))
                    .or_insert(vec![*v]);
            })
        });
        m.into_iter()
            .map(|(_k, v)| v.into_iter())
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(nums, vec![1,4,2,7,5,3,8,6,9]);
    }
}
