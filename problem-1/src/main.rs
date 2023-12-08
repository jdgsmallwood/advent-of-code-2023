use std::fs;

fn main() {
    let input_file_name = "src/input.txt";

    let file_contents = fs::read_to_string(input_file_name).expect("");

    let mut final_sum = 0;
    for line in file_contents.split('\n') {
        final_sum += concat_first_and_last_numeric_character(line);
    }

    println!("The final sum is {}.", final_sum);
}

fn get_first_numeric_character(input_string: &str) -> String {
    for character in input_string.chars() {
        if character.is_numeric() {
            return character.to_string();
        }
    }
    "".to_string()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn get_last_numeric_character(input_string: &str) -> String {
    let reversed_string = reverse(input_string);

    get_first_numeric_character(&reversed_string)
}

fn concat_first_and_last_numeric_character(input_string: &str) -> i32 {
    let first_numeric_character_is_empty_check = get_first_numeric_character(input_string);
    if first_numeric_character_is_empty_check.is_empty() {
        return 0;
    }

    let first_numeric_character = first_numeric_character_is_empty_check;
    let last_numeric_character = get_last_numeric_character(input_string);
    let concat_characters = format!("{}{}", first_numeric_character, last_numeric_character);
    concat_characters.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_numeric_character() {
        assert_eq!(get_first_numeric_character("a1b2c3"), "1");
    }

    #[test]
    fn test_get_first_numeric_character_second() {
        assert_eq!(get_first_numeric_character("b2c3d4"), "2");
    }

    #[test]
    fn test_get_first_numeric_character_where_no_numeric_character_exists() {
        assert_eq!(get_first_numeric_character("bcd"), "");
    }

    #[test]
    fn test_get_first_numeric_character_where_string_empty() {
        assert_eq!(get_first_numeric_character(""), "");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("bcd"), "dcb");
    }

    #[test]
    fn test_reverse_empty_string() {
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_get_last_numeric_character() {
        assert_eq!(get_last_numeric_character("a1b2c3"), "3");
    }

    #[test]
    fn test_get_last_numeric_character_second() {
        assert_eq!(get_last_numeric_character("b2c3d4"), "4");
    }

    #[test]
    fn test_get_last_numeric_character_where_no_numeric_character_exists() {
        assert_eq!(get_last_numeric_character("bcd"), "");
    }

    #[test]
    fn test_get_last_numeric_character_where_string_empty() {
        assert_eq!(get_last_numeric_character(""), "");
    }

    #[test]
    fn test_concat_first_and_last_numeric_character_1() {
        assert_eq!(concat_first_and_last_numeric_character("a1b2c3"), 13)
    }

    #[test]
    fn test_concat_first_and_last_numeric_character_only_one_numeric_character() {
        assert_eq!(concat_first_and_last_numeric_character("ab2c"), 22)
    }

    #[test]
    fn test_concat_first_and_last_numeric_character_no_numeric_character() {
        assert_eq!(concat_first_and_last_numeric_character("abc"), 0)
    }

    #[test]
    fn test_concat_first_and_last_numeric_character_2() {
        assert_eq!(concat_first_and_last_numeric_character("1abc2"), 12)
    }
}
