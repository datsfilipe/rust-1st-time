fn main() {
    vars();
}

fn vars() {
    // mutable variable
    let mut first_var: u32;

    // this is called scoping and is valid
    {
        // immutable variable
        let second_var = 1;
        println!("second_var: {}", second_var);
    }
    // this is not valid because second_var is out of scope
    // println!("second_var: {}", second_var);

    // valid because first_var is mutable
    first_var = 2;

    println!("first_var: {}", first_var);

    // it can't be assigned a negative value because it's unsigned (u32)
    // in orders for integers to be negative, they must be signed (i*)
    first_var = 3;

    // shadowing
    let first_var = first_var + 1;
    // the old value is still available, but can't be used anymore

    println!("first_var: {}", first_var);
}
