use std::collections::HashMap;

pub fn find_words(sentence: String) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for word in sentence.split_whitespace() {
        let count: &mut i32 = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    map
}
