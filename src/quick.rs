// time complexity -> O(nlogn)

pub fn sort(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        return vec;
    }
    // choose a pivot
    // TODO: to choose pivot randomly
    let pivot = &vec[0];

    // partitioning
    let mut smaller = vec![];
    let mut larger = vec![];
    for i in &vec[1..] {
        match *i <= *pivot {
            true => smaller.push(*i),
            false => larger.push(*i),
        }
    }

    // combining altogether
    let mut head = sort(smaller);
    let mut tail = sort(larger);
    head.push(*pivot);
    head.append(&mut tail);
    head
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_vec() {
        let xs = vec![];
        let e: Vec<i32> = vec![];
        assert_eq!(e, sort(xs));
    }
    #[test]
    fn sort_vec_with_one_item() {
        let xs = vec![0];
        let e = vec![0];
        assert_eq!(e, sort(xs));
    }
    #[test]
    fn sort_vec_with_two_items() {
        let xs = vec![1, 0];
        let e = vec![0, 1];
        assert_eq!(e, sort(xs));
    }
    #[test]
    fn sort_vec_with_three_items() {
        let xs = vec![2, 0, 1];
        let e = vec![0, 1, 2];
        assert_eq!(e, sort(xs));
    }
    #[test]
    fn sort_vec_with_multiple_items() {
        let xs = vec![5, 2, 0, 1, 4, 3];
        let e = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(e, sort(xs));
    }
}
