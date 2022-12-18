/*
Rust Datatypes
==============

Implicity variables definition

let v = value;

Explicity variables definition

let v:datatype = value;
*/

fn main() {
    //Integers

    /*
    # Fixed Integer Types
    i8: The 8-bit signed integer type

    i16: The 16-bit signed integer type

    i32: The 32-bit signed integer type

    i64: The 64-bit signed integer type

    u8: The 8-bit unsigned integer type

    u16: The 16-bit unsigned integer type

    u32: The 32-bit unsigned integer type

    u64: The 64-bit unsigned integer type


    # Variable Size Types
    # depends on the underlying machine architecture

    isize: The pointer-sized signed integer type

    usize: The pointer-sized unsigned integer type

    */

    //Explicity definition of integers
    let integer1:i8 = 1;
    let integer2:u16 = 1;

    println!("integer1: {}", integer1);
    println!("integer2: {}", integer2);

    //Floats

    /*
    ## Float Types
    f32: The 32-bit floating point type.
    f64: The 64-bit floating point type.
    */

    //Explicity definition of floats
    let float1:f32 = 1.8;
    let float2:f64 = 2.4567;

    println!("float1: {}", float1);
    println!("float2: {}", float2);


    //Booleans
    //Explicity definition of booleans
    let bool1: bool = true;
    
    //Implicity definition of booleans
    let bool2 = false;

    println!("bool1: {}", bool1);
    println!("bool2: {}", bool2);


    //Characters
    //Important: Characters enclosed within single quotes

    //Explicity definition of characters
    let my_char1 = 'a'; 

    // Implicity definition of a character
    let my_char2 = 'b';

    println!("my_char1: {}", my_char1);
    println!("my_char2: {}", my_char2);

    //Strings
    //Important: String enclosed within double quotes

     //Explicity definition of strings
     let my_string1 = "a"; 

     // Implicity definition of a strings
     let my_string2 = "b";
 
    println!("my_string1: {}", my_string1);
    println!("my_string2: {}", my_string2);


    // Arrays
    // Definition -> let name: [type;size] = [elements...]
    // Important: By default, arrays are immutable

    // Define a array with 4 elements
    let arr:[i32;4] = [1,2,3,4];

    // Initialize array of size 4 with 0
    // the 4 positions will be filled with 0
    // Rust understand that the type is integer
    let arr2 = [0;4];

    // Define a mutate array
    // the 4 positions will be filled with 1
    // Rust undenstand that the type is integer 
    let mut array_mutate = [1;4];

    //Access element
    println!("Position 1 from arr {}",arr[1]);
    println!("Position 0 from arr2 {}",arr2[0]);

    // Change value in mutate array
    array_mutate[1] = 0;
    println!("{:?}",array_mutate);

    // Get size
    println!("The size is {}", arr.len());

    // Slice array
    // At this point we need to use a data pointer to get the value to slice
    // Starting index is inclusive and ending is exclusive
    let slice_array:&[i32] = &arr[0..2];
    println!("Slice arrray is {:?}", slice_array);
















    


    

    

}
