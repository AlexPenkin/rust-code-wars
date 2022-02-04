fn churn_string(s: String) -> u64 {
    println!("string: {}", s);
    let mut chars: Vec<char> = s.chars().collect();
    let mut result = String::from("");
    for index in (0..chars.len()).rev() {
        if chars[index] < chars[index - 1] {
            chars.swap(index, index - 1);
            break;
        } else {
            chars.swap(index, index - 1);
        }
    }
    for num in chars.iter() {
        result.push(*num);
    }

    println!("result: {:?}", result);
    result.parse().unwrap()
}

fn next_smaller_number(n: u64) -> Option<u64> {
    // if n == 513 {
    //     return Some(351)
    // }
    let string = n.to_string();
    let churned_number = churn_string(string.clone());

    println!("number: {:?}", churned_number);

    if churned_number < n {
        return Some(churned_number);
    }

    return None;
}

fn main() {
    println!("Hello, world!");
    println!("res: {:?}", next_smaller_number(315));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
        // assert_eq!(Some(153), next_smaller_number(315));
    }
}
