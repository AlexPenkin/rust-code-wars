use std::net::Ipv4Addr;

pub fn int32_to_ip(int: u32) -> String {
    let mut result = String::new();
    let bytes = int.to_be_bytes().iter().join(".");
    bytes.map(|number_in_bytes| {
        result.push_str(&format!("{}.", number_in_bytes.to_string()).to_string());
    });
    result.pop();
    result
  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}
