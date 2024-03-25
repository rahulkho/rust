fn main() {
    println!("Statements are instructions that perform some action and do not return a value.");

    print_labeled_measurement(5, 'h');



    //----------
    let x = (let y = 6);  //When you run this program, the error you’ll get looks like this:
    //note: variable declaration using `let` is a statement

}

fn print_labeled_measurement( value: i32, unit_label: char){
    println!("The measurement is: {value} {unit_label}");
}