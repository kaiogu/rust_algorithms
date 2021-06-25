use std::cmp::Ordering;

pub fn binary_search<T: Ord>(val: &T, vec: &[T]) -> bool {
    let mut low: usize = 0;
    let mut high: usize = vec.len();
    let mut middle: usize;
    while high - low > 0 {
        middle = (high - low) / 2 + low;
        match vec[middle].cmp(val) {
            Ordering::Less => low = middle + 1,
            Ordering::Greater => high = middle,
            Ordering::Equal => return true,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_an_empty_vec() {
        assert_eq!(binary_search(&42, &vec![]), false);
    }

    #[test]
    fn binary_search_simple_fail() {
        assert_eq!(binary_search(&42, &vec![100]), false);
    }

    #[test]
    fn binary_search_simple_succeed() {
        assert_eq!(binary_search(&42, &vec![42]), true);
    }

    #[test]
    fn binary_search_upper() {
        assert_eq!(binary_search(&42, &vec![1, 30, 42]), true);
    }

    #[test]
    fn binary_search_lower() {
        assert_eq!(binary_search(&42, &vec![42, 43, 44]), true);
    }

    #[test]
    fn binary_search_long() {
        assert_eq!(
            binary_search(&-300, &vec![-2000, -300, 0, 12, 26, 27, 28, 29, 42, 43, 44]),
            true
        );
    }

    #[test]
    fn binary_search_char() {
        assert_eq!(binary_search(&'O', &vec!['A', 'E', 'I', 'O', 'U']), true);
    }

    #[test]
    fn binary_search_char_fail() {
        assert_eq!(binary_search(&'7', &vec!['A', 'E', 'I', 'O', 'U']), false);
    }
}
