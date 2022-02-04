mod converter;

use converter::int32_to_ip;

fn main() {
    let first = int32_to_ip(2149583361);
    print!("first: {:?}", first)
}
