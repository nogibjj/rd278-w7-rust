pub fn check_palindrome(word: &str) -> String {
    let reversed_word = word.chars().rev().collect::<String>();
    if word == reversed_word {
        format!("Message: {} is a palindrome", word)
    } else {
        format!("Message: {} is not a palindrome", word)
    }
}
