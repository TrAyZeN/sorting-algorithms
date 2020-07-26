/*
 * Insertion Sort
 *
 * Time complexity:
 * Best:  O(n)
 * Avg:   O(n^2)
 * Worst: O(n^2)
 *
 * Space complexity: O(1)
 */
pub fn sort<T: Copy + Clone + Ord + PartialOrd>(arr: &Vec<T>) -> Vec<T> {
    let mut result = arr.clone();

    for i in 1..result.len() {
        let mut j = i;
        let v = result[i];
        while j > 0 && result[j-1] > v {
            result[j] = result[j-1];
            j -= 1;
        }
        result[j] = v;
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
    assert_eq!(sort(&vec![5, 1, 6, 4, 2, 4]), vec![1, 2, 3, 4, 5, 6]);
}
