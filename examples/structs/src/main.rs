
// Struct 
struct Person{
    name: String,
    age: i8,
    address: String
}

// Create methods in the struct
impl Person{
    fn to_string(&self) -> String{
        return format!("{} {} {}",self.name,self.age,self.address);
    }

    // create a static method, in this case we do not need to instantiating the struct
    fn my_static_method(name: String, age: i8, address: String) -> Person{
        return Person{name:name,age:age,address:address};
    }
}

// Tuple Struct
// In this case, we can add many differents types of things 
// and use the struct style

struct FruitTuple(String,i8);

fn main() {

    // Initialize a Struct
    let mut person1 = Person{name:"Jon".to_string(),age:18,address:"Campina Grande".to_string()};

    // Initialize a tuple Struct
    let fruits =  FruitTuple("orange".to_string(),10);

    // Access values of struct 
    println!("{}",person1.name);
    println!("{}",person1.age);
    println!("{}",person1.address);

    // Access values from tuple
    println!("{}",fruits.0);
    println!("{}",fruits.1);
    
    // Update value
    person1.address = "Joao Pessoa".to_string();
    println!("{}",person1.address);

    // To String
    println!("{}",person1.to_string());

    // My static method 
    let person2 = Person::my_static_method("Jonny".to_string(),30,"Campina Grande".to_string());
    println!("{}",person2.to_string());



    
}
