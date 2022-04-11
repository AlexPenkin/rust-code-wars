fn churn_string(s: String) -> u64 {
    let mut chars: Vec<char> = s.chars().collect();
    let mut result = String::from("");
    for index in (0..chars.len()).rev() {
        if index == 0 {
            break;
        }
        if chars[index] < chars[index - 1] {
            let x: Option<usize> = Some(index - 1);
            if let Some(x) = x {
                let finded_index = chars.iter().rev().position(|&r| {
                    return r < chars[x];
                });
                match finded_index {
                    Some(i) => {
                        println!("chars len: {}, i: {}", chars.len() - 1, i);
                        let reversed_finded_index = chars.len() - 1 - i;
                        chars.swap(x, reversed_finded_index);
                        chars[x + 1..].sort_by(|a, b| b.cmp(a))
                    }
                    None => break,
                }
            }
            break;
        }
    }
    if chars[0] == '0' {
        return u64::MAX;
    }
    for num in chars.iter() {
        result.push(*num);
    }

    result.parse().unwrap()
}

fn next_smaller_number(n: u64) -> Option<u64> {
    let string = n.to_string();
    let churned_number = churn_string(string.clone());

    if churned_number < n {
        return Some(churned_number);
    }

    return None;
}

fn main() {
    println!("res: {:?}", next_smaller_number(1262347));
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
        assert_eq!(Some(153), next_smaller_number(315));
        assert_eq!(Some(2017), next_smaller_number(2071));
        assert_eq!(None, next_smaller_number(135));
    }
}
