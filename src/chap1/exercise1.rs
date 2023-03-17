fn main() {
    let x = [1, 2, 3, 4, 5];
    println!("{}", sum(&x));
}

fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        let x = [1, 2, 3, 4, 5];
        assert_eq!(sum(&x), 15);

        let y = [-5, 0, 5];
        assert_eq!(sum(&y), 0);

        let z = [];
        assert_eq!(sum(&z), 0);
    }
}
