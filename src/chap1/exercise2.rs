fn main() {
    if is_prime(3) {
        println!("prime")
    } else {
        println!("false")
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let mut divisor = 2;
    loop {
        if n % divisor == 0 {
            return false;
        }
        divisor += 1;
        if divisor == n {
            return true;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(-1));
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(6));
        assert!(!is_prime(9));
    }
}
