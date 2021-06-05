pub fn binary_search<T: PartialOrd>(val: &T, vec: &Vec<T>) -> bool {
    let mut low: usize = 0;
    let mut high: usize = vec.len();
    let mut middle: usize;
    while high - low > 0 {
        middle = (high - low)/2 + low;
        if vec[middle] < *val {
            low = middle + 1;
        } else if vec[middle] > *val {
            high = middle;
        } else {
            return true
        }
    }

    false
}

pub fn selection_sort<T: PartialOrd>(mut vec: Vec<T>) -> Vec<T> {
    let mut min;
    let mut min_idx;
    let mut idx;
    for start in 0..vec.len() {
        min = &vec[start];
        min_idx = start;
        idx = start;
        for val in &vec[start..vec.len()] {
            if val < min {
                min = val;
                min_idx = idx;
            }
        idx += 1;
        }
        vec.swap(start, min_idx)
    }
    vec
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

    #[test]
    fn binary_search_float() {
        assert_eq!(binary_search(&-300f64, &vec![-2000.0, -300.0, 0.0, 12.0, 26.0, 27.0, 28.0, 29.0, 42.0, 43.0, 44.0]), true);
    }

    #[test]
    fn binary_search_char() {
        assert_eq!(binary_search(&'O', &vec!['A', 'E', 'I', 'O', 'U']), true);
    }

    #[test]
    fn binary_search_char_fail() {
        assert_eq!(binary_search(&'7', &vec!['A', 'E', 'I', 'O', 'U']), false);
    }

    #[test]
    fn selection_sort_empty() {
        let vec: Vec<i32> = vec![];
        assert_eq!(selection_sort(vec), Vec::new());
    }

    #[test]
    fn selection_sort_single_element() {
        assert_eq!(selection_sort(vec![42]), vec![42]);
    }

    #[test]
    fn selection_sort_inverted() {
        assert_eq!(selection_sort(vec![45, 44, 43, 42, 41, 40]), vec![40, 41, 42, 43, 44, 45]);
    }

    #[test]
    fn selection_sort_with_chars() {
        assert_eq!(selection_sort(vec!['\'', 'ö', 'l', 'P', 'U', 'O', '4', '5', '8', '7', '=', ')', '(', '&', 'g', 'L', 'J']), vec!['&', '\'', '(', ')', '4', '5', '7', '8', '=', 'J', 'L', 'O', 'P', 'U', 'g', 'l', 'ö']);
    }

}
