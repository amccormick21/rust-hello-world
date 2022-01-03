use rand::Rng;
use std::fmt;
use std::io;
use std::vec;

fn main() {
    loop {
        println!("Which menu option do you want?");
        println!("1 - Play guess the number");
        println!("2 - Find the area of a rectangle");
        println!("3 - Play rectangle stacking");
        println!("4 - Play football");
        println!("5 - Vectors");

        let menu = get_menu_option(1, 5);

        match menu {
            1 => guess_the_number(),
            2 => area_of_rectangle(),
            3 => rectangle_stacking(),
            4 => play_football(),
            5 => vectors(),
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
