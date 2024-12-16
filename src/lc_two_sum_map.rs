use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let x = target - nums[i];
        if map.contains_key(&x) {
            return vec![map[&x], i as i32];
        }
        map.insert(nums[i], i as i32);
    }
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
    #[test]
    fn ex2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
    #[test]
    fn ex3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
    #[test]
    fn ex4() {
        assert_eq!(two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]);
    }
}
