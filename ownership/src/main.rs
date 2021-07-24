fn main() {
    let mut s1 = String::from("Nep");
    let s2 = &s1;
    println!("Hello, world! {}",s2);
    println!("Hello, world! {}",change(&mut s1));
    println!("the first word is on {}", first_word(&s1));
    //string slicing

    println!(" the first word slice is {}",first_word_slice(&s1));
    let mut s = String::from("hello world");
    let hello = &mut s[0..5];
    println!("{}", hello);

    let world = &s[6..11];
    println!("{}", world);

    //tuple
    let c = (12 , 34, 55);
    println!("{}, {} , {}" , c.0, c.1, c.2);

    struct color(i32, i32, i32);

}

 fn change(some_string: &mut String) -> &String {
     some_string.push_str(" ,reddit");
     some_string
 }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }

    }
    s.len()
}

fn first_word_slice(s:&String) -> &str {
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
