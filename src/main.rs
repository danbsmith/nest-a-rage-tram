fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        println!(
            "Incorrect number of arguments.  Expected usage: {} <base word>",
            args.next().unwrap()
        );
        panic!();
    }
    let base_word = args.nth(1).unwrap();
    let stdin = std::io::stdin();
    let mut test_word = String::new();
    while let Ok(bytes) = stdin.read_line(&mut test_word) {
        if bytes <= 1 {
            break;
        }
        print!("{} versus {}", base_word, test_word);
        let base_word: String = base_word
            .to_lowercase()
            .split_whitespace()
            .flat_map(|s| s.chars())
            .collect();
        test_word = test_word
            .to_lowercase()
            .split_whitespace()
            .flat_map(|s| s.chars())
            .collect();
        if is_anagram(&base_word, &test_word) {
            println!("✔️");
        } else {
            println!("❌")
        }
        test_word.clear();
    }
}

fn is_anagram(base: &std::string::String, test: &std::string::String) -> bool {
    let mut test: std::vec::Vec<char> = test.chars().collect();
    for c in base.chars() {
        match test.iter().position(|&x| x==c) {
            Some(index) => { test.remove(index); }
            None => {return false;}
        }
    }
    test.is_empty()
}
