use std::fs;

fn main() {
    let path = "input.txt";
    let binding = fs::read_to_string(path).expect("Error reading");
    let rows = binding.split("\n");

    let mut rows_total = 0;
    let mut row_totals: Vec<i32> = Vec::new();    
    for row in rows {
        if row.trim() == ""{
            row_totals.push(rows_total.clone());
            rows_total = 0;
        } else {
            let parsed_value = row.parse::<i32>().expect("Unable to parse string");
            rows_total += parsed_value;
        }
    }
    row_totals.sort();
    row_totals.reverse();


    println!("{:#?}", row_totals); 
    println!("{:#?}", row_totals[0]); 
    println!("{:#?}", row_totals[0] + row_totals[1] + row_totals[2]); 
}
