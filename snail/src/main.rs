#[derive(PartialEq, Debug)]
enum Directions {
    Left,
    Down,
    Right,
    Up,
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 0 {
        return Vec::new();
    }
    let mut result: Vec<i32> = vec![];
    let rows_len = matrix.len();
    let columns_len = matrix[0].len();
    let size = rows_len * columns_len;

    if size <= rows_len {
        return matrix[0].to_vec();
    }

    let mut direction: Directions = Directions::Right;
    let mut row_index = 0;
    let mut column_index = 0;
    let mut row_max = rows_len as usize;
    let mut column_max = columns_len as usize;
    let mut row_min = 0;
    let mut column_min = 0;
    for _ in 0..size {
        if result.len() >= size {
            break;
        }
        if direction == Directions::Right {
            if result.len() >= size {
                break;
            }
            if column_index + 1 < column_max {
                result.push(matrix[row_index][column_index]);
                column_index = column_index + 1;
            } else {
                result.push(matrix[row_index][column_index]);
                row_min = row_min + 1;
                row_index = row_index + 1;
                direction = Directions::Down;
            }
        }
        if direction == Directions::Down {
            if result.len() >= size {
                break;
            }
            if row_index + 1 < row_max {
                result.push(matrix[row_index][column_index]);
                row_index = row_index + 1;
            } else {
                result.push(matrix[row_index][column_index]);
                column_index = column_index - 1;
                column_max = column_max - 1;
                direction = Directions::Left;
            }
        }
        if direction == Directions::Left {
            if result.len() >= size {
                break;
            }
            if column_index as i8 - 1 >= column_min {
                result.push(matrix[row_index][column_index]);
                column_index = column_index - 1;
            } else {
                result.push(matrix[row_index][column_index]);
                row_index = row_index - 1;
                row_max = row_max - 1;
                direction = Directions::Up;
            }
        }
        if direction == Directions::Up {
            if result.len() >= size {
                break;
            }
            if row_index as i8 - 1 >= row_min {
                result.push(matrix[row_index][column_index]);
                row_index = row_index - 1;
            } else {
                result.push(matrix[row_index][column_index]);
                column_index = column_index + 1;
                column_min = column_min + 1;
                direction = Directions::Right;
            }
        }
    }
    return result;
}

fn main() {
    let square = &[
        vec![1, 2, 3, 4],
        vec![12, 13, 14, 5],
        vec![11, 16, 15, 6],
        vec![10, 9, 8, 7],
    ];
    println!("result: {:?}", (snail(square)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }
    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }
    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }
    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
    #[test]
    fn sample_test5() {
        let square = &[vec![544, 246], vec![109, 805]];
        let expected = vec![544, 246, 805, 109];
        assert_eq!(snail(square), expected);
    }
}
