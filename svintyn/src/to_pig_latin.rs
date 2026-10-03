pub fn to_pig_latin(text: &str) -> String {
    let mut ans = String::new();

    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    for word in text.split_whitespace() {
        if word.is_empty() {
            continue;
        }

        if !ans.is_empty() {
            ans.push(' ');
        }

        match word.chars().next() {
            Some(first_char) => {
                if vowels.contains(&first_char) {
                    ans.push_str(word);
                    ans.push_str("-hay");
                } else {
                    let rest: String = word.chars().skip(1).collect();
                    ans.push_str(&rest);
                    ans.push_str(&format!("-{first_char}ay"));
                }
            }
        None => {}
        }
    }
    ans
}