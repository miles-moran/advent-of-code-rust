use std::fs;

fn main() {
    let text = fs::read_to_string("input.txt").expect("Error reading");
    
    let mut score = 0;
    for row in text.split("\n"){
        println!("{}", row);
        let split = row.split_at(row.len() / 2);
        println!("{:#?}", split);
        let first = split.0;
        let second = split.1;
        let s = first.chars().find(|&f| second.contains(f));
        if s.is_none(){
            println!("skipping");
            continue;
        }
        let c = s.unwrap();
        println!("{:#?}", c);
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let point = alphabet.find(c).unwrap_or(0);
        score += point + 1;
    }

    println!("{}", score);
} 
