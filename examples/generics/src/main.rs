use std::fmt::Display;

// Create a generic function
fn concatenate<T:Display>(t:T,s:T) -> String{
    let result = format!("{} {}",t,s);
    return result;
}

// Create a generic struct
struct Rectangule<T>{
    width: T,
    height: T
}

impl<T:Display> Rectangule<T>{
    fn to_string(&self) -> String{
        let res = format!("Width: {}  Height: {}",self.width,self.height);
        return res;
    }
}

fn main() {
    let number1 = 1;
    let number2 = 2;
    
    let string1 = "oi";
    let string2 = "opaa";

    // Call concatenate with string
    let res1 = concatenate(number1,number2);
    println!("{}",res1);

    // Call concatenate with intege
    let res2 = concatenate(string1,string2);
    println!("{}",res2);

    // Initialize struct with integer
    let rectangule1 = Rectangule{width:1,height:2};

    // Initialize struct with float
    let rectangule2 = Rectangule{width:2.2,height:1.8};

    println!("{}",rectangule1.to_string());
    println!("{}",rectangule2.to_string());

}
