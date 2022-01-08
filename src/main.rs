use rand::Rng;
use std::{
    collections::{hash_map, HashMap},
    fmt, fs, io, path, vec,
};

fn main() {
    loop {
        println!("Which menu option do you want?");
        println!("1 - Play guess the number");
        println!("2 - Find the area of a rectangle");
        println!("3 - Play rectangle stacking");
        println!("4 - Play football");
        println!("5 - Vectors");
        println!("6 - Mean, median, and mode");
        println!("7 - Pig Latin");
        println!("8 - Employee Database");
        println!("9 - Grep");

        let menu = get_menu_option(1, 9);

        match menu {
            1 => guess_the_number(),
            2 => area_of_rectangle(),
            3 => rectangle_stacking(),
            4 => play_football(),
            5 => vectors(),
            6 => mean_median_mode(),
            7 => pig_latin_converter(),
            8 => employee_database(),
            9 => grep(),
            _ => break,
        }
    }
}

fn get_menu_option(min: u32, max: u32) -> u32 {
    loop {
        println!("Selection: ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read input");
                continue;
            }
        }

        let input: u32 = match input
            .trim() // clear whitespace // optional: i32, u32, etc.
            .parse()
        {
            Ok(num) => num, // Returned an ok value, so continue with guess = num
            Err(_) => {
                println!("Failed to read input");
                continue;
            }
        }; // Handle error

        if input < min || input > max {
            println!("Input out of range");
            continue;
        }

        break input;
    }
}

fn guess_the_number() {
    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..101); // Immutable by default
        let guess = get_menu_option(1, 101);

        println!("The correct answer was {}", secret_number);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Small!"),
            std::cmp::Ordering::Greater => println!("Too Big!"),
            std::cmp::Ordering::Equal => {
                println!("Correct, you win!");
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, internal: &Rectangle) -> bool {
        internal.width < self.width && internal.height < self.height
    }

    fn square(side: f64) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn area_of_rectangle() {
    let width = 5.0;
    let height = 7.0;
    println!(
        "The area of the rectangle is {} square pixels.",
        area_from_dimensions(width, height)
    );

    // f64 can be copied, so this code is valid
    // Normally you wouldn't be allowed to use width and height here
    // They could be passed as references into area_from_dimensions to be sure
    let rect = Rectangle { width, height };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_from_struct(&rect)
    );
    println!("The rectangle (in debug) looks like {:?}", &rect);

    println!(
        "The area of the rectangle by method is {}",
        area_from_method(&rect)
    );

    let square = Rectangle::square(4.0);
    println!("The area of the square is {}", square.area())
}

fn area_from_dimensions(width: f64, length: f64) -> f64 {
    width * length
}

fn area_from_struct(rect: &Rectangle) -> f64 {
    rect.height * rect.width
}

fn area_from_method(rect: &Rectangle) -> f64 {
    rect.area()
}

fn rectangle_stacking() {
    let r1 = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let r2 = Rectangle {
        width: 3.0,
        height: 1.0,
    };

    let r3 = Rectangle {
        width: 6.0,
        height: 5.0,
    };

    let mut operator = if r1.can_hold(&r2) { "can" } else { "cannot" };
    println!(
        "Rectangle r2 ({:?}) {} fit inside rectangle r1 ({:?})",
        &r2, &operator, &r1
    );

    operator = if r2.can_hold(&r3) { "can" } else { "cannot" };
    println!(
        "Rectangle r3 ({:?}) {} fit inside rectangle r2 ({:?})",
        &r3, &operator, &r2
    );
}

#[derive(Debug)]
enum MatchResult {
    HomeWin,
    Draw,
    AwayWin,
    Abandoned,
    Postponed,
    Scheduled(String),
}

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let MatchResult::Scheduled(time) = self {
            write!(f, "{}", time)
        } else {
            write!(f, "Game Over")
        }
    }
}

struct MatchScore {
    home: u32,
    away: u32,
}

impl MatchScore {
    fn match_result(&self) -> MatchResult {
        match self.home.cmp(&self.away) {
            std::cmp::Ordering::Less => MatchResult::AwayWin,
            std::cmp::Ordering::Greater => MatchResult::Draw,
            std::cmp::Ordering::Equal => MatchResult::HomeWin,
        }
    }

    fn to_string(&self) -> String {
        format!("{} - {}", self.home, self.away)
    }
}

impl fmt::Display for MatchScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

