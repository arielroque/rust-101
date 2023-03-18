fn main() {
    // In Rust, we can pass parameters by value or reference

    fn function_by_value(mut value:i32){
        value +=1;
        println!("Inside function by value {}",value); 
        // return is not required, if is not set
        // the last line will be returned
    }

    fn function_by_reference(value:&mut i32){
        *value += 1;
        println!("Inside function by reference {}",value); 
    }

    fn function_with_multiple_parameters() -> (i32,i32){ // It is not required, but can specify the return
        let x =1;
        let y =2;
        return (x,y);
    }

    let mut x = 1;
    function_by_value(x);
    println!("Outside function by value {}",x);

    function_by_reference(&mut x);
    println!("Outside function by reference {}",x);

    let (a,b) = function_with_multiple_parameters();
    println!("Outside function with multiple parameters: a:{} b:{}",a,b);
}
