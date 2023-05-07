fn main() {
    vars();
}

fn vars() {
    // mutable variable
    let mut first_var: u32;
    // float variables. Obs: _ in the beginning of the variable name is to indicate that var is not being used
    let number_64 = 4.0;      // compiler infers the value to use the default type f64
    let _number_32: f32 = -5.0; // type f32 specified via annotation

    // valid because first_var is mutable
    first_var = 2;

    // all operations can be done with the number primitives types in rust
    println!("multiplying integer: {}, and floating: {}", number_64 * 4.0, first_var * 16u32);
    // 16u32 is an integer with the value of 16

    // this is called scoping and is valid
    {
        // immutable variable
        let second_var = 1;
        println!("second_var: {}", second_var);
    }
    // this is not valid because second_var is out of scope
    // println!("second_var: {}", second_var);

    println!("first_var: {}", first_var);

    // it can't be assigned a negative value because it's unsigned (u32)
    // in orders for integers to be negative, they must be signed (i*)
    first_var = 3;

    // shadowing
    let first_var = first_var + 1;
    // the old value is still available, but can't be used anymore
    println!("first_var (amongus) after mutation: {}", first_var); // prints 4

    // booleans
    let is_bigger = 1 > 4; // type is bool
    println!("Is 1 > 4? {}", is_bigger);
    let is_bigger = is_bigger > false; // of course we can do this
    println!("Is is_bigger > false? {}", is_bigger);

    // chars
    // Some languages treat their char types as 8-bit unsigned integers,
    // which is the equivalent of the Rust u8 type. The char type in Rust contains unicode code points,
    // but they don't use UTF-8 encoding. A char in Rust is a 21-bit integer that's padded to be 32 bits wide
    let smiley_face = '😃';
    let uppercase_s: u8 = 'S' as u8; // casting to u8 by using the as keyword

    // strings
    // The str type, also known as a string slice is a view into string data. Most of the time,
    // we refer to these types by using reference-style syntax that precedes the type with the ampersand &str.
    // We'll cover references in the following modules. For now, you can think of &str as a pointer to immutable string data.
    // String literals are all of type &str.
    // Seems that we won't get a full idea of the difference between String and &str until we learn about Rust's ownership and borrowing system.
    let string_1 = "miley ";
    let string_2: &str = "face";
    let string_3 = string_1.to_string() + string_2; // to_string coverts the given value to a String
    println!("{} is a {}{}", smiley_face, uppercase_s as char, string_3); // prints 😃 is a Smiley face (casting again otherwise it would print the number 83)
}
