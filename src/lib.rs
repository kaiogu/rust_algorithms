pub fn binary_search(&val: &i32, vec: &Vec<i32>) -> bool {
    let mut low: usize = 0;
    let mut high: usize = vec.len();
    let mut middle: usize;
    while high - low > 0 {
        middle = (high - low)/2 + low;
        if vec[middle] < val {
            low = middle + 1;
        } else if vec[middle] > val {
            high = middle;
        } else {
            return true
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
        assert_eq!(binary_search(&-300, &vec![-2000, -300, 0, 12, 26, 27, 28, 29, 42 ,43,44]), true);
    }
}