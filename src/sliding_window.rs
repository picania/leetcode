#![allow(dead_code)]

// [1876. Substrings of Size Three with Distinct Characters](https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters)
mod p1876 {

    pub struct Solution;

    impl Solution {
        pub fn count_good_substrings(s: String) -> i32 {
            if s.len() < 3 {
                return 0;
            }

            const K: usize = 3;
            let len = s.len() - K;
            let mut counter = 0;
            
            for i in 0..=len {
                let slice = s[i..(i+3)].as_bytes();

                if Self::unique(slice[0], slice[1], slice[2]) {
                    counter += 1;
                }
            }
            
            counter
        }

        fn unique(a: u8, b: u8, c: u8) -> bool {
            a != b && a != c && b != c
        }
    }

    #[cfg(test)]
    mod test {
        use crate::sliding_window::p1876::Solution;

        #[test]
        fn check_unique() {
            assert!(Solution::unique(b'a', b'b', b'c'));

            assert!(!Solution::unique(b'a', b'a', b'c'));
            assert!(!Solution::unique(b'a', b'b', b'a'));

            assert!(!Solution::unique(b'a', b'b', b'b'));
        }

        #[test]
        fn example_1() {
            assert_eq!(1, Solution::count_good_substrings("xyzzaz".into()));
        }

        #[test]
        fn example_2() {
            assert_eq!(4, Solution::count_good_substrings("aababcabc".into()));
        }
    }

}
