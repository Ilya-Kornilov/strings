
fn main() {
    let s1 = "Hello, мир! 🦁";
    let s2 = String::from("سلام دنیا");
    let s3 = "salve mundi".to_string();
    let s4 = "salve mundi".to_owned();
    let _s5 = &s4[..];

    println!(" > 1: {s1}");
    println!(" > 2: {s2}");
    println!(" > 3: {s3}");
    println!(" > 4: {s4}");
    println!(" > 5: {_s5}");

    let mut s = String::from("foo");
    
    s.push_str("bar");
    println!(" > s: {s}");

    s.replace_range(.., "baz");
    println!(" > new 's': {s}");

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // String must be first
    /*
     * println!(" > 5: {s5}");  
     * canNOT be done as s5 is borrowed
     * &s5 + &s6 won't work
     * s5 + s6 won't work as well
     * &s5 + s6 won't work either    
    */
    println!(" > 6: {s6}");
    println!(" > 7: {s7}");

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10= String::from("toe");
    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!(" > 8: {s8}");
    println!(" > 9: {s9}");
    println!(" > 10: {s10}");
    println!(" > 11: {s11}");

    println!(" > ramdom: {}", "random");

    let s12 = ["first ", "second"].concat();
    let s13 = format!("{}{}", "first ", "second");
    let s14 = concat!("first ", "second");
    let s15 = String::from("test_");
    let s16 = s15 + "ok"; // String must be first

    println!(" > 12: {s12}");
    println!(" > 13: {s13}");
    println!(" > 14: {s14}");
    // println!(" > 15: {s15}");   
    // canNOT be done as s15 is borrowed by s16
    println!(" > 16: {s16}");

    // Indexing Strings
    let s17 = "🦁🦁🦁🦁🦁";
    let s18 = &s17[..4];
        // as an emoji is 4 bytes
    println!(" > 17: {s17}");
    println!(" > 18: {s18}");

    let mut count = 1;
    for character in s17.bytes() {
        println!("\t> {count}: {character}");
        count +=1;
    }

    count = 1;
    for character in s17.chars() {
        println!("   >> {count}: {character}");
        count +=1;
    }

    let thank_you = "شكرًا لك";
    count = 1;
    for character in thank_you.chars() {
        println!("   >> {count}: {character}");
        count +=1;
    }
    /*
    count = 1;
    for character in thank_you.graphemes(true) {
        println!("   >> {count}: {character}");
        count +=1;
    }
    use unicode_segmentation::UnicodeSegmentation;
        to print individual singns of this string 
        in an appropriate way
    */ 

    let s19 = "Hello, World!";
    let s20 = String::from("Hello, World!");

    println!("{}", my_function(s19));
    println!("{}", my_function(&s20));
    
}

fn my_function(a: &str) -> String {
    return format!(" > {a}")
}
