fn demo_mutref() {
    let x = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    let mut s1: String = "RUST".to_string();
    let len: usize = calculate_length(&s1);

    let r: &mut String = &mut s1;
    *r = String::from("hello");

    println!("Length is {}", len);
    println!("s1.len() is {}", len);
    // println!("value of s1: {}", s1); // wont work if un commented: s1 is  mutable. and you can have only one reference of a mutable var
    println!("value of r: {}", r); // hello
    println!("value of s1: {}", s1); // RUST -to-> hello
    println!("result is {}", x);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
