// challenge
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car {
        color,
        transmission,
        convertible,
        mileage: 0,
    };
    car
}

// the #[derive(Debug)] syntax lets us see certain values during the code execution that aren't otherwise viewable
// Define a classic struct
#[derive(Debug)]
struct MouseClick { _x: i64, _y: i64 }

#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(String, char) }

fn main() {
    let mut car = car_factory(String::from("red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
    car = car_factory(String::from("silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
    car = car_factory(String::from("green"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    vars();
    types();
    functions();
    compound_data_and_test_values();
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

    // tuples
    // elements can't be added or removed from a tuple.
    let tuple = (500, 6.4, 1);
    println!("The value of tuple is: {:#?}", tuple); // prints (500, 6.4, 1)
    // as you saw, in rust the simple {} can't be used to print a tuple, you have to use {:?} {:#?} instead
    // the difference between them is that {:#?} prints the tuple in a more readable way (with line breaks and indentation)
    // accessing tuple elements
    let second_value = tuple.1; // tuple.1 is a float (this works different than I thought (I'm not gonna explain here because I don't want to sound stupid))
    println!("The value of tuple.0 is: {}, and tuple.1 is: {}", tuple.0, second_value); // prints 500
}

fn types() {
    // booleans
    let is_bigger = 1 > 4; // type is bool
    println!("Is 1 > 4? {}", is_bigger);
    let is_bigger = is_bigger > false; // of course we can do this
    println!("Is is_bigger > false? {}", is_bigger);

    // chars
    // Some languages treat their char types as 8-bit unsigned integers,
    // which is the equivalent of the Rust u8 type. The char type in Rust contains unicode code points,
    // but they don't use UTF-8 encoding. A char in Rust is a 21-bit integer that's padded to be 32 bits wide
    // Interesting, Microsoft asked this question: Which statement correctly describes how Rust supports text character values?
    // and  the answer was: In Rust, all text types are valid UTF-8 representations.
    // But char type is a valid UTF-8 representation? they said that it doesn't use UTF-8 encoding
    // hmm, I think that I'll have to learn more about this (later :D)
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
    
    // data collections
    // structs
    // those are types composed of other types (benefits: naming each field, so it's clear what the values mean)
    // classic struct: fields with names and can be accessed by using dot notation (<struct>.<field>)
    // tuple struct: fields without names and can be accessed by using dot notation (<struct>.<index>)
    // unit-like struct: commonly used as markers (we'll see later it seems)
    // defining the structs
    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }
    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);
    // Unit struct
    struct _Unit;
    // instantiating the structs
    let student = Student { name: String::from("John"), level: 5, remote: false };
    let mark = Grades('A', 'A', 'A', 'A', 3.75);
    // accessing the structs
    println!("{} is in level {}, isRemote: {}", student.name, student.level, student.remote);
    println!("{} {} {} {} {} is a {} student", mark.0, mark.1, mark.2, mark.3, mark.4, student.level);
    // as you saw: String data that's stored inside another data structure, such as a struct or vector, must be converted
    // from a string literal reference (&str) to a String type. To do the conversion, we use the standard String::from(&str) or .to_string() method.
    
    // enums (algebraic data types)
    // enums are variants, and each variant can have different types and amounts of associated data
    // defining the enum
    enum _WebEvent {
        // An enum variant can be like a unit struct without fields or data types
        WELoad,
        // An enum variant can be like a tuple struct with data types but no named fields
        WEKeys(String, char),
        // An enum variant can be like a classic struct with named fields and their data types
        WEClick { x: i64, y: i64 }
    }
    // WELoad has no associated data type or data.
    // WEKeys has two fields with data types String and char.
    // WEMClick contains an anonymous struct with named fields x and y, and their data types (i64).
    // Any function that uses a variant of the WebEvent enum must accept all the variants in the enum.
    // Each variant in the enum isn't its own type.
    // defining an enum with structs
    // Define a classic struct
    struct _MouseClick { x: i64, y: i64 }
    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    enum _WebEvent2 { WELoad(bool), WEClick(MouseClick), WEKeys(String, char) }
    // instantiate an enum
    let load = WebEvent::WELoad(true); // simple enum variant
    let click = MouseClick { _x: 100, _y: 250 }; // instantiate a MouseClick struct and bind the coordinate values
    let we_click = WebEvent::WEClick(click); // enum variant with a struct
    let we_keys = WebEvent::WEKeys("Ctrl".to_string(), 'C'); // enum variant with a tuple struct
    // print values
    println!("\nWebEvent enum variants: \n{:?}\n{:?}\n{:?}\n", load, we_click, we_keys);
}

fn functions() {
    // parameters example
    let casual_greeting = "Hello";
    let formal_greeting = "Good evening";
    fn greet(name: &str, greeting: &str) {
        println!("{}, {}!", greeting, name);
    }
    greet("John", casual_greeting);
    greet("Mr. Smith", formal_greeting);
    // return values example
    fn add(x: i32, y: i32) -> i32 {
        if (x == 0) || (y == 0) {
            return 0; // return early
        }
        x * y // no semicolon at the end of the line, otherwise it would be a statement and not an expression
    }
    println!("The sum of 5 and 6 is: {}", add(0, 6));
}

fn compound_data_and_test_values () {
    // arrays
    // arrays are fixed-length data structures that contain elements of the same data type
    let numbers = [1, 2, 3, 4, 5]; // [i32; 5] is the type of the array -> [T; size]
    let zeros = [0; 5]; // this declaration makes the array with 5 elements of the value 0
    println!("The first number is: {}", numbers[0]);
    println!("Zeros array: {:?}", zeros);
}