enum MatchCompletion {
    Incomplete,
    NormalTime {
        score: MatchScore,
    },
    ExtraTime {
        score: MatchScore,
        aet: MatchScore,
    },
    Penalties {
        score: MatchScore,
        aet: MatchScore,
        penalties: MatchScore,
    },
}

impl MatchCompletion {
    fn match_result(&self) -> MatchResult {
        match self {
            MatchCompletion::Incomplete => MatchResult::Scheduled(String::from("15:00")),
            MatchCompletion::NormalTime { score } => score.match_result(),
            MatchCompletion::ExtraTime { aet, .. } => aet.match_result(),
            MatchCompletion::Penalties { penalties, .. } => penalties.match_result(),
        }
    }

    fn game_time(&self) -> String {
        if let MatchResult::Scheduled(time) = self.match_result() {
            time
        } else {
            String::from("Game Finished")
        }
    }

    fn to_string(&self) -> String {
        let score_string = match self.match_result() {
            MatchResult::HomeWin | MatchResult::Draw | MatchResult::AwayWin => match self {
                MatchCompletion::NormalTime { score } => score.to_string(),
                MatchCompletion::ExtraTime { score, aet } => {
                    format!("{} ({} AET)", score.to_string(), aet.to_string())
                }
                MatchCompletion::Penalties {
                    score,
                    aet,
                    penalties,
                } => format!(
                    "{} ({} AET) ({} Pen)",
                    score.to_string(),
                    aet.to_string(),
                    penalties.to_string()
                ),
                MatchCompletion::Incomplete => String::from(""),
            },
            MatchResult::Abandoned => String::from("A - A"),
            MatchResult::Postponed => String::from("P - P"),
            MatchResult::Scheduled(time) => time,
        };

        format!("Team 1 {} Team 2", score_string)
    }
}

impl fmt::Display for MatchCompletion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn play_football() {
    let unplayed_game = MatchCompletion::Incomplete;
    let home_win_normal_time = MatchCompletion::NormalTime {
        score: MatchScore { home: 2, away: 1 },
    };
    let home_win_extra_time = MatchCompletion::ExtraTime {
        score: MatchScore { home: 2, away: 2 },
        aet: MatchScore { home: 4, away: 3 },
    };
    let home_win_penalties_time = MatchCompletion::Penalties {
        score: MatchScore { home: 2, away: 2 },
        aet: MatchScore { home: 4, away: 4 },
        penalties: MatchScore { home: 5, away: 1 },
    };
    let draw = MatchCompletion::NormalTime {
        score: MatchScore { home: 0, away: 0 },
    };
    let away_win_normal_time = MatchCompletion::NormalTime {
        score: MatchScore { home: 0, away: 4 },
    };

    println!("Game Schedule:");
    println!("Unplayed game: {}", unplayed_game.game_time());
    println!("Home win: {}", home_win_normal_time.game_time());
    println!("Home win AET: {}", home_win_extra_time.game_time());
    println!("Home win PEN: {}", home_win_penalties_time.game_time());
    println!("Draw: {}", draw.game_time());
    println!("Away win: {}", away_win_normal_time.game_time());

    println!("Unplayed game: {}", unplayed_game);
    println!("Home win: {}", home_win_normal_time);
    println!("Home win AET: {}", home_win_extra_time);
    println!("Home win PEN: {}", home_win_penalties_time);
    println!("Draw: {}", draw);
    println!("Away win: {}", away_win_normal_time);
}

fn append_sum(vec: &mut Vec<i32>) {
    let mut sum: i32 = 0;

    for v in vec.iter() {
        sum += v;
    }

    // Long way to deref the vector in a function call:
    vec.push(sum);
}

fn games_to_be_played(all_games: &[MatchResult]) -> Vec<&MatchResult> {
    let mut scheduled_games: Vec<&MatchResult> = Vec::new();
    for g in all_games {
        if let MatchResult::Scheduled(..) = g {
            scheduled_games.push(g);
        }
    }
    scheduled_games
}

fn multiply_vec(v: &mut Vec<f64>) {
    for (i, val) in v.into_iter().enumerate() {
        let f = i as f64;
        *val *= f;
    }
}

