/*
 * Shaker Sort
 *
 * Time complexity:
 * Best:  O(n)
 * Avg:   O(n^2)
 * Worst: O(n^2)
 *
 * Space complexity: O(1)
 */
pub fn sort<T: Clone + Ord + PartialOrd>(arr: &[T]) -> Vec<T> {
    let mut result = arr.to_owned();

    for i in 0..result.len() {
        let mut swapped = false;

        if i % 2 == 0 {
            for j in i / 2..result.len() - 1 - i / 2 {
                if result[j] > result[j + 1] {
                    result.swap(j, j + 1);
                    swapped = true;
                }
            }
        } else {
            for j in (i / 2 + 1..result.len() - 1 - i / 2).rev() {
                if result[j - 1] > result[j] {
                    result.swap(j, j - 1);
                    swapped = true;
                }
            }
        }

        if !swapped {
            break;
        }
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
