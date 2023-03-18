fn main() {
   //Ownership
   // In simple terms means to have possession of the variables. This
   // is a concept that helps ensure memory safety and prevent issues
   // like null pointer

   // Three Rules of Ownership
  
   // Rule1
   // Each values has a variable binding called its owner
   // When a owner goes out of scope, the values is automatically freed

   let a = 1; // has only 1 owner "a"

   // Rule2
   // There can only be onew owner at a time
   // This prevent multiple variables from access the same memory

   // B is the owner of the integer 4,
   // When we assign B to C, the ownership is transferred to C
   // This mean that B can not longe use B

   let b = 4; 
   let c = b;

   println!("{}",c);

   // Rule3
   // When the owner goes out of the scope, the value will be dropped

   {
    let d = String::from("oi");
   }

   // Here the d is not work

}