fn vectors() {
    println!("Creating vectors");

    let mut v1: Vec<i32> = Vec::new(); // Type of Vec::new() is impplied by defining the type in let
    v1.push(0);
    v1.push(1);
    v1.push(2);
    append_sum(&mut v1);

    println!("v1: {:?}", v1);

    let mut v2 = vec![
        MatchResult::Abandoned,
        MatchResult::Postponed,
        MatchResult::Scheduled(String::from("15:00")),
        MatchResult::HomeWin,
        MatchResult::Draw,
        MatchResult::Scheduled(String::from("19:00")),
    ];

    // There should be only two games to be played
    let to_be_played = games_to_be_played(&v2);
    println!("Games to be played: {:?}", to_be_played);

    // Change the games: the details should update because we have returned a slice
    v2[2] = MatchResult::Scheduled(String::from("15:45"));
    v2[5] = MatchResult::HomeWin;

    let to_be_played_now = games_to_be_played(&v2);
    println!("Games to be played: {:?}", to_be_played_now);

    let mut v3 = vec![0.5, 0.6, 1.4, -0.3, 7.2];
    println!("Values in the array: {:?}", &v3);
    multiply_vec(&mut v3);
    println!("Modified values in the array: {:?}", &v3);
}

struct ArrayDetails {
    mean: f64,
    mode: i32,
    median: f64,
}

impl fmt::Display for ArrayDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Mean: {}\nMedian: {}\nMode: {}",
            self.mean, self.median, self.mode
        )
    }
}

fn get_averages(vec: &mut Vec<i32>) -> ArrayDetails {
    let mut sum: f64 = 0.;
    let mut count: i32 = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    vec.sort();

    for v in vec.iter() {
        sum += *v as f64;
        count += 1;

        let entry = map.entry(*v).or_insert(0);
        *entry += 1;
    }

    let mut most_frequent = 0;
    let mut freq_of_most_frequent = 0;
    for (k, v) in &map {
        if *v > freq_of_most_frequent {
            freq_of_most_frequent = *v;
            most_frequent = *k;
        }
    }

    let mid_idx = (count / 2) as usize;
    let median = if count & 1 == 1 {
        // Count is odd, there is a middle point
        vec[mid_idx] as f64
    } else {
        (vec[mid_idx - 1] + vec[mid_idx]) as f64 / 2.
    };

    ArrayDetails {
        mean: sum / count as f64,
        mode: most_frequent,
        median,
    }
}

fn mean_median_mode() {
    let mut test1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9];

    println!("Test1 vector: {:?}", test1);
    let metrics = get_averages(&mut test1);
    println!("Metrics:");
    println!("{}", metrics);

    let mut test2 = vec![-1, -1, 1, 1];

    println!("Test2 vector: {:?}", test2);
    let metrics = get_averages(&mut test2);
    println!("Metrics:");
    println!("{}", metrics);

    let mut test3 = vec![4, 4, 4, 4, 5, 5, 5, 5, 6];

    println!("Test3 vector: {:?}", test3);
    let metrics = get_averages(&mut test3);
    println!("Metrics:");
    println!("{}", metrics);

    let mut test4 = vec![9, 7, 4, 6, 8, 23, 5, -3, 23, 6, 4, 4, 8, 3, 0, -10];

    println!("Test4 vector: {:?}", test4);
    let metrics = get_averages(&mut test4);
    println!("Metrics:");
    println!("{}", metrics);
}

fn is_consonant(c: &str) -> bool {
    match c {
        "a" | "e" | "i" | "o" | "u" | "A" | "E" | "I" | "O" | "U" => false,
        _ => true,
    }
}

fn to_pig_latin(s: &str) -> String {
    // Is the first letter a vowel or a consonant?
    let mut sep = 1;
    while !s.is_char_boundary(sep) && sep < s.len() {
        sep += 1;
    }

    if sep == s.len() {
        // All vowels
        let mut new_str = String::from(s);
        new_str.push_str("hay");
        return new_str;
    }

    // The first character is then a slice:
    let first_char = &s[0..sep];

    if is_consonant(first_char) {
        let mut new_str = String::from(&s[sep..]);
        new_str.push_str(first_char);
        new_str.push_str("ay");
        new_str
    } else {
        let mut new_str = String::from(s);
        new_str.push_str("hay");
        new_str
    }
}

fn pig_latin_converter() {
    let story =
        "The quick brown fox jumped over the lazy dog aaa a b oops Здравствуйте aЗдравствуйте Apple First";

    let mut translated_story = String::new();
    for word in story.split(" ") {
        translated_story.push_str(&to_pig_latin(word));
        translated_story.push_str(" ");
    }

    println!("Story: {}", story);
    println!("Translation: {}", translated_story);
}

