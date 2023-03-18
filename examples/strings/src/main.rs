fn main() {
    // String literals (&str)
    // Properties:
    // - Primitive type
    // - Immutable
    // - Fixed-length string stored somewhere in memory 
    // - Value of string is known at compile time


    let language:&str = "Rust";
    println!("{}",language);
    println!("Size: {}",language.len());


    // String Object (String)
    // Properties
    // - A string is encoded as UTF-8 sequence
    // - The size of the string can be modified
    // - Non null-terminated
    // - Encoded string values that are ginven at the run time

    // Create a string object

    let language_string = String::new(); // empty string
    let language_string1 = language.to_string(); // convert literal string to object string
    let language_string2 = String::from("Rust"); //Initialize a string object with value
    
    // Methods

    println!("Size {}",language_string2);
    println!("Capacity {}",language_string2.capacity()); // check the capacity of string
    println!("Contains R {}",language_string2.contains("R"));
    
    let result = language_string2.replace("Rust","Java");
    println!("Replace Rust to Java: {}",result);

    let string = "   Rust     Programming     ".to_string();
    let trim_string = string.trim(); 
    println!("Trimmed_string : {}", trim_string);

    // Iterating over Strings using spaces
    let str = String::from("Rust Programming");

    for i in str.split_whitespace(){
        println!("{}",i);
    }

    // Interatig over Strings with custom split
    let str = String::from("Educative, course on, Rust, Programming");  
 
    for token in str.split(","){
        println!("{}", token);
    }

    // Interating over chars
    let str = String::from("Rust Programming");  
   
    for token in str.chars(){
        println!("{}", token);
    }

    // Push element into a string

    let mut course = String::from("Rus");
    course.push('t'); // add a char;

    println!("Adding a char: {}", course);

    course.push_str(" course"); // add a string
    println!("Adding a string: {}", course);

    // Concatanation using +

    let course = "Rust".to_string();
    let course_type = " beginner course".to_string();
    let result = course + &course_type; // to do this, we use borrowing

    println!("{}",result);

    // Format Macro

    let course = String::from("Rust");
    let course_type = "beginner course".to_string();

    let result = format!("{1} {0}",course,course_type);

    println!("{}",result);

    // Slice Strings

    let course = String::from("Rust");
    let course_sliced = &course[0..3];
    println!("{}",course_sliced);
}
