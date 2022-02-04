fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let anagrams = words
        .to_vec()
        .into_iter()
        .filter(|checked_word| {
            let mut word_copy = checked_word.clone();
            if word.len() != checked_word.len() {
                return false;
            }
            for letter in word.chars() {
                if let Some(index) = word_copy.find(letter) {
                    word_copy.remove(index);
                } else {
                    return false;
                }
            }
            return true;
        })
        .collect();

    anagrams
}

fn main() {
    let result = anagrams(
        "abba",
        &["aabb", "abcd", "bbaa", "dada"].map(|value| value.to_string()),
    );
    println!("result: {:?}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);
        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}