#[derive(Debug)]
struct EmployeeDetails {
    name: String,
    department: String,
}

enum EmployeeActions {
    Add(EmployeeDetails),
    Remove(EmployeeDetails),
    Repeat(String),
    Display,
    Quit,
}

fn get_single_action(action: &str) -> EmployeeActions {
    match &action.to_lowercase()[..] {
        "quit" | "exit" => EmployeeActions::Quit,
        "continue" | "display" | "finished" => EmployeeActions::Display,
        "repeat" => EmployeeActions::Repeat(String::from("Repeat requested")),
        _ => EmployeeActions::Repeat(format!("Did not recognise input {}", action)),
    }
}

fn get_details_action(action: &[&str]) -> EmployeeActions {
    if action.len() >= 3 {
        let name = String::from(action[1]);

        let department = if action.len() == 3 {
            String::from(action[2])
        } else if action.len() >= 4 {
            String::from(action[3])
        } else {
            return EmployeeActions::Repeat(String::from("Failed to parse a department"));
        };

        let employee_details = EmployeeDetails { name, department };

        match &action[0].to_lowercase()[..] {
            "add" => EmployeeActions::Add(employee_details),
            "remove" => EmployeeActions::Remove(employee_details),
            _ => EmployeeActions::Repeat(String::from("Failed to parse an action")),
        }
    } else {
        EmployeeActions::Repeat(String::from("Not enough inputs"))
    }
}

fn parse_employee_details(line: &str) -> EmployeeActions {
    let parts: Vec<&str> = line.split(" ").collect();

    match parts.len() {
        0 => EmployeeActions::Repeat(String::from("No data was entered")),
        1 => get_single_action(parts[0]),
        2 => EmployeeActions::Repeat(String::from("Insufficient data entered")),
        3 | 4 => get_details_action(&parts[..]),
        _ => EmployeeActions::Repeat(String::from("Too much data entered")),
    }
}

fn employee_database() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    let mut getting_inputs = true;

    while getting_inputs {
        println!("Enter instruction");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read input");
                continue;
            }
        }

        match parse_employee_details(input.trim()) {
            EmployeeActions::Add(employee) => {
                let department = company.entry(employee.department).or_insert(Vec::new());
                department.push(employee.name);
            }
            EmployeeActions::Remove(employee) => {
                if let hash_map::Entry::Occupied(department) =
                    company.entry(employee.department.clone())
                {
                    let values = department.into_mut();
                    if let Some(idx) = values.iter().position(|x| *x == employee.name) {
                        values.remove(idx);
                    };
                    if values.len() == 0 {
                        company.remove(&employee.department);
                    }
                }
            }
            EmployeeActions::Display => getting_inputs = false,
            EmployeeActions::Quit => return,
            EmployeeActions::Repeat(message) => println!("Try again: {}", message),
        };
    }

    for (k, v) in company {
        println!("Department: {}\nEmployees: {:?}", k, v);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rectangle_should_fit() {
        let r1 = Rectangle {
            width: 5.,
            height: 10.,
        };

        let r2 = Rectangle {
            width: 10.,
            height: 12.,
        };

        assert!(r2.can_hold(&r1));
    }

    #[test]
    fn rectangle_should_not_fit() {
        let r1 = Rectangle {
            width: 5.,
            height: 10.,
        };

        let r2 = Rectangle {
            width: 10.,
            height: 12.,
        };

        assert!(!r1.can_hold(&r2));
    }

    #[test]
    fn rectangle_might_fit() {
        let r1 = Rectangle {
            width: 5.,
            height: 10.,
        };

        let r2 = Rectangle {
            width: 6.,
            height: 4.,
        };

        assert!(!r1.can_hold(&r2));
    }
}

fn grep() {
    println!("Enter the file name to search: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    println!("Enter the search term: ");
    let mut search_term = String::new();
    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read search term");

    let filepath = path::Path::new("res/").join(&input.trim());
    let file_contents = fs::read_to_string(&filepath).expect("Failed to read file");

    let mut matching_lines: Vec<&str> = Vec::new();
    for line in file_contents.lines() {
        println!("{}", line);
        if line.contains(&search_term) {
            matching_lines.push(line)
        }
    }

    for line in matching_lines.iter() {
        println!("{}", line);
    }
}
