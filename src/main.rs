fn main() {
    let first_var: i32;

    {
        let second_var = 1;
        println!("second_var: {}", second_var);
    }

    first_var = 2;

    println!("first_var: {}", first_var);

    first_var = 3;

    return;
}
