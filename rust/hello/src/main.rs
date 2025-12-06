
fn main() {
    println!("Hello, world!");

    // Formats: display {} and debug {:?}

    // Primitive data types:
    // int, float, bool, char

    println!("-- integers -------------------------");
    // Integers
    // i8, i16, i32, i64, i128 - signed
    // u8, u16, u32, u64, u128 - unsigned

    let x: u16 = 420;
    let y: i8  = -13;

    println!("x: {}", x);
    println!("y: {}", y);

    println!("-- floats ---------------------------");
    // Floats
    // f32, f64

    let pi: f32 = 3.13;

    println!("pi: {}", pi);

    println!("-- booleans -------------------------");
    // Booleans
    // false, true

    let night: bool = false;

    println!("night: {}", night);

    println!("-- chars ----------------------------");
    // Chars
    // char

    let letter: char = 'x';

    println!("letter: {}", letter);

    // Compound data types:
    // array, tuple, slice, string

    println!("-- arrays ---------------------------");
    // Arrays

    let numbers: [i8; 5] = [1, 2, 3, 4, 5];

    println!("numbers: {:?}", numbers);

    let cars: [&str; 3] = ["bmw", "tesla", "nissan"];

    println!("cars:    {:?}", cars);
    println!("cars[0]: {:?}", cars[0]);

    println!("-- tuples ---------------------------");
    // Tuples

    let user1: (String, u8, bool, [u8; 3]) = ("user1".to_string(), 100, false, [1, 2, 3]);
    let user2: (&str, u8, bool, [u8; 3])   = ("user2", 200, false, [4, 5, 6]);
    let users: ((String, u8, bool, [u8; 3]), (&str, u8, bool, [u8; 3])) = (user1.clone(), user2);

    println!("user1:    {:?}", user1);
    println!("user2:    {:?}", user2);
    println!("users:    {:?}", users);

    println!("-- slices ---------------------------");
    // Slices

    let numbers_dynamic: &[i8] = &[1, 2, 3, 4, 5];

    println!("numbers_dynamic: {:?}", numbers_dynamic);

    let cars_dynamic1: &[&str]    = &["bmw", "tesla", "nissan", "byd"];
    let cars_dynamic2: &[String]  = &["bmw".to_string(), "tesla".to_string(), "nissan".to_string(), "byd".to_string()];
    let cars_dynamic3: &[&String] = &[&"bmw".to_string(), &"tesla".to_string(), &"nissan".to_string(), &"byd".to_string()];

    println!("cars_dynamic1: {:?}", cars_dynamic1);
    println!("cars_dynamic2: {:?}", cars_dynamic2);
    println!("cars_dynamic3: {:?}", cars_dynamic3);

    println!("-- strings --------------------------");
    // Strings

    let mut book: String = String::from("Jung");

    println!("book:  {:?}", book);

    book.push_str(" - Red Book");

    println!("book:  {:?}", book);

    let title: &str = &book;

    println!("title: {:?}", title);
}

