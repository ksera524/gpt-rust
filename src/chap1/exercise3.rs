fn main() {
    println!("The 5th Fibonacci number is {}",fibonacci(5));
    println!("The 10th Fibonacci number is {}",fibonacci_gpt(10));
}


fn fibonacci(n: u32) -> u32 {
    match n {
        0 => n,
        1 => n,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}


fn fibonacci_gpt(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut prev_prev = 0;
    let mut prev = 1;
    let mut current = 0;
    for _ in 2..=n {
        current = prev_prev + prev;
        prev_prev = prev;
        prev = current;
    }
    current
}


#[cfg(test)]
mod test {
 use super::*;

    #[test]
    fn test_fibonacci(){
        assert_eq!(fibonacci(0),0);
        assert_eq!(fibonacci(1),1);
        assert_eq!(fibonacci(2),1);
        assert_eq!(fibonacci(3),2);
        assert_eq!(fibonacci(7),13);
        assert_eq!(fibonacci(10),55);
    }

    #[test]
    fn test_fibonacci_gpt() {
        assert_eq!(fibonacci_gpt(0), 0);
        assert_eq!(fibonacci_gpt(1), 1);
        assert_eq!(fibonacci_gpt(2), 1);
        assert_eq!(fibonacci_gpt(3), 2);
        assert_eq!(fibonacci_gpt(7), 13);
        assert_eq!(fibonacci_gpt(10), 55);
    }
}