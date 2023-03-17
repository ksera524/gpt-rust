fn main() {
    if is_palindrome("abcba".to_string()) {
        println!("abcba is palindrome");
    }

}

fn is_palindrome(sentence: String) -> bool {
    sentence.chars().collect::<String>() == sentence.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_palindrome_test(){
        assert!(is_palindrome("abcba".to_string()));
        assert!(!is_palindrome("abcde".to_string()));
    }
}