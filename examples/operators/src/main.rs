fn main() {
    
    // Unary Operators
    // - Borrow -> &, &mut
    //   - Means access like a pointer the memory address of a variable
    //     Type:
    //       - Shared borrowing: A piece of data that is shared by single or multiple variables but it cannot be altered
    //       - Mutable borrowing: A piece of data that is shared and altered by a single variable (but the data is inaccessible to other variables at that time)
    
    // - Dereference -> *
    //    - Once you have a mutable reference to a variable, you can change the value

    // - Negation -> !

    // Binary Operators
    // -, +, -, *, /, % (Arithmetic)
    // &&, || (Logical)
    // >, <, <=, >=, ==, != (Comparison)
    // &, | , ^ (Bitwase)

    // Casting - as
    // - Integer can be type casted to floating-point and vice versa
    // - Integer can be typecasted to String
    // - String (&str) or character cannot be type casted to the data type of type integer or float
    // - Character cannot be type casted to String type and vice versa

    // Examples

    let a = 4;
    let b = 3;
        
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("Addition:{}", a + b);
    println!("Subtraction:{}", a - b);
    println!("Multiplication:{}", a * b);
    println!("Division:{}", a / b);
    println!("Modulus:{}", a % b);

    let a = true;
    let b = false;
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("AND:{}", a && b);
    println!("OR:{}", a || b);
    println!("NOT:{}", ! a);

    let a = 5;
    let b = 6;

    println!("Operand 1: {}, Operand 2: {}", a , b);
    println!("AND: {}", a & b);
    println!("OR: {}", a | b);
    println!("XOR: {}", a ^ b);
    println!("NOT a: {}", !a);
    println!("Left shift: {}", a << 2);
    println!("Right shift: {}", a >> 1);


    let a = 2;
    let b = (a as f64) / 2.0; 
    println!("a: {}", a);
    println!("b: {}", b);


    let x = 10;
    let mut y = 13;
    //immutable reference to a variable
    let a = &x;
    println!("Value of a:{}", a); 
    println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed
    //mutable reference to a variable
    let b = &mut y;
    println!("Value of b:{}", b);

    *b = 11; // derefencing 
    println!("Value of b:{}", b); // updated value of b
    println!("Value of y:{}", y); // y value can be changed as it is mutuably borrowed
}
