// struct SumPair {
//     first_value: i8,
//     first_index: usize,
//     second_value: i8,
//     second_index: usize,
// }

// impl SumPair {
//     fn is_earlier(&self, another_pair: &Option<SumPair>) -> bool {
//         return match another_pair {
//             None => true,
//             Some(pair) => self.second_index < pair.second_index,
//         };
//     }
//     fn is_shortest(&self) -> bool {
//         return self.second_index - self.first_index == 1;
//     }
// }

use std::collections::HashSet;

pub fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut seen = HashSet::new();
    for &i in ints {
        if seen.contains(&(s - i)) {
            return Some((s - i, i));
        }
        seen.insert(i);
    }
    None
}

#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    // assert_eq!(sum_pairs(&vals, 10), Some((13, -3)));
}
