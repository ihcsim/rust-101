use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    scores.insert(String::from("Red"), 30);
    for (team, score) in &scores {
        println!("team {}: {}", team, score);
    }

    // insert if not exists
    scores.entry(String::from("Green")).or_insert(20);

    println!("before scores correction: ");
    println!("{:?}", scores);

    // update existing value
    let mut corrected = HashMap::new();
    for (team, score) in &scores {
        let corrected_score = corrected.entry(team).or_insert(0);
        *corrected_score = score + 80;
    }

    println!("after scores correction: ");
    println!("{:?}", corrected);
}
