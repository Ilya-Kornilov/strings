fn main() {
    let s1 = "Hello, мир! 🦁";
    let s2 = String::from("سلام دنیا");
    let s3 = "salve mundi".to_string();
    let s4 = "salve mundi".to_owned();
    let s5 = &s4[..];

    println!(" > 1: {s1}");
    println!(" > 2: {s2}");
    println!(" > 3: {s3}");
    println!(" > 4: {s4}");
    println!(" > 5: {s5}");

    let mut s = String::from("foo");
    
    s.push_str("bar");
    println!(" > s: {s}");

    s.replace_range(.., "baz");
    println!(" > new 's': {s}");
}
