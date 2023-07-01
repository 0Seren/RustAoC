use std::collections::HashMap;

fn main() {
    let points_for_throw = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
    ]);

    let wins = HashMap::from([
        ('A', 'B'),
        ('B', 'C'),
        ('C', 'A'),
    ]);

    let input = std::fs::read_to_string("src/strategy").unwrap().replace("\r\n", "\n");
    // let input = String::from("A Y\nB X\nC Z"); //15, 12
    let sum1 = calculate_score1(&input, &points_for_throw, &wins);
    let sum2 = calculate_score2(&input, &points_for_throw, &wins);

    print!("{sum1}\n{sum2}");
}

fn calculate_score1(input: &String, points_for_throw: &HashMap<char, u32>, wins: &HashMap<char, char>) -> u32 {
    let mut sum = 0;
    for game in input.split("\n"){
        let opp_throw = game.chars().nth(0).unwrap();
        let strategy = game.chars().nth(2).unwrap();
        let my_throw = map_throw(strategy);

        sum += calculate_points(my_throw, opp_throw, &points_for_throw, &wins);
    }
    return sum;
}

fn calculate_score2(input: &String, points_for_throw: &HashMap<char, u32>, wins: &HashMap<char, char>) -> u32 {
    let mut sum = 0;
    for game in input.split("\n"){
        let opp_throw = game.chars().nth(0).unwrap();
        let strategy = game.chars().nth(2).unwrap();
        let my_throw = find_throw(opp_throw, strategy, &wins);

        sum += calculate_points(my_throw, opp_throw, &points_for_throw, &wins);
    }
    return sum;
}

fn map_throw(throw: char) -> char {
    match throw {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => '\0'
    }
}

fn find_throw(opp_throw: char, match_result: char, wins: &HashMap<char, char>) -> char{
    let mut losses = HashMap::new();
    for key in wins.keys() {
        losses.insert(wins[key], key);
    }

    match match_result {
        'X' => *losses[&opp_throw],
        'Y' => opp_throw,
        'Z' => wins[&opp_throw],
        _ => '\0'
    }
}

fn calculate_points(my_throw: char, opp_throw: char, points_for_throw: &HashMap<char, u32>, wins: &HashMap<char, char>) -> u32{
    let mut sum = 0;
    
    sum += points_for_throw[&my_throw];

    if wins[&opp_throw] == my_throw{
        sum += 6;
    } else if wins[&my_throw] != opp_throw{
        sum += 3;
    }

    return sum;
}