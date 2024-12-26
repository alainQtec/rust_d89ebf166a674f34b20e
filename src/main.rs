fn main() {
    let age: u32 = 30;
    let is_adult: bool = age >= 18;
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    let statement: &str = if is_adult {
        "You are an adult."
    } else {
        "You are still young"
    };
    println!("statement : {}", statement);
    println!("result    : {}", result);
}
