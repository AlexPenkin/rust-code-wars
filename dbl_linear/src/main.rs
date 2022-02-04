fn dbl_linear(n: u32) -> u32 {
    let mut vector: Vec<u32> = vec![1];
    let (mut first_index, mut second_index, mut equal) = (0, 0, 0);

    while first_index + second_index < n + equal {
        let first_val = 2 * vector[first_index as usize] + 1;
        let second_val = 3 * vector[second_index as usize] + 1;

        if first_val < second_val {
            vector.push(first_val);
            first_index += 1;
        } else if first_val > second_val {
            vector.push(second_val);
            second_index += 1;
        } else {
            vector.push(first_val);
            first_index += 1;
            second_index += 1;
            equal += 1;
        }
    }
    vector.pop().unwrap()
}

fn main() {
    println! {"{}", dbl_linear(999999)}
}

#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }
    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
        testing(999999, 54381285);
        testing(10000000, 1031926810);
    }
}
