use core::ops::Range;

fn demo_primitives() {
    let x: f32 = 5.0; // float
    let y: f64 = 100.3; // float
    let is_snowing: bool = true; // boolean
    let letter: char = 'a'; // char
    let numbers: &[Range<i32>] = &[1..5];
    let names: [&str; 2] = ["alain", "herve"]; // 2*string[ref] array
    let human: (&str, i32, bool) = ("alain", 25, false); // tupple
    let number_slices: &[i32] = &[0, 4, 8, 12, 16, 20]; // num slice
    let animal_slices: &[String] = &["dog".to_string(), "cat".to_string()];

    let a: String = "Hello world!".to_string();
    let sla: &str = &a[0..5];
    println!("++++ Demoing primitives +++++");

    println!("letter           : {}", letter);
    println!("signed integer   : {}", x);
    println!("unsigned integer : {}", y);
    println!("is_snowing       : {}", is_snowing);
    println!("range            : {:?}", numbers);
    println!("names            : {:?}", names);
    println!("human            : {:?}", human);
    println!("number_slices    : {:?}", number_slices);
    println!("animal_slices    : {:?}", animal_slices);
    println!("");
    println!("a                : {}", a);
    println!("sla              : {}", sla);
}
