
use std::cmp;

fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let mut max = Box::new(pyramid.to_vec());
    for y in (0..pyramid.len() - 1).rev() {
        for x in 0..pyramid[y].len() {
            max[y][x] = pyramid[y][x] + cmp::max(max[y + 1][x], max[y + 1][x + 1]);
        }
    }
    return max[0][0];
}

fn main() {
    let small = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
    println!("Result is: {:?}", longest_slide_down(&small));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let small = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        assert_eq!(
            longest_slide_down(&small),
            23,
            "It should work for small pyramids"
        );
    }
    #[test]
    fn test_small2() {
        let small2 = vec![
            vec![11],
            vec![12, 13],
            vec![14, 15, 16],
            vec![90, 18, 19, 20],
        ];
        assert_eq!(
            longest_slide_down(&small2),
            127,
            "It should work for small pyramids 2"
        );
    }
    #[test]
    fn test_medium() {
        let medium = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 4, 82, 47, 65],
            vec![19, 1, 23, 75, 3, 34],
            vec![88, 2, 77, 73, 7, 63, 67],
            vec![99, 65, 4, 28, 6, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
        ];
        assert_eq!(
            longest_slide_down(&medium),
            1074,
            "It should work for medium pyramids"
        );
    }
}
