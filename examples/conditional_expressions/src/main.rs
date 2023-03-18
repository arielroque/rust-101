fn main() {

   let a = 2;
   let b = 10;

   // Basic Condition

   if a > b {
      println!("A is greater than B");
   }
   else{
      println!("B is greater than A");
   }

   // If let -> in this case use pattern matching to verify 

   let course = ("Rust","beginner","course");

   if let ("Rust","beginner","course") = course{
      println!("Match all fields");
   } 
   else{
      println!("Value unmatched");
   }


   if let ("Rust",_c,_d) = course {
       println!("Verify if the first value match")
   }
   else{
       println!("First value is not matched")
   }


   // Match expression
   // It is like switch case on Java

   let language = "Rust";

   match language {
      "Java" => {
         println!("Choose Java");
      },
      "Python" => {
        println!("Choose Python");
      },
      "Rust" => {
        println!("Choose Rust");
      }
      _ =>{
        println!("Choose unknow language");
      }
   }



    
}
