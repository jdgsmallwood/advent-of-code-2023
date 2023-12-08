use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file_name = "src/input.txt";

    let file_contents = fs::read_to_string(input_file_name).expect("");

    let mut final_sum = 0;
    for line in file_contents.split('\n') {
        let converted_number = concat_first_and_last_numeric_character(&line);
        final_sum += converted_number;
    }

    println!("The final sum is {}.", final_sum);
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

fn get_first_numeric_character(input_string: &str) -> String {
    let number_map: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut minimum_index: i32 = 100;
    let mut minimum_value = "";
    for (key, value) in &number_map {
        let key_index = input_string.find(key);
        let value_index = input_string.find(value);

        (minimum_index, minimum_value) = match (key_index, value_index) {
            (Some(key_index), Some(value_index))
                if ((key_index as i32) < value_index as i32)
                    && ((key_index as i32) < minimum_index) =>
            {
                (key_index as i32, value)
            }
            (Some(_), Some(value_index)) if ((value_index as i32) < minimum_index) => {
                (value_index as i32, value)
            }
            (None, Some(value_index)) if ((value_index as i32) < minimum_index) => {
                (value_index as i32, value)
            }
            (Some(key_index), None) if ((key_index as i32) < minimum_index) => {
                (key_index as i32, value)
            }
            _ => (minimum_index, &minimum_value),
        }
    }
    minimum_value.to_string()
}

fn get_last_numeric_character(input_string: &str) -> String {
    let number_map: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut maximum_index: i32 = -1;
    let mut maximum_value = "";
    for (key, value) in &number_map {
        let key_index = input_string.rfind(key);
        let value_index = input_string.rfind(value);

        (maximum_index, maximum_value) = match (key_index, value_index) {
            (Some(key_index), Some(value_index))
                if (key_index as i32 > value_index as i32)
                    && (key_index as i32 > maximum_index) =>
            {
                (key_index as i32, value)
            }
            (Some(_), Some(value_index)) if (value_index as i32 > maximum_index) => {
                (value_index as i32, value)
            }
            (None, Some(value_index)) if (value_index as i32 > maximum_index) => {
                (value_index as i32, value)
            }
            (Some(key_index), None) if (key_index as i32 > maximum_index) => {
                (key_index as i32, value)
            }
            _ => (maximum_index, &maximum_value),
        }
    }
    maximum_value.to_string()
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
    fn test_get_last_numeric_character_where_last_is_string() {
        assert_eq!(get_last_numeric_character("2eight"), "8");
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

    #[test]
    fn test_get_last_numeric_character_3() {
        assert_eq!(
            get_last_numeric_character("six2twobgzsfsptlqnine42xtmdprjqc"),
            "2"
        );
    }
}
