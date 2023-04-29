fn main() {
   //Ownership
   // In simple terms means to have possession of the variables. This
   // is a concept that helps ensure memory safety and prevent issues
   // like null pointer

   // Three Rules of Ownership
  
   // Rule1
   // Each values has a variable binding called its owner
   // When a owner goes out of scope, the values is automatically freed

   let _a = 1; // has only 1 owner "a"

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
    let _d = String::from("oi");
   }

   // Here the d is not work


   // To try mitigate the rule 2 (move semantic), we can use
   // cloning or reference. Reference is more cheaper tha clone


   // cloning
    
   let my_string = "hellooo".to_string();
   let cloned_string = my_string.clone();

   println!("The original string: {}",my_string);
   println!("The cloned string: {}",cloned_string);

   
   // reference

   let my_string2 = "helloo".to_string();
   let reference = &my_string2;

   println!("original: {} and reference: {}",my_string2,reference);


   // Lifetime

   // In the following function is need to pass lifetime to the variables
   // if we running without the a lifetime variable rust doest not know
   // until the moment that need to keep product 1 and product 2, so we
   // tell to keep until the end of function (a variable)

   #[derive(Clone)]
   #[derive(Debug)]
   struct Product {
      name: String,
      cost: f64
   }

   fn price(product: Product) -> f64 {
      product.cost * 1.30
   }

   fn discount(product: Product) -> f64 {
      price(product) / 1.10
   }


   fn get_high_cost<'a>(product1: &'a Product, product2: &'a Product) -> &'a Product {
      if product1.cost > product2.cost {
          product1
      } else {
          product2
      }
  }

  let product1 = Product{name: "shoes".to_string(), cost: 100.0};
  let product2 = Product{name: "hat".to_string(), cost: 90.0};
 
  println!("The product with the high cost is: {:#?}", get_high_cost(&product1, &product2));

}
