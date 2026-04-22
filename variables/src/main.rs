fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value after inner scope is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS} seconds.");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces.")

    // This is illegal due to different types.
    // let mut spaces = "   ";
    // spaces = spaces.len()

}
 