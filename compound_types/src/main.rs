use std::io;

fn main() {
    println!("
        Compound types can group multiple values into one type. 
        Rust has two primitive compound types: 
            tuples and arrays.
    ");

    loop{
        println!("Select Data Type :");
        println!("Option 1:  Introduction Tupal");
        println!("Option 2:  How to declare Tupal");
        println!("Option 3:  Destructuring Tupal");
        println!("Option 4:  Access Tupal Element");
        println!("Option 5:  Array"); 
        println!("Option 6:  Exit");

        let mut selected_dtype = String::new();
        io::stdin().read_line(&mut selected_dtype).expect("Failed to read line");

        let selected_dtype: u32 = match selected_dtype.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option.");
                return;
            }
        };

        
        match selected_dtype {
                1 => {
                    println!("
                        A tuple is a general way of grouping together a number of values with a variety of types into 
                        one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
                    ");

                
                    println!("
                        To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value:
                    ");

                    let _tup = (500, 6.4, 1);

                    let (_x, _y, _z) = _tup;

                    println!("The value of y is: {_y}");
                },
                2 => {
                    println!("
                            let tup: (i32, f64, u8) = (500, 6.4, 1);
                    ");

                    let _tup: (i32, f64, u8) = (500, 6.4, 1);

                },
                3 => {
                    println!("
                        let tup = (500, 6.4, 1);

                        let (x, y, z) = tup;
                    ");
                    let _tup = (500, 6.4, 1);

                    let (_x, _y, _z) = _tup;

                },
                4 => {
                    println!("
                        
                        let x: (i32, f64, u8) = (500, 6.4, 1);

                        let five_hundred = x.0;

                        let six_point_four = x.1;

                        let one = x.2;
                    ");

                    let _x: (i32, f64, u8) = (500, 6.4, 1);

                    let _five_hundred = _x.0;

                    let _six_point_four = _x.1;

                    let _one = _x.2;
                },
                5 => {
                    println!("
                        Another way to have a collection of multiple values is with an array. Unlike a tuple, 
                        every element of an array must have the same type. Unlike arrays in some other languages, 
                        arrays in Rust have a fixed length.
                    ");
                    let _a = [1, 2, 3, 4, 5];

                    //You write an array’s type using square brackets with the type of each element, a semicolon, and 
                    //then the number of elements in the array, like so:
                    let _a: [i32; 5] = [1, 2, 3, 4, 5];

                    //You can also initialize an array to contain the same value for each element by specifying the initial value, 
                    //followed by a semicolon, and then the length of the array in square brackets, as shown here:
                    let _a = [3; 5];



                    //An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. 
                    //You can access elements of an array using indexing, like this:

                    let _a = [1, 2, 3, 4, 5];

                    let _first = _a[0];
                    let _second = _a[1];


                },
                6 => {
                    println!("Exiting...");
                    return;
                },
                _ => {
                    println!("Invalid option.");
                }
            }

    }
}