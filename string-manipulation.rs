use std::collections::HashMap;


fn get_string() -> String {
    let mut input = String::new();
    println!("Please give any string:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}


fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}


fn capitalize_string(input: &str) -> String {
    let mut str_iter = input.chars();
    let first_char = str_iter.next().expect("No first char in string");
    let first_upper = first_char.to_uppercase().collect::<String>();
    let rest_lower = str_iter.as_str().to_lowercase();
    first_upper + &rest_lower
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}


fn count_characters(input: &str) -> HashMap<char, u32> {
    let mut char_map = HashMap::new();

    for ch in input.chars() {
        let count = char_map.entry(ch).or_insert(0);
        *count += 1;
    }

    char_map
}


fn main(){
    let input_string = get_string();
    println!("You entered: {input_string}");
    
    let reversed_string = reverse_string(&input_string);
    println!("Reversed string: {reversed_string}");

    let capitalized_string = capitalize_string(&input_string);
    println!("Capitalized string: {capitalized_string}");

    let word_count = count_words(&input_string);
    println!("Word count of string: {word_count}");

    let char_count = count_characters(&input_string);
    println!("Character count map for string: {char_count:?}");

}

/*=>
Please give any string:
sadf adsfa /sd/sf adsfas
You entered: sadf adsfa /sd/sf adsfas
Reversed string: safsda fs/ds/ afsda fdas
Capitalized string: Sadf adsfa /sd/sf adsfas
Word count of string: 4
Character count map for string: {'/': 2, 'd': 4, 'a': 5, 's': 6, 'f': 4, ' ': 3}
 */



