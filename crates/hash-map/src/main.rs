use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert("blue", 10);
    map.insert("yellow", 5);

    let score = map.get("blue");

    match score {
        Some(s) => println!("{}", s),
        None => println!("doesn't exist"),
    }

    map.entry("pink").or_insert(10);

    for (k, v) in &map {
        println!("{}, {}", k, v);
    }
}
