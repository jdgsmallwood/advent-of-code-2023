use regex::Regex;
use std::fs;

fn main() {
    let input_file_name = "src/input.txt";

    let file_contents = fs::read_to_string(input_file_name).expect("Failed to read file");

    let symbol_locations: Vec<Vec<usize>> =
        file_contents.lines().map(get_symbol_locations).collect();

    let number_locations: Vec<Vec<(usize, i32)>> =
        file_contents.lines().map(get_number_locations).collect();

    let mut sum: i32 = 0;

    for (line_num, numbers) in number_locations.iter().enumerate() {
        for (number_start, number) in numbers.iter() {
            if symbol_immediately_precedes_number(number_start, &symbol_locations, line_num)
                || symbol_immediately_succeedes_number(
                    number_start,
                    &symbol_locations,
                    line_num,
                    number,
                )
                || symbol_above_number(number_start, &symbol_locations, line_num, number)
                || symbol_below_number(number_start, &symbol_locations, line_num, number)
            {
                sum += number;
                continue;
            }
        }
    }
    println!("The sum is {}.", sum);
}

fn get_symbol_locations(input_string: &str) -> Vec<usize> {
    let mut v = Vec::new();
    for (index, character) in input_string.chars().enumerate() {
        match character {
            '.' => {}
            c if c.is_digit(10) => {}
            _ => {
                v.push(index);
            }
        };
    }
    v
}

fn get_number_locations(input_string: &str) -> Vec<(usize, i32)> {
    let mut v = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for number_match in re.find_iter(input_string) {
        v.push((
            number_match.start(),
            number_match.as_str().parse::<i32>().unwrap(),
        ))
    }
    v
}

fn symbol_immediately_precedes_number(
    number_start: &usize,
    symbol_locations: &[Vec<usize>],
    line_num: usize,
) -> bool {
    *number_start > 0 && ((*symbol_locations)[line_num]).contains(&(number_start - 1))
}

fn symbol_immediately_succeedes_number(
    number_start: &usize,
    symbol_locations: &[Vec<usize>],
    line_num: usize,
    number: &i32,
) -> bool {
    ((*symbol_locations)[line_num]).contains(&(number_start + number.to_string().len()))
}

fn symbol_above_number(
    number_start: &usize,
    symbol_locations: &[Vec<usize>],
    line_num: usize,
    number: &i32,
) -> bool {
    // Saturating Sub means that this will never go below zero, which is not a problem for the
    // logical statement but number_start is of type usize which cannot go below zero, so if
    // it is zero this will cause a panic.
    line_num > 0
        && ((number_start.saturating_sub(1))..=(number_start + number.to_string().len()))
            .any(|i| ((*symbol_locations)[line_num - 1]).contains(&i))
}

fn symbol_below_number(
    number_start: &usize,
    symbol_locations: &[Vec<usize>],
    line_num: usize,
    number: &i32,
) -> bool {
    line_num < symbol_locations.len()
        && ((number_start.saturating_sub(1))..=(number_start + number.to_string().len()))
            .any(|i| ((*symbol_locations)[line_num + 1]).contains(&i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_symbol_locations_blank() {
        assert_eq!(get_symbol_locations("....."), []);
    }

    #[test]
    fn test_get_symbol_locations_no_symbols_but_numeric() {
        assert_eq!(get_symbol_locations(".2..5.."), []);
    }

    #[test]
    fn test_get_symbol_locations_symbols_only() {
        assert_eq!(get_symbol_locations("%"), [0]);
    }

    #[test]
    fn test_get_symbol_locations_symbols_and_dots() {
        assert_eq!(get_symbol_locations("..%...."), [2]);
    }

    #[test]
    fn test_get_symbol_locations_symbols_and_dots_and_numeric() {
        assert_eq!(get_symbol_locations("..2%...."), [3]);
    }

    #[test]
    fn test_get_number_locations_blank() {
        assert_eq!(get_number_locations(""), [])
    }

    #[test]
    fn test_get_number_location_symbols_only() {
        assert_eq!(get_number_locations("...>%S+/"), [])
    }

    #[test]
    fn test_get_number_location_symbols_number_only() {
        assert_eq!(get_number_locations("423"), [(0, 423)])
    }

    #[test]
    fn test_get_number_location_symbols_example_one() {
        assert_eq!(get_number_locations("....423.+23"), [(4, 423), (9, 23)])
    }
}
