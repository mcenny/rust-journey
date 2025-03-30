fn main() {
    // Variables
    println!("Hello, rust from CARGO!");

    // Integers
    let x: i32 = -42;
    let y: u32 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Floats
    let z: f32 = 3.14;
    let w: f32 = 16.21;
    let product: f32 = z * w;
    println!("Product: {}", product);

    // Arrays
    println!("********** Arrays **********");

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
    println!("Second number: {}", numbers[1]);
    println!("Third number: {}", numbers[2]);
    println!("Fourth number: {}", numbers[3]);
    println!("Fifth number: {}", numbers[4]);
    println!("Length of array: {}", numbers.len());
    println!("Sum of array: {}", numbers.iter().sum::<i32>());
    println!("Array: {:#?}", numbers);

    let human: (String, i32, bool) = ("John Doe".to_string(), 42, true);
    println!("Name: {:?}", human);

    let (name, age, is_alive) = human;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is alive: {}", is_alive);

    // Tuples
    println!("********** Tuples **********");

    let my_mix_tuple = ("Francis", 33, true, 3.14, "Rust", [1, 2, 3]);
    println!("Tuple: {:?}", my_mix_tuple);

    // Slices
    println!("********** Slices **********");

    let slice: [i32; 5] = [3, 1, 6, 4, 5];
    let slice_slice: &[i32] = &slice[1..4];
    let slice_slice2: &[i32] = &slice[1..];
    println!("Slice: {:?}", slice);
    println!("Slice slice: {:?}", slice_slice);
    println!("Slice slice2: {:?}", slice_slice2);

    // Strings
    println!("********** Strings **********");

    let my_string = String::from("Hello, world!");
    println!("String: {}", my_string);
    println!("Length of string: {}", my_string.len());
    println!(
        "First character of string: {}",
        my_string.chars().next().unwrap()
    );
    println!(
        "Last character of string: {}",
        my_string.chars().last().unwrap()
    );

    // Functions
    println!("********** Functions **********");

    let result = add(2, 3);
    println!("Result: {}", result);
}

// Functions
fn add(a: i32, b: i32) -> i32 {
    a + b
}
