/*
 * Merge Sort
 *
 * Time complexity:
 * Best:  O(nlog(n))
 * Avg:   O(nlog(n))
 * Worst: O(nlog(n))
 *
 * Space complexity: O(n)
 */
pub fn sort<T: Copy + Ord + PartialOrd>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_owned();
    }

    let left = sort(&arr[..arr.len() / 2]);
    let right = sort(&arr[arr.len() / 2..]);

    merge(&left, &right)
}

fn merge<T: Copy + Ord + PartialOrd>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::new();

    let mut j = 0;
    for e in left {
        while j < right.len() && right[j] <= *e {
            result.push(right[j]);
            j += 1;
        }

        result.push(*e);
    }

    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

#[test]
fn flat() {
    let v = vec![1; 6];
    assert_eq!(sort(&v), v);
}

#[test]
fn reversed() {
    assert_eq!(sort(&vec![6, 5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn shuffled() {
    assert_eq!(sort(&vec![5, 1, 6, 4, 2, 3]), vec![1, 2, 3, 4, 5, 6]);
}
