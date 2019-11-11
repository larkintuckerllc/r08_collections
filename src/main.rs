fn main() {
    // VECTOR (SAME TYPE)
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let one = v.get(0);
    if let Some(i) = one {
        println!("The value of the first item of v is: {}", i); // 1
    }
    for i in &v {
        println!("{}", i); // 1 2 3 4 5 6 7 8
    }

    // VECTOR (MULTIPLE TYPES)
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let three = row.get(0);
    if let Some(i) = three {
        match i {
            SpreadsheetCell::Int(value) => {
                println!("The value of the first item (i32) of row is: {}", value); // 3
            },
            SpreadsheetCell::Float(value) => {
                println!("The value of the first item (f64) of row is: {}", value);
            },
            SpreadsheetCell::Text(value) => {
                println!("The value of the first item (String) of row is: {}", value);
            },
        }
    }

    // HASH MAP
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get(&String::from("Blue"));
    if let Some(v) = score {
        println!("The value of the Blue item of scores is: {}", v); // 10
    }
    for (key, value) in &scores {
        println!("{}: {}", key, value); // Yellow: 50 Blue: 10
    }

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let score2 = scores2.get(&String::from("Blue"));
    if let Some(v) = score2 {
        println!("The value of the Blue item of scores2 is: {}", v); // 10
    }
    for (key, value) in &scores2 {
        println!("{}: {}", key, value); // Yellow: 50 Blue: 10
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
