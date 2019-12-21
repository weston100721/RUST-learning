use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let k = vec![1,2,3,4,5];

    v.push(34);
    v.push(35);
    v.push(36);
    v.push(37);
    v.push(37);

    let third: i32 = v[3];
    println!("The third element is {}", third);

    let seven: i32 = v[4];

    match v.get(4) {
        Some(seven) => println!("The third element is {}", seven),
        None => println!("There is no third element."),
    }


    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("score = {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);



    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
