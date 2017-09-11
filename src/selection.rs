/// time complexity -> O(n^2)
/// unstable
// TODO: make it in-place
pub fn sort<T: PartialOrd>(vec: &mut Vec<T>) -> Vec<T> {
    let len = vec.len();
    let mut xs = Vec::with_capacity(len);
    for _ in 0..len {
        let i = find_smallest_index(&vec);
        xs.push(vec.swap_remove(i));
    }
    xs
}

fn find_smallest_index<T: PartialOrd>(xs: &Vec<T>) -> usize {
    let mut smallest = 0;
    for (i, x) in xs.iter().enumerate() {
        if *x < xs[smallest] {
            smallest = i;
        }
    }
    smallest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_vec() {
        let mut xs: Vec<i32> = vec![];
        let e: Vec<i32> = vec![];
        assert_eq!(e, sort(&mut xs));
    }
    #[test]
    fn sort_vec_with_one_item() {
        let mut xs = vec![0];
        let e = vec![0];
        assert_eq!(e, sort(&mut xs));
    }
    #[test]
    fn sort_vec_with_two_items() {
        let mut xs = vec![1, 0];
        let e = vec![0, 1];
        assert_eq!(e, sort(&mut xs));
    }
    #[test]
    fn sort_vec_with_multiple_items() {
        let mut xs = vec![1, 0, 5, 3, 2, 4];
        let e = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(e, sort(&mut xs));
    }
    #[test]
    fn sort_vec_with_multiple_items_with_duplicates() {
        let mut xs = vec![1, 0, 5, 4, 3, 2, 4, 0];
        let e = vec![0, 0, 1, 2, 3, 4, 4, 5];
        assert_eq!(e, sort(&mut xs));
    }

    #[test]
    fn find_smallest_index_from_empty_vec() {
        let vec: Vec<i32> = vec![];
        assert_eq!(0, find_smallest_index(&vec));
    }
    #[test]
    fn find_smallest_index_from_vec_with_one_item() {
        assert_eq!(0, find_smallest_index(&vec![1]));
    }
    #[test]
    fn find_smallest_index_from_vec_with_two_items() {
        assert_eq!(1, find_smallest_index(&vec![2, 1]));
    }
    #[test]
    fn find_smallest_index_from_vec_with_smallest_in_middle() {
        assert_eq!(1, find_smallest_index(&vec![2, 1, 3]));
    }
    #[test]
    fn find_smallest_index_from_vec_with_duplicates() {
        assert_eq!(2, find_smallest_index(&vec![2, 3, 1, 3, 1]));
    }
}
