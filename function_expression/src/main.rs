fn main() {
    println!("Expressions evaluate to a resultant value. Let’s look at some examples. ");

    //----------
    //let x = (let y = 6);  //When you run this program, the error you’ll get looks like this:
    //note: variable declaration using `let` is a statement

    let y = {
        let x = 3;
        x + 3
    };

    println!("The value of y is: {y}");


    //Function With Return Value 
    five();

    //function with argument or parameter or concrete and return value
    let z = plus_one(9);

    println!("The value of z is: {z}");


}

fn five() -> u32 {
    5
}

fn plus_one( x: i32) -> i32{
    x + 1
}