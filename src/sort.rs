use std::cmp::Ordering;

pub fn selection_sort<T: Ord>(mut vec: Vec<T>) -> Vec<T> {
    let mut min: &T;
    let mut min_idx: usize;
    let mut idx: usize;
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

pub fn quick_sort<T: Ord + Copy>(vec: Vec<T>) -> Vec<T> {
    let size = vec.len();
    match size {
        0 => vec,
        1 => vec,
        _ => {
            let pivot = &vec[size / 2];
            let mut less = Vec::new();
            let mut equal = Vec::new();
            let mut greater = Vec::new();
            for &e in vec.iter() {
                match &e.cmp(pivot) {
                    Ordering::Less => less.push(e),
                    Ordering::Equal => equal.push(e),
                    Ordering::Greater => greater.push(e),
                }
            }
            let mut result = Vec::new();
            result.append(&mut quick_sort(less));
            result.append(&mut equal);
            result.append(&mut quick_sort(greater));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(
            selection_sort(vec![45, 44, 43, 42, 41, 40]),
            vec![40, 41, 42, 43, 44, 45]
        );
    }

    #[test]
    fn selection_sort_with_chars() {
        assert_eq!(
            selection_sort(vec![
                '\'', 'ö', 'l', 'P', 'O', '7', '4', '=', '(', '&', 'g', 'L', 'J'
            ]),
            vec!['&', '\'', '(', '4', '7', '=', 'J', 'L', 'O', 'P', 'g', 'l', 'ö']
        );
    }

    #[test]
    fn quick_sort_empty() {
        let vec: Vec<i32> = vec![];
        assert_eq!(quick_sort(vec), Vec::new());
    }

    #[test]
    fn quick_sort_single_element() {
        assert_eq!(quick_sort(vec![42]), vec![42]);
    }

    #[test]
    fn quick_sort_two_sorted_elements() {
        assert_eq!(quick_sort(vec![0, 42]), vec![0, 42]);
    }

    #[test]
    fn quick_sort_two_reversed_elements() {
        assert_eq!(quick_sort(vec![42, 0]), vec![0, 42]);
    }

    #[test]
    fn quick_sort_inverted() {
        assert_eq!(
            quick_sort(vec![45, 44, 43, 42, 41, 40]),
            vec![40, 41, 42, 43, 44, 45]
        );
    }

    #[test]
    fn quick_sort_with_chars() {
        assert_eq!(
            selection_sort(vec![
                '\'', 'ö', 'l', 'P', 'O', '7', '4', '=', '(', '&', 'g', 'L', 'J'
            ]),
            vec!['&', '\'', '(', '4', '7', '=', 'J', 'L', 'O', 'P', 'g', 'l', 'ö']
        );
    }
}
