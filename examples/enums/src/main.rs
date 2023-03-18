// Definition on enum
#[derive(Debug)]
#[derive(Clone)]
enum Semaphore{
    Red,
    Yellow,
    Green
}

// method in a enum

impl Semaphore{
    fn is_stop(&self) -> bool{
        match self{
            Semaphore::Red => return true,
            _ => return false
        }
    }
}

// Definition on enum with datatype
#[derive(Debug)]
enum Semaphore2{
    Red(String),
    Yellow(String),
    Green(String)
}

struct Player{
    name: String,
    color: Semaphore
}

fn result_semaphore(semaphore:Semaphore) -> Option<bool>{
    match semaphore{
        Semaphore::Red => return Some(true),
        _ => return None
    }
}

fn main() {
    // Access enum values
    let red = Semaphore::Red;
    let yellow = Semaphore::Yellow;
    let green = Semaphore::Green;

    // To print values in a enum, need use debug trait {:?}
    println!("{:?}",red);
    println!("{:?}",yellow);
    println!("{:?}",green);

    // Initialize enum values
    let red2 = Semaphore2::Red("Red".to_string());
    let yellow2 = Semaphore2::Yellow("Yellow".to_string());
    let green2 = Semaphore2::Green("Green".to_string());

    // Print enum
    println!("{:?}",red2);
    println!("{:?}",yellow2);
    println!("{:?}",green2);

    // Test method
    let action = Semaphore::Red;
    println!("{}",action.is_stop());

    // Initialize struct with enum

    let color = Semaphore::Red;
    let player1 = Player{name:"Jon".to_string(),color: color.to_owned()};

    println!("{:?}",player1.name);
    println!("{:?}",player1.color);

    // Using Options
    // Options is built enum that is used when:
    // - the return value is none
    // - the value of a variable is option
    // - out of bound exception is to be displayed

    let result = result_semaphore(color);

    if result.is_some(){
        println!("{}",result.unwrap());
    }






}


