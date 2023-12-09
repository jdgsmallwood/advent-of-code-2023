use std::fs;

fn main() {
    let input_file_name = "src/input.txt";

    let file_contents = fs::read_to_string(input_file_name).expect("");

    let mut final_sum = 0;
    for line in file_contents.split('\n') {
        if !line.contains("Game") {
            continue;
        }

        if game_is_possible(line) {
            final_sum += get_id_from_game_string(line);
        }
    }

    println!("The final sum is {}.", final_sum);
}

fn game_is_possible(input_string: &str) -> bool {
    let round_string = get_round_string(input_string);

    for substring in round_string.split(';') {
        let cube_counts = get_number_of_cubes_of_each_color_from_string(substring);

        if !round_is_possible(cube_counts) {
            return false;
        }
    }
    true
}

fn get_round_string(input_string: &str) -> String {
    let colon_index = match input_string.find(':') {
        Some(index) => index,
        _ => 0,
    };

    let round_string = input_string.get((colon_index + 1)..);
    round_string.expect("").trim().to_string()
}

fn get_id_from_game_string(input_string: &str) -> i32 {
    let split_string: Vec<_> = input_string.split(':').collect();

    let game_id_string: Vec<_> = split_string[0].split(' ').collect();

    let id = game_id_string.last();

    id.expect("").parse::<i32>().unwrap()
}

fn get_number_of_cubes_of_each_color_from_string(input_string: &str) -> (i32, i32, i32) {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    for substring in input_string.split(',') {
        let split_substring = substring.trim();
        let space_index = split_substring.find(' ').unwrap_or(0);

        match split_substring.get(space_index..) {
            Some(index) if index.trim() == "blue" => {
                blue += split_substring
                    .get(..space_index)
                    .expect("")
                    .trim()
                    .parse::<i32>()
                    .unwrap()
            }
            Some(index) if index.trim() == "red" => {
                red += split_substring
                    .get(..space_index)
                    .expect("")
                    .trim()
                    .parse::<i32>()
                    .unwrap()
            }
            Some(index) if index.trim() == "green" => {
                green += split_substring
                    .get(..space_index)
                    .expect("")
                    .trim()
                    .parse::<i32>()
                    .unwrap()
            }
            _ => {}
        }
    }

    (blue, red, green)
}

fn round_is_possible(cube_counts: (i32, i32, i32)) -> bool {
    // this is blue -> red -> green
    if cube_counts.0 > 14 || cube_counts.0 < 0 {
        return false;
    }

    if cube_counts.1 > 12 || cube_counts.1 < 0 {
        return false;
    }

    if cube_counts.2 > 13 || cube_counts.2 < 0 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_cubes_of_each_color_from_string() {
        assert_eq!(
            get_number_of_cubes_of_each_color_from_string("3 blue, 4 red"),
            (3, 4, 0)
        );
    }

    #[test]
    fn test_get_number_of_cubes_of_each_color_from_string_2() {
        assert_eq!(
            get_number_of_cubes_of_each_color_from_string("3 blue, 4 red, 8 green"),
            (3, 4, 8)
        );
    }

    #[test]
    fn test_get_number_of_cubes_of_each_color_from_string_3() {
        assert_eq!(
            get_number_of_cubes_of_each_color_from_string("12 green"),
            (0, 0, 12)
        );
    }

    #[test]
    fn test_get_number_of_cubes_of_each_color_from_string_3_with_whitespace() {
        assert_eq!(
            get_number_of_cubes_of_each_color_from_string(" 12 green "),
            (0, 0, 12)
        );
    }

    #[test]
    fn test_round_is_possible() {
        assert_eq!(round_is_possible((0, 0, 1)), true)
    }

    #[test]
    fn test_round_is_possible_2() {
        assert_eq!(round_is_possible((0, 0, 100)), false)
    }

    #[test]
    fn test_round_is_possible_3() {
        assert_eq!(round_is_possible((-20, 0, 1)), false)
    }

    #[test]
    fn test_game_is_possible() {
        assert_eq!(game_is_possible("Game 1: 4 blue"), true)
    }

    #[test]
    fn test_game_is_possible_2() {
        assert_eq!(game_is_possible("Game 1: 4 blue; 4 red, 400 green"), false)
    }

    #[test]
    fn test_get_id_from_game_string() {
        assert_eq!(
            get_id_from_game_string("Game 1: 4 blue; 4 red, 400 green"),
            1
        )
    }
    #[test]
    fn test_get_id_from_game_string_2() {
        assert_eq!(
            get_id_from_game_string("Game 102: 4 blue; 4 red, 400 green"),
            102
        )
    }

    #[test]
    fn test_get_round_string() {
        assert_eq!(
            get_round_string("Game 102: 4 blue; 4 red, 400 green"),
            "4 blue; 4 red, 400 green"
        )
    }
}
