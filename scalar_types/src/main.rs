use std::io;

fn main() {
    println!("Select Data Type :");
    println!("Option 1:  Integer");
    println!("Option 2:  Float");
    println!("Option 3:  Boolean");
    println!("Option 4:  Characters");
    println!("Option 5:  Numeric Operations");  
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
                let guess = ''42''.parse().expect(''Not a number!'');

                If we dont add the : u32 type annotation shown in the preceding code, Rust will display the error, 
                which means the compiler needs more information from us to know which type we want to use.

                let guess: u32 = ''42''.parse().expect(''Not a number!'');

                This type declaration indicates that the value its associated with should be an unsigned integer 
                (signed integer types start with i instead of u)that takes up 32 bits of space.
                Length	Signed	Unsigned
                8-bit	i8	    u8
                16-bit	i16	    u16
                32-bit	i32	    u32
                64-bit	i64	    u64
                128-bit	i128	u128
                arch	isize	usize

                Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. 
                So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. 
                Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, 
                which equals 0 to 255.

                Additionally, the isize and usize types depend on the architecture of the computer your program is running on, 
                which is denoted in the table as “arch”: 64 bits if you re on a 64-bit architecture and 32 bits if you re on a 
                32-bit architecture.

                You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be 
                multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use 
                _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as 
                if you had specified 1000.

                Number      literals Example
                Decimal	        98_222
                Hex	            0xff
                Octal	        0o77
                Binary	        0b1111_0000
                Byte(u8 only)	b'A'
            ");
        },
        2 => {
            println!("
            
                Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. 
                Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. 
                The default type is f64 because on modern CPUs.

                let x = 2.0; // f64

                let y: f32 = 3.0; // f32

            ");
        },
        3 => {
            println!("
                As in most other programming languages, a Boolean type in Rust has two possible values: true and false. 
                Booleans are one byte in size

                let t = true;   

                let f: bool = false; // with explicit type annotation
            ");
        },
        4 => {
            println!("

            let c = 'z';
            let z: char = 'ℤ'; // with explicit type annotation
            let heart_eyed_cat = '😻';

            Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. 
            Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a 
            lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width 
            spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to 
            U+10FFFF inclusive.

            ");
        },
        5 => {
            
            // addition
            let sum = 5 + 10;

            // subtraction
            let difference = 95.5 - 4.3;

            // multiplication
            let product = 4 * 30;

            // division
            let quotient = 56.7 / 32.2;
            let truncated = -5 / 3; // Results in -1

            // remainder
            let remainder = 43 % 5;
            return;
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