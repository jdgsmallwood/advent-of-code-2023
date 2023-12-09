use std::fs;

fn main() {
    let input_file_name = "src/input.txt";

    let file_contents = fs::read_to_string(input_file_name).expect("Failed to read the file");

    let final_sum: i32 = file_contents
                        .lines()
                        .filter(|line| line.contains("Game"))
                        .map(|line| {
                            let round_string = get_round_string(line);

                            let max_cube_counts = get_maximum_number_of_cubes_of_each_color_in_game(&round_string);
                    
                            get_power(max_cube_counts)
                    
                        })
                        .sum();

    println!("The final sum is {}.", final_sum);
}

fn get_round_string(input_string: &str) -> String {
    let colon_index = input_string.find(':').unwrap_or(0);
    input_string.get((colon_index + 1)..).unwrap_or("").trim().to_string()
}

fn get_maximum_number_of_cubes_of_each_color_in_game(game_string: &str) -> (i32, i32, i32) {
    
    let (max_blue, max_red, max_green) = game_string
    .split(';')
    .map(|substring| get_number_of_cubes_of_each_color_from_string(substring))
    .fold((0,0,0), |acc, cube_counts| {
        (
            acc.0.max(cube_counts.0),
            acc.1.max(cube_counts.1),
            acc.2.max(cube_counts.2),
        )
    });

    (max_blue, max_red, max_green)
    
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

fn get_power(cube_counts: (i32, i32, i32)) -> i32 {
    cube_counts.0 * cube_counts.1 * cube_counts.2
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
    fn test_get_round_string() {
        assert_eq!(
            get_round_string("Game 102: 4 blue; 4 red, 400 green"),
            "4 blue; 4 red, 400 green"
        )
    }

    #[test]
    fn test_get_maximum_number_of_cubes_of_each_color_in_game() {
        assert_eq!(
            get_maximum_number_of_cubes_of_each_color_in_game(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            (15, 14, 3)
        )
    }

    #[test]
    fn test_get_power() {
        assert_eq!(get_power((2, 3, 4)), 2 * 3 * 4)
    }
}
