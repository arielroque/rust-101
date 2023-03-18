fn main() {

    // For loop
    for i in 1 .. 3{
        println!("{}",i);
    }

    // While loop 

    let mut x = 0;

    while x < 3{
        println!("{}",x);
        x +=1;
    }

    // Infinite loop with break

    let mut x = 0;

    loop {

       if x == 3{
         break;
       } 

       println!("{}",x);

       x += 1;
    }

}
