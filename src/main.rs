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

    println!("first_var: {}", first_var);
}
