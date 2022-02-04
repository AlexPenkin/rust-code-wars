fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut prev;
    let mut first = 0;
    let mut second = 1;
    loop {
        prev = first;
        first = second;
        second = first + prev;
        let multiple_result = first * second;
        if multiple_result == prod  {
            return (first, second, true);
        }
        if multiple_result > prod  {
            return (first, second, false);
        }
    }
}

fn main() {
    println!("Hello, world!");

    let result = product_fib(4895);
    let result2 = product_fib(5895);
    println!("result: {:?}", result);
    println!("result2: {:?}", result2);
}
#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
