fn demo_fns() {
    say_hello();
    tell_height(180);
    get_human_id("Joel", 25, 180.0);

    println!("++++ Demoing functions +++++");
    println!("BMI is : {:.2}", calculate_bmi(90.0, 112.0987));
    println!("result is {}", add(4, 6)); // just a namdom number for demo
}

fn say_hello() {
    println!("Hello, world from rust!");
}

fn tell_height(height: u32) {
    println!("Height is {}", height);
}

fn get_human_id(name: &str, age: u32, height: f32) {
    println!("Name is {}, age is {}, height is {}cm.", name, age, height);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
