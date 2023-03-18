fn main() {
    // Create a empty vector
    // In this case the type is required
    // In addition, this vector is mutate
    let mut my_vector1:Vec<i8> = Vec::new();

    // Create with values
    let my_vector2 = vec![1,2,3,4];
    
    // Create with range values
    // Collect function transform in collection
    // In this case the type is required
    let my_vector3:Vec<i8> = (1 .. 5).collect();

    // Print initial values
    println!("{:?}",my_vector1);
    println!("{:?}",my_vector2);
    println!("{:?}",my_vector3);

    //Methods

    // Add values
    my_vector1.push(1);
    my_vector1.push(2);
    my_vector1.push(3);
    my_vector1.push(4);
    my_vector1.push(5);

    // Pop values
    my_vector1.pop(); //remove last values

    // Contains
    let res = my_vector1.contains(&2);

    // Remove by index
    my_vector1.remove(0);

    println!("{:?}",my_vector1);

    // Remove specific value
    // In this case, we will remove the number 2
    let index_value = my_vector1.iter().position(|&r| r == 2).unwrap();
    my_vector1.remove(index_value);

    // Len
    let len_my_vector1 = my_vector1.len();

    // Capacity 
    let capacity_my_vector1 = my_vector1.capacity();

    println!("Contains: {}",res);
    println!("Len: {}",len_my_vector1);
    println!("Capacity: {}",capacity_my_vector1);

    // Loop through the values
    for i in my_vector1.iter(){
        println!("{}",i);
    }

    // Slice vector
    let sliced_vector = &my_vector2[0..3];

    println!("{:?}",sliced_vector);








    
    
}
