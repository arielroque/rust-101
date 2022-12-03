fn main() {
    
    //print element without new line in the end
    print!("Examples:");

    //print element with new line add in the end
    println!("Hello, world!");

    //print with placeholder string 
    println!("Hello {}","user");

    //print with placeholder string 
    println!("Hello {}, you have {} cars in the game","user",2);

    //print element with math operactions
    println!("The result of {} + {} = {}",2.2,3,5.2);

    //print element in binary and octal way 
    println!("Bin: {:b}  Octal: {:o}",2,8);

    //print non defined number of element using placeholder, this is used to debug for example
    println!("Debug test: {:?}",("fdfdf",2,3,"333"));
    
    // print error
    eprintln!("This is a error {}",404);
    
    // prints error
    eprint!("This is a error");
    






}
