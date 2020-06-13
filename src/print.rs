pub fn run(){
    //Print to console
    /*
        To print a basic string to console
     */
    println!("Hello from the print file");

    //formatting
    /*
        to print a number or variable
     */
    println!("Number: {}", 1); //for printing a number;

    //printing strings ->  here
    println!("{} is from {}", "Toluwanimi","Nigeria");

    //positional arguments
    println!("{0} is from God and {0} wants {1}", "Toluwanimi","Temitayo");

    //named arguments{params}
    println!("{name} likes to {act}", name = "Toluwanimi", act = "code");
    //placeholder traits
    print!("Binary: {:b} Hex : {:x} Octal : {:o}", 10, 10, 10);

    //placeholder for debug trait [tuple]
    println!("{:?}", (12, true, "Hey"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}