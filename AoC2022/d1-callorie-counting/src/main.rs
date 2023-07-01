fn main() {
    let input = std::fs::read_to_string("src/calories").unwrap();
    count_caleries(input);
}

fn count_caleries(input: String) {
    let mut cals = Vec::new();
    
    for group in input.replace("\r\n", "\n").split("\n\n") {
        let mut sum = 0;

        for calorie in group.split("\n"){
            sum += calorie.parse::<i32>().unwrap();
        }

        cals.push(sum);
    }

    cals.sort();
    cals.reverse();
    
    print!("{}\n", cals[0]);
    print!("{}", cals[0] + cals[1] + cals[2]);
}