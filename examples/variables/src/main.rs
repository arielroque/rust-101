fn main() {
    // Variables are by default imultable in Rust
    let first_variable = 1;

    println!("First Variable: {}", first_variable);

    // To change the values we need to use modifier mut
    let mut second_variable;

    println!("Second Variable: {}", second_variable);

    second_variable = 3;

    println!("Second Variable: {}", second_variable);

    //In variables created before we let Rust identify the type, but
    //we could specify this
    let third_variable: i32 = 2;

    println!("Third Variable: {}", third_variable);

    // Assigning multiple variables
    let (x, y) = (1, 2);

    println!("X:{} , Y:{}", x, y);

    // Shadowing variable are when the use the name of variable in different blocks
    let outer_variable = 112;

    // Start of code block
    {
        let outer_variable = 10;
        println!("Outer variable inside block: {}", outer_variable);
    }
    // end of code block

    println!("Outer variable outside block: {}", outer_variable);
}
