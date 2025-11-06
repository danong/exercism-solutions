use std::collections::HashSet;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;


fn normalize(s: &str) -> String {
    // s.nfkc() yields normalized chars
    // .flat_map(|c| c.to_lowercase()) lowercases each char
    // and flattens multi-char results
    // collect() consumes iterator to build normalized lowercase string
    s.nfkc().flat_map(|c| c.to_lowercase()).collect()
}

fn char_count_for(word: &str) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for char in word.chars() {
        let count = char_count.entry(char).or_insert(0);
        *count += 1;
    }
    char_count
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    let normalized_word = normalize(word);
    let target_char_count = char_count_for(&normalized_word);
    
    for candidate in possible_anagrams {
        let normalized_candidate = normalize(candidate);

        if normalized_candidate == normalized_word {
            continue
        }
        
        let candidate_char_count = char_count_for(&normalized_candidate);
        if target_char_count == candidate_char_count {
            anagrams.insert(candidate);
        }
    }
    anagrams
}
