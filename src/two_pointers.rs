#![allow(dead_code)]

// [125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome)
mod p125 {

    pub struct Solution;

    impl Solution {
        pub fn is_palindrome(s: &str) -> bool {
            let s = s
                .as_bytes()
                .iter()
                .filter(|c| c.is_ascii_alphanumeric())
                .map(|c| c.to_ascii_lowercase())
                .collect::<Vec<_>>();

            if s.is_empty() {
                return true;
            }

            let mut i = 0;
            let mut j = s.len() - 1;

            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }

            true
        }
    }

    #[cfg(test)]
    mod test {
        use crate::two_pointers::p125::Solution;

        #[test]
        fn example_i() {
            assert!(Solution::is_palindrome("A man, a plan, a canal: Panama"));
        }

        #[test]
        fn example_ii() {
            assert!(!Solution::is_palindrome("raceacar"));
        }

        #[test]
        fn example_iii() {
            assert!(Solution::is_palindrome(" "));
        }

        #[test]
        fn example_iv() {
            assert!(!Solution::is_palindrome("0P"));
        }
    }
}

// [15. 3Sum](https://leetcode.com/problems/3sum)
mod p15 {

    pub struct Solution;

    impl Solution {
        pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result = vec![];
            let mut cache = std::collections::HashMap::<i32, Vec<(i32, i32)>>::new();

            for i in 0..nums.len() {
                let first = nums[i];
                let remain = 0 - first;

                let two_sum = if let Some(ts) = cache.get(&first) {
                    ts.clone()
                } else {
                    let ts = Self::two_sum(&nums[i + 1..], remain);

                    cache.insert(first, ts.clone());
                    ts
                };

                for &(second, third) in &two_sum {
                    let mut r = vec![first, second, third];

                    r.sort();
                    result.push(r);
                }
            }

            result.sort();
            result.dedup();

            result
        }

        pub fn two_sum(nums: &[i32], sum: i32) -> Vec<(i32, i32)> {
            let mut result = vec![];
            let mut cache = std::collections::HashSet::<i32>::new();

            for i in 0..nums.len() {
                let first = nums[i];
                let remain = sum - first;

                if let Some(&second) = cache.get(&remain) {
                    result.push((first, second));
                } else {
                    cache.insert(first);
                }
            }

            result
        }
    }

    #[cfg(test)]
    mod test {
        use crate::two_pointers::p15::Solution;

        #[test]
        fn example_i() {
            let nums = vec![-1, 0, 1, 2, -1, -4];
            let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

            assert_eq!(expected, Solution::three_sum(nums));
        }

        #[test]
        fn example_ii() {
            let nums = vec![0, 1, 1];

            assert!(Solution::three_sum(nums).is_empty());
        }

        #[test]
        fn example_iii() {
            let nums = vec![0, 0, 0];
            let expected = vec![vec![0, 0, 0]];

            assert_eq!(expected, Solution::three_sum(nums));
        }
    }
}
