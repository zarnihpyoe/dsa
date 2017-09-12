// extern crate rand;

// use self::rand::Rng;

// average case time complexity -> O(nlogn)
// worst case time complexity -> O(n^2)
// unstable | inplace

pub fn sort<T: PartialOrd + Copy>(xs: &mut Vec<T>) {
    let len = xs.len();
    if len < 2 {
        return;
    }
    sub_routine(xs, 0, len - 1);
}

fn sub_routine<T: PartialOrd + Copy>(xs: &mut Vec<T>, l: usize, h: usize) {
    // choose a random pivot
    // let mut rng = rand::thread_rng();
    // let pivot = xs[rng.gen_range(l, h+1)];
    let pivot = xs[h];

    // left and right pointers
    let mut i = l;
    let mut j = h;

    // partitioning
    while i <= j {
        while xs[i] < pivot {
            i += 1;
        }
        while xs[j] > pivot {
            j -= 1;
        }
        if i <= j {
            xs.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    // left recursion
    if l < j {
        sub_routine(xs, l, j);
    }
    // right recursion
    if h > i {
        sub_routine(xs, i, h);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_vec() {
        let mut xs = vec![];
        sort(&mut xs);
        let e: Vec<i32> = vec![];
        assert_eq!(e, xs);
    }
    #[test]
    fn sort_vec_with_one_item() {
        let mut xs = vec![0];
        sort(&mut xs);
        let e = vec![0];
        assert_eq!(e, xs);
    }
    #[test]
    fn sort_vec_with_two_items() {
        let mut xs = vec![1, 0];
        sort(&mut xs);
        let e = vec![0, 1];
        assert_eq!(e, xs);
    }
    #[test]
    fn sort_vec_with_three_items() {
        let mut xs = vec![2, 0, 1];
        sort(&mut xs);
        let e = vec![0, 1, 2];
        assert_eq!(e, xs);
    }
    #[test]
    fn sort_vec_with_multiple_items() {
        let mut xs = vec![5, 2, 0, 1, 4, 3];
        sort(&mut xs);
        let e = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(e, xs);
    }
    #[test]
    fn sort_vec_with_multiple_items_with_duplicates() {
        let mut xs = vec![41, 32, 4, 56, 7, 45, 73, 60, 89, 8, 92];
        sort(&mut xs);
        let e = vec![4, 7, 8, 32, 41, 45, 56, 60, 73, 89, 92];
        assert_eq!(e, xs);
    }
}
