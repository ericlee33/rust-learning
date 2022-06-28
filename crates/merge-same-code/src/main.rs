fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];

    for item in list {
        if *item > max {
            max = *item;
        }
    }
    max
}

fn main() {
    let arr1 = vec![22, 33, 111];
    let max = largest(&arr1);

    println!("{}", max);
}
